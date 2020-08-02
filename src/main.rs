use image::{ImageBuffer, Rgb, RgbImage};
use std::time::Instant;

const MAX_TRACE_STEPS: usize = 200;
const MIN_DIST: f64 = 0.001;
const MAX_DIST: f64 = 1000.0;

fn main() {
    for fractal_type in &[
        FractalType::Spheres,
        FractalType::Triangles,
        FractalType::TrianglesWithFold,
    ] {
        for color_type in &[
            ColorType::BlackAndWhite,
            ColorType::Colored,
            ColorType::ColoredWithShades,
        ] {
            let instant = Instant::now();
            let img_buffer = img(800, 800, *fractal_type, *color_type);
            img_buffer
                .save(&format!("fractal-{:?}-{:?}.png", fractal_type, color_type))
                .unwrap();
            println!(
                "Completed image for {:?} - {:?} in {:?} ms",
                fractal_type,
                color_type,
                instant.elapsed().as_millis()
            );
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum ColorType {
    BlackAndWhite,
    Colored,
    ColoredWithShades,
}

#[derive(Copy, Clone, Debug)]
enum FractalType {
    Spheres,
    Triangles,
    TrianglesWithFold,
}

impl FractalType {
    fn camera_pos(&self) -> Point {
        match self {
            FractalType::Spheres => Point::new(3.0, 4.0, -4.0),
            FractalType::Triangles | FractalType::TrianglesWithFold => Point::new(-2.0, -2.0, -5.0),
        }
    }

    fn light_pos(&self) -> Point {
        self.camera_pos().add(Point::new(2.0, 2.0, 0.0))
    }
}

fn img(
    width: u32,
    height: u32,
    fractal_type: FractalType,
    color_type: ColorType,
) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let mut img: RgbImage = ImageBuffer::new(width, height);
    let screen_dim = Point::new(width as f64, height as f64, 0.0);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        *pixel = render(
            Point::new(x as f64, y as f64, 0.0),
            screen_dim,
            fractal_type,
            color_type,
        );
    }

    img
}

fn render(
    p: Point,
    screen_dim: Point,
    fractal_type: FractalType,
    color_type: ColorType,
) -> Rgb<u8> {
    let uv = Point::new(
        (p.x - 0.5 * screen_dim.x) / screen_dim.y,
        (p.y - 0.5 * screen_dim.y) / screen_dim.y,
        0.0,
    );

    let rd = get_camera_ray_dir(uv, fractal_type.camera_pos(), Point::new(0.0, 0.0, 0.0), 1.);
    let color = cast_ray(fractal_type.camera_pos(), rd, fractal_type, color_type);

    color
}

fn get_camera_ray_dir(uv: Point, p: Point, l: Point, z: f64) -> Point {
    let f = l.sub(p).normalize();
    let r = Point::new(0.0, 1.0, 0.0).cross(f).normalize();
    let u = f.cross(r);
    let c = p.add(f.mul_scalar(z));
    let i = c.add(r.mul_scalar(uv.x)).add(u.mul_scalar(uv.y));
    i.sub(p).normalize()
}

fn cast_ray(ro: Point, rd: Point, fractal_type: FractalType, color_type: ColorType) -> Rgb<u8> {
    let mut dist = 0.0;
    let mut i = 0;
    while i < MAX_TRACE_STEPS {
        let p = ro.add(rd.mul_scalar(dist));
        let res = match fractal_type {
            FractalType::Spheres => estimate_distance(fractal_type, p),
            FractalType::Triangles => estimate_distance(fractal_type, p),
            FractalType::TrianglesWithFold => estimate_distance(fractal_type, p),
        };
        dist += res;
        if res < MIN_DIST || dist > MAX_DIST {
            break;
        }
        i += 1;
    }

    get_color(
        fractal_type,
        color_type,
        fractal_type.camera_pos().add(rd.mul_scalar(dist)),
    )
}

fn get_color(fractal_type: FractalType, color_type: ColorType, p: Point) -> Rgb<u8> {
    match color_type {
        ColorType::BlackAndWhite => {
            let mut light_val = 0.0;
            if p.length() < MAX_DIST {
                light_val = get_light(fractal_type, p);
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
                light_val = get_light(fractal_type, p);
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

fn get_light(fractal_type: FractalType, p: Point) -> f64 {
    let l = fractal_type.light_pos().sub(p).normalize();
    let n = get_normal(fractal_type, p);

    (n.dot(l) * 0.5 + 0.5).max(0.).min(1.)
}

fn get_normal(fractal_type: FractalType, p: Point) -> Point {
    let d = estimate_distance(fractal_type, p);
    let ex = 0.001;
    let ey = 0.01;

    Point::new(
        d - estimate_distance(fractal_type, Point::new(p.x - ex, p.y - ey, p.z - ey)),
        d - estimate_distance(fractal_type, Point::new(p.x - ey, p.y - ex, p.z - ey)),
        d - estimate_distance(fractal_type, Point::new(p.x - ey, p.y - ey, p.z - ex)),
    )
    .normalize()
}

fn estimate_distance(fractal_type: FractalType, p: Point) -> f64 {
    match fractal_type {
        FractalType::Spheres => estimate_distance_spheres(p),
        FractalType::Triangles => estimate_distance_tri(p),
        FractalType::TrianglesWithFold => estimate_distance_tri_with_folding(p),
    }
}

fn estimate_distance_spheres(p: Point) -> f64 {
    p.r#mod(1.).add_scalar(-0.5).length() - 0.15
}

fn estimate_distance_tri(mut z: Point) -> f64 {
    let a1 = Point::new(1., 1., 1.);
    let a2 = Point::new(-1., -1., 1.);
    let a3 = Point::new(1., -1., -1.);
    let a4 = Point::new(-1., 1., -1.);
    let mut c;
    let mut n = 0;
    let mut dist;
    let mut d;
    let scale = 2.;
    let iterations = 15;
    while n < iterations {
        c = a1;
        dist = z.sub(a1).length();
        d = z.sub(a2).length();
        if d < dist {
            c = a2;
            dist = d;
        }
        d = z.sub(a3).length();
        if d < dist {
            c = a3;
            dist = d;
        }
        d = z.sub(a4).length();
        if d < dist {
            c = a4;
        }
        z = z.mul_scalar(scale).sub(c.mul_scalar(scale - 1.0));
        n += 1;
    }

    z.length() * scale.powf(-n as f64)
}

fn estimate_distance_tri_with_folding(mut z: Point) -> f64 {
    let mut n = 0;
    let iterations = 10;
    let offset = Point::new(1., 1., 1.);
    let scale = 2.0;
    while n < iterations {
        if z.x + z.y < 0. {
            z.x = z.x.abs();
            z.y = z.y.abs();
        } // fold 1
        if z.x + z.z < 0. {
            z.x = z.x.abs();
            z.z = z.z.abs();
        } // fold 2
        if z.y + z.z < 0. {
            z.z = z.z.abs();
            z.y = z.y.abs();
        } // fold 3
        z = z.mul_scalar(scale).sub(offset.mul_scalar(scale - 1.0));
        n += 1;
    }
    z.length() * scale.powf(-n as f64)
}

#[derive(Copy, Clone, Debug)]
struct Point {
    x: f64,
    y: f64,
    z: f64,
}

impl Point {
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn mul_scalar(&self, value: f64) -> Point {
        Point::new(self.x * value, self.y * value, self.z * value)
    }

    pub fn add(&self, other: Point) -> Point {
        Point::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }

    pub fn add_scalar(&self, value: f64) -> Point {
        Point::new(self.x + value, self.y + value, self.z + value)
    }

    pub fn sub(&self, other: Point) -> Point {
        Point::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }

    pub fn length(&self) -> f64 {
        1.0 / self.dot(*self).powf(-0.5)
    }

    pub fn dot(&self, other: Point) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn normalize(&self) -> Point {
        self.mul_scalar(self.dot(*self).powf(-0.5))
    }

    pub fn cross(&self, other: Point) -> Point {
        Point::new(self.y * other.z, self.z * other.x, self.x * other.y).sub(Point::new(
            self.z * other.y,
            self.x * other.z,
            self.y * other.x,
        ))
    }

    pub fn r#mod(&self, n: f64) -> Point {
        Point::new(r#mod(self.x, n), r#mod(self.y, n), r#mod(self.z, n))
    }
}

fn r#mod(a: f64, b: f64) -> f64 {
    a - b * (a / b).floor()
}
