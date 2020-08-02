use crate::point::Point;
use image::Rgb;
use std::mem;

const MAX_TRACE_STEPS: usize = 1000;
const MIN_DIST: f64 = 0.001;
const MAX_DIST: f64 = 1000.0;

pub trait Fractal {
    const CAMERA_POS: Point;
    fn estimate_distance(p: Point) -> f64;

    fn render(p: Point, screen_dim: Point, color_type: ColorType) -> Rgb<u8> {
        let uv = Point::new(
            (p.x - 0.5 * screen_dim.x) / screen_dim.y,
            (p.y - 0.5 * screen_dim.y) / screen_dim.y,
            0.0,
        );
        let rd = Self::get_camera_ray_dir(uv, Self::CAMERA_POS, Point::new(0.0, 0.0, 0.0), 1.);
        let d = Self::cast_ray(Self::CAMERA_POS, rd);

        Self::get_color(color_type, Self::CAMERA_POS.add(rd.mul_scalar(d)))
    }

    fn get_camera_ray_dir(uv: Point, p: Point, l: Point, z: f64) -> Point {
        let f = l.sub(p).normalize();
        let r = Point::new(0.0, 1.0, 0.0).cross(f).normalize();
        let u = f.cross(r).normalize();
        let c = p.add(f.mul_scalar(z));
        let i = c.add(r.mul_scalar(uv.x)).add(u.mul_scalar(uv.y));
        i.sub(p).normalize()
    }

    fn cast_ray(ro: Point, rd: Point) -> f64 {
        let mut t = 0.0;
        for _ in 0..MAX_TRACE_STEPS {
            let p = ro.add(rd.mul_scalar(t));
            let res = Self::estimate_distance(p);
            t += res;
            if res < MIN_DIST || t > MAX_DIST {
                break;
            }
        }
        t
    }

    fn get_color(color_type: ColorType, p: Point) -> Rgb<u8> {
        match color_type {
            ColorType::BlackAndWhite => {
                let mut light_val = 0.0;
                if p.length() < MAX_DIST {
                    light_val = Self::get_light(p);
                }
                light_val = light_val.powf(0.4545);

                let light_val = (light_val * 256.0) as u8;
                Rgb([light_val, light_val, light_val])
            }
            ColorType::Colored => {
                let color_value = (p.length() / MAX_DIST * 255.0f64.powf(3.)) as usize;
                let r = ((color_value >> 16) & 255) as u8;
                let g = ((color_value >> 8) & 255) as u8;
                let b = (color_value & 255) as u8;
                Rgb([r, g, b])
            }
            ColorType::ColoredWithShades => {
                let color_value = (p.length() / MAX_DIST * 255.0f64.powf(3.)) as usize;
                let r = ((color_value >> 16) & 255) as f64;
                let g = ((color_value >> 8) & 255) as f64;
                let b = (color_value & 255) as f64;
                let mut light_val = 0.0;
                if p.length() < MAX_DIST {
                    light_val = Self::get_light(p);
                    light_val = light_val.powf(0.4545);
                    light_val *= 256.0;
                }

                Rgb([
                    (r * light_val) as u8,
                    (g * light_val) as u8,
                    (b * light_val) as u8,
                ])
            }
        }
    }

    fn get_light(p: Point) -> f64 {
        let light_pos = Self::CAMERA_POS.add(Point::new(2.0, 2.0, 0.0));
        let l = light_pos.sub(p).normalize();
        let n = Self::get_normal(p);
        (n.dot(l) * 0.5 + 0.5).max(0.).min(1.)
    }

    fn get_normal(p: Point) -> Point {
        let d = Self::estimate_distance(p);
        let ex = 0.001;
        let ey = 0.01;
        Point::new(
            d - Self::estimate_distance(Point::new(p.x - ex, p.y - ey, p.z - ey)),
            d - Self::estimate_distance(Point::new(p.x - ey, p.y - ex, p.z - ey)),
            d - Self::estimate_distance(Point::new(p.x - ey, p.y - ey, p.z - ex)),
        )
        .normalize()
    }
}

pub struct Spheres;

impl Fractal for Spheres {
    const CAMERA_POS: Point = Point::new(3.0, 4.0, -4.0);

    fn estimate_distance(p: Point) -> f64 {
        p.r#mod(1.).add_scalar(-0.5).length() - 0.15
    }
}

pub struct Triangles;

impl Fractal for Triangles {
    const CAMERA_POS: Point = Point::new(-2.0, -2.0, -3.0);

    fn estimate_distance(mut z: Point) -> f64 {
        let mut n = 0;
        let iterations = 10;
        let offset = Point::new(1., 1., 1.);
        let scale = 2.0;
        while n < iterations {
            if z.x + z.y < 0. {
                // fold 1
                invert_and_swap(&mut z.x, &mut z.y);
            }
            if z.x + z.z < 0. {
                // fold 2
                invert_and_swap(&mut z.x, &mut z.z);
            }
            if z.y + z.z < 0. {
                // fold 3
                invert_and_swap(&mut z.z, &mut z.y);
            }
            z = z.mul_scalar(scale).sub(offset.mul_scalar(scale - 1.0));
            n += 1;
        }
        z.length() * scale.powf(-n as f64)
    }
}

pub struct Squares;

impl Fractal for Squares {
    const CAMERA_POS: Point = Point::new(-2.0, -2.0, -3.0);

    fn estimate_distance(mut z: Point) -> f64 {
        let mut r = z.dot(z);
        let mut i = 0;
        let iterations = 10;
        let cx = 1.;
        let cy = 1.;
        let cz = 1.;
        let scale = 3.;
        let bailout = 1000.;
        while i < iterations && r < bailout {
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

            z.z -= 0.5 * cz * (scale - 1.) / scale;
            z.z = -z.z.abs();
            z.z += 0.5 * cz * (scale - 1.) / scale;

            z.x = scale * z.x - cx * (scale - 1.);
            z.y = scale * z.y - cy * (scale - 1.);
            z.z *= scale;

            r = z.dot(z);
            i += 1;
        }
        r.sqrt() * scale.powf(-i as f64)
    }
}

fn invert_and_swap(x: &mut f64, y: &mut f64) {
    *x *= -1.;
    *y *= -1.;
    mem::swap(x, y);
}

#[derive(Copy, Clone, Debug)]
pub enum ColorType {
    BlackAndWhite,
    Colored,
    ColoredWithShades,
}
