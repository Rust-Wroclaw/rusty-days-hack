use crate::point::Point;
use image::Rgb;
use std::mem;

pub trait Fractal: Sync + Send {
    const MAX_TRACE_STEPS: usize = 1000;
    const MIN_DIST: f64 = 0.001;
    const MAX_DIST: f64 = 1000.0;
    const CAMERA_POS: Point;
    const CAMERA_DEST: Point = Point::ZERO;
    fn estimate_distance(&self, p: Point) -> f64;

    fn render(&self, p: Point, screen_dim: Point, color_type: ColorType) -> Rgb<u8> {
        let uv = Point::new(
            (p.x - 0.5 * screen_dim.x) / screen_dim.y,
            (p.y - 0.5 * screen_dim.y) / screen_dim.y,
            0.0,
        );
        let rd = Self::get_camera_ray_dir(uv, Self::CAMERA_POS, Self::CAMERA_DEST, 1.);
        let d = self.cast_ray(Self::CAMERA_POS, rd);

        if d < Self::MAX_DIST {
            None
        } else {
            color_type.background_color(uv)
        }
        .unwrap_or_else(|| self.get_color(color_type, Self::CAMERA_POS + rd * d))
    }

    fn get_camera_ray_dir(uv: Point, p: Point, l: Point, z: f64) -> Point {
        let f = (l - p).normalize();
        let r = Point::new(0.0, 1.0, 0.0).cross(f).normalize();
        let u = f.cross(r).normalize();
        let c = p + f * z;
        let i = c + r * uv.x + u * uv.y;
        (i - p).normalize()
    }

    fn cast_ray(&self, ro: Point, rd: Point) -> f64 {
        let mut t = 0.0;
        for _ in 0..Self::MAX_TRACE_STEPS {
            let p = ro + rd * t;
            let res = self.estimate_distance(p);
            t += res;
            if res < Self::MIN_DIST || t > Self::MAX_DIST {
                break;
            }
        }
        t
    }

    fn get_color(&self, color_type: ColorType, p: Point) -> Rgb<u8> {
        match color_type {
            ColorType::Grayscale => {
                let mut light_val = 0.0;
                if p.length() < Self::MAX_DIST {
                    light_val = self.get_light(p);
                }
                light_val = light_val.powf(0.4545);
                Rgb([(light_val * 256.0) as u8; 3])
            }
            ColorType::Distance => {
                let color_value = (p.length() / Self::MAX_DIST * 255.0f64.powf(3.)) as usize;
                let r = ((color_value >> 16) & 255) as u8;
                let g = ((color_value >> 8) & 255) as u8;
                let b = (color_value & 255) as u8;
                Rgb([r, g, b])
            }
            ColorType::Normal => {
                let half_point = Point::new(0.5, 0.5, 0.5);
                let color = self.get_normal(p) * half_point + half_point;
                let r = color.x * 255.;
                let g = color.y * 255.;
                let b = color.z * 255.;
                let mut light_val = 0.0;
                if p.length() < Self::MAX_DIST {
                    light_val = self.get_light(p);
                    light_val = light_val.powf(0.4545);
                }

                Rgb([
                    (r * light_val) as u8,
                    (g * light_val) as u8,
                    (b * light_val) as u8,
                ])
            }
        }
    }

    fn get_light(&self, p: Point) -> f64 {
        let light_pos = Self::CAMERA_POS + Point::new(2.0, 2.0, 0.0);
        let l = (light_pos - p).normalize();
        let n = self.get_normal(p);
        (n.dot(l) * 0.5 + 0.5).max(0.).min(1.)
    }

    fn get_normal(&self, p: Point) -> Point {
        let d = self.estimate_distance(p);
        let ex = 0.001;
        let ey = 0.0;
        Point::new(
            d - self.estimate_distance(Point::new(p.x - ex, p.y - ey, p.z - ey)),
            d - self.estimate_distance(Point::new(p.x - ey, p.y - ex, p.z - ey)),
            d - self.estimate_distance(Point::new(p.x - ey, p.y - ey, p.z - ex)),
        )
        .normalize()
    }
}

pub struct Spheres {
    radius: f64,
}

impl Spheres {
    pub fn new(radius: f64) -> Self {
        Spheres { radius }
    }
}

impl Default for Spheres {
    fn default() -> Self {
        Spheres::new(0.15)
    }
}

impl Fractal for Spheres {
    const CAMERA_POS: Point = Point::new(3.2, 4.0, -3.85);

