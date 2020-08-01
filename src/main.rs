use image::{ImageBuffer, Rgb, RgbImage};

const MAX_TRACE_STEPS: usize = 64;
const MIN_DIST: f64 = 0.0001;

fn main() {
    let width = 800;
    let height = 800;
    let mut img: RgbImage = ImageBuffer::new(width, height);
    let screen_dim = Point::new(width as f64, height as f64, 0.);
    let camera = Point::new(-2., -2., -5.);
    let look_at = Point::ZERO;

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let uv = normalize_screen_coords(Point::new(x as f64, y as f64, 0.), screen_dim);
        let ray_dir = get_camera_ray_dir(uv, camera, look_at);
        *pixel = render(camera, ray_dir);
    }

    img.save("fractal.png").unwrap();
}

fn normalize_screen_coords(p: Point, screen_dim: Point) -> Point {
    Point::new(
        2.0 * (p.x / screen_dim.x - 0.5),
        2.0 * (p.y / screen_dim.y - 0.5),
        0.,
    )
}

fn get_camera_ray_dir(image_pxl: Point, camera_pos: Point, camera_dir: Point) -> Point {
    let cam_forward = camera_dir.sub(camera_pos).normalize();
    let cam_right = Point::new(0., 1., 0.).cross(cam_forward).normalize();
    let cam_up = cam_forward.cross(cam_right).normalize();
    let persp = 2.0;

    let direction_vec = cam_right
        .mul_scalar(image_pxl.x)
        .add(cam_up.mul_scalar(image_pxl.y))
        .add(cam_forward.mul_scalar(persp));
    direction_vec.normalize()
}

fn render(camera: Point, dir: Point) -> Rgb<u8> {
    let mut col;
    let t = cast_ray(camera, dir);

    if t == -1. {
        let val = dir.y * 0.4;
        col = Point::new(0.9, 0.9, 0.9).sub(Point::new(val, val, val));
    } else {
        col = Point::new(0.1, 0.1, 0.1);
    }

    col = col.pow(0.4545);

    Rgb([
        (col.x * 256.) as u8,
        (col.y * 256.) as u8,
        (col.z * 256.) as u8,
    ])
}

fn cast_ray(cam_pos: Point, cam_dir: Point) -> f64 {
    let tmax = 100.;
    let mut t = 0.0;

    for _ in 0..MAX_TRACE_STEPS {
        let pos = cam_pos.add(cam_dir.mul_scalar(t));
        let res = estimate_distance_tri2(pos);
        if res < MIN_DIST * t {
            return t;
        } else if res > tmax {
            return -1.;
        }
        t += res;
    }

    -1.0
}

fn estimate_distance_sphere(pos: Point) -> f64 {
    pos.sub(Point::new(0., 0., 10.)).length() - 3.0
}

fn estimate_distance(mut z: Point) -> f64 {
    z.x = z.x % 1. - 0.5;
    z.y = z.y % 1. - 0.5;
    return z.length() - 0.3;
}

fn estimate_distance_tri2(mut z: Point) -> f64 {
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
    return (z.length()) * scale.powf(-n as f64);
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

    return z.length() * scale.powf(-n as f64);
}

#[derive(Copy, Clone, Debug)]
struct Point {
    x: f64,
    y: f64,
    z: f64,
}

impl Point {
    const ZERO: Point = Point {
        x: 0.,
        y: 0.,
        z: 0.,
    };

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn mul(&self, other: Point) -> Point {
        Point::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }

    pub fn mul_scalar(&self, value: f64) -> Point {
        Point::new(self.x * value, self.y * value, self.z * value)
    }

    pub fn add(&self, other: Point) -> Point {
        Point::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }

    pub fn sub(&self, other: Point) -> Point {
        self.add(other.mul_scalar(-1.))
    }

    pub fn length(&self) -> f64 {
        self.dot(*self).sqrt()
    }

    pub fn dot(&self, other: Point) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn normalize(&self) -> Point {
        self.mul_scalar(self.dot(*self).powf(-0.5))
    }

    pub fn pow(&self, n: f64) -> Point {
        Point::new(self.x.powf(n), self.y.powf(n), self.z.powf(n))
    }

    pub fn cross(&self, other: Point) -> Point {
        Point::new(self.y * other.z, self.z * other.x, self.x * other.y).sub(Point::new(
            self.z * other.y,
            self.x * other.z,
            self.y * other.x,
        ))
    }
}
