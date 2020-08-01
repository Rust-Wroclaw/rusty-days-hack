use image::{ImageBuffer, Rgb, RgbImage};

const MAX_TRACE_STEPS: usize = 200;
const MIN_DIST: f64 = 0.001;
const MAX_DIST: f64 = 100.0;
const CAMERA_POS: Point = Point::new(3.0, 4.0, -4.0);
const LIGHT_POS: Point = Point::new(5.0, 6.0, -4.0);

fn main() {
    let width = 640;
    let height = 480;
    let mut img: RgbImage = ImageBuffer::new(width, height);
    let screen_dim = Point::new(width as f64, height as f64, 0.0);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        *pixel = render(Point::new(x as f64, y as f64, 0.0), screen_dim);
    }

    img.save("fractal.png").unwrap();
}

fn render(p: Point, screen_dim: Point) -> Rgb<u8> {
    let uv = Point::new(
        (p.x - 0.5 * screen_dim.x) / screen_dim.y,
        (p.y - 0.5 * screen_dim.y) / screen_dim.y,
        0.0,
    );

    let rd = get_camera_ray_dir(uv, CAMERA_POS, Point::new(0.0, 1.0, 0.0), 1.);
    let d = cast_ray(CAMERA_POS, rd);

    let mut col = 0.0;
    if d < MAX_DIST {
        let p = CAMERA_POS.add(rd.mul_scalar(d));
        col = get_light(p);
    }
    col = col.powf(0.4545);

    let col = (col * 256.0) as u8;
    Rgb([col, col, col])
}

fn get_camera_ray_dir(uv: Point, p: Point, l: Point, z: f64) -> Point {
    let f = l.sub(p).normalize();
    let r = Point::new(0.0, 1.0, 0.0).cross(f).normalize();
    let u = f.cross(r);
    let c = p.add(f.mul_scalar(z));
    let i = c.add(r.mul_scalar(uv.x)).add(u.mul_scalar(uv.y));
    i.sub(p).normalize()
}

fn cast_ray(ro: Point, rd: Point) -> f64 {
    let mut t = 0.0;

    for _ in 0..MAX_TRACE_STEPS {
        let p = ro.add(rd.mul_scalar(t));
        let res = estimate_distance(p);
        t += res;
        if res < MIN_DIST || t > MAX_DIST {
            break;
        }
    }

    t
}

fn get_light(p: Point) -> f64 {
    let l = LIGHT_POS.sub(p).normalize();
    let n = get_normal(p);

    (n.dot(l) * 0.5 + 0.5).max(0.).min(1.)
}

fn get_normal(p: Point) -> Point {
    let d = estimate_distance(p);
    let ex = 0.001;
    let ey = 0.0;

    Point::new(
        d - estimate_distance(Point::new(p.x - ex, p.y - ey, p.z - ey)),
        d - estimate_distance(Point::new(p.x - ey, p.y - ex, p.z - ey)),
        d - estimate_distance(Point::new(p.x - ey, p.y - ey, p.z - ex)),
    )
    .normalize()
}

fn estimate_distance(p: Point) -> f64 {
    p.r#mod(1.).add_scalar(-0.5).length() - 0.15
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