    fn estimate_distance(&self, p: Point) -> f64 {
        ((p % 1.) - 0.5).length() - self.radius
    }
}

pub struct Tetrahedron {
    transf_mat1: [[f64; 3]; 3],
    transf_mat2: [[f64; 3]; 3],
}

impl Tetrahedron {
    pub fn new(rotate1: Point, rotate2: Point) -> Self {
        Tetrahedron {
            transf_mat1: rotate1.create_transformation(),
            transf_mat2: rotate2.create_transformation(),
        }
    }
}

impl Default for Tetrahedron {
    fn default() -> Self {
        Tetrahedron::new(Point::ZERO, Point::ZERO)
    }
}

impl Fractal for Tetrahedron {
    const MIN_DIST: f64 = 0.002;
    const CAMERA_POS: Point = Point::new(1.6, -0.8, -2.1);
    const CAMERA_DEST: Point = Point::new(0.0, -0.15, 0.0);

    fn estimate_distance(&self, mut z: Point) -> f64 {
        let mut n = 0;
        let iterations = 10;
        let offset = 1.0;
        let scale = 2.0;
        while n < iterations {
            // To modify the shape, rotation can be added
            z = z.apply_transformation(self.transf_mat1);

            if z.x + z.y < 0. {
                invert_and_swap(&mut z.x, &mut z.y);
            }
            if z.x + z.z < 0. {
                invert_and_swap(&mut z.x, &mut z.z);
            }
            if z.y + z.z < 0. {
                invert_and_swap(&mut z.z, &mut z.y);
            }

            z = z.apply_transformation(self.transf_mat2);

            z = z * scale - offset * (scale - 1.0);
            n += 1;
        }
        z.length() * scale.powf(-n as f64)
    }
}

pub struct Cube {
    transf_mat1: [[f64; 3]; 3],
    transf_mat2: [[f64; 3]; 3],
    c: Point,
    scale: f64,
}

impl Cube {
    pub fn new(rotate1: Point, rotate2: Point, c: Point, scale: f64) -> Self {
        Cube {
            transf_mat1: rotate1.create_transformation(),
            transf_mat2: rotate2.create_transformation(),
            c,
            scale,
        }
    }
}

impl Default for Cube {
    fn default() -> Self {
        Cube::new(Point::ZERO, Point::ZERO, Point::ONE, 3.0)
    }
}

impl Fractal for Cube {
    const CAMERA_POS: Point = Point::new(-1.8, -1.8, -2.7);
    const CAMERA_DEST: Point = Point::new(0.0, 0.3, 0.0);

    fn estimate_distance(&self, mut z: Point) -> f64 {
        let mut r = z.dot(z);
        let mut i = 0;
        let iterations = 10;
        let bailout = 100000.;
        while i < iterations && r < bailout {
            // To modify the shape, rotation can be added
            z = z.apply_transformation(self.transf_mat1);

            z = z.abs();
            if z.x - z.y < 0. {
                mem::swap(&mut z.x, &mut z.y);
            }
            if z.x - z.z < 0. {
                mem::swap(&mut z.x, &mut z.z);
            }
            if z.y - z.z < 0. {
                mem::swap(&mut z.z, &mut z.y);
            }

            z.z -= 0.5 * self.c.z * (self.scale - 1.) / self.scale;
            z.z = -z.z.abs();
            z.z += 0.5 * self.c.z * (self.scale - 1.) / self.scale;

            // To get even more unusual shapes, add more rotation
            z = z.apply_transformation(self.transf_mat2);

            z.x = self.scale * z.x - self.c.x * (self.scale - 1.);
            z.y = self.scale * z.y - self.c.y * (self.scale - 1.);
            z.z *= self.scale;

            r = z.dot(z);
            i += 1;
        }
        r.sqrt() * self.scale.powf(-i as f64)
    }
}

fn invert_and_swap(x: &mut f64, y: &mut f64) {
    *x *= -1.;
    *y *= -1.;
    mem::swap(x, y);
}

#[derive(Copy, Clone, Debug)]
pub enum ColorType {
    Grayscale,
    Distance,
    Normal,
}

impl ColorType {
    fn background_color(&self, uv: Point) -> Option<Rgb<u8>> {
        match self {
            ColorType::Grayscale => Some(100.0),
            ColorType::Distance => None,
            ColorType::Normal => Some(200.0),
        }
        .map(|v| Rgb([(v - uv.distance(Point::ZERO) * 50.0) as u8; 3]))
    }
}
