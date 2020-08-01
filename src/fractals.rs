use crate::point::Point;
use image::{ImageBuffer, Rgb, RgbImage};

const MAX_TRACE_STEPS: usize = 1000;
const MIN_DIST: f64 = 0.001;
const MAX_DIST: f64 = 1000.0;

pub trait Fractal {
    const CAMERA_POS: Point;
    fn estimate_distance(p: Point) -> f64;

    fn img(width: u32, height: u32) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        let mut img: RgbImage = ImageBuffer::new(width, height);
        let screen_dim = Point::new(width as f64, height as f64, 0.0);
        for (x, y, pixel) in img.enumerate_pixels_mut() {
            *pixel = Self::render(Point::new(x as f64, y as f64, 0.0), screen_dim);
        }
        img
    }

    fn render(p: Point, screen_dim: Point) -> Rgb<u8> {
        let uv = Point::new(
            (p.x - 0.5 * screen_dim.x) / screen_dim.y,
            (p.y - 0.5 * screen_dim.y) / screen_dim.y,
            0.0,
        );
        let rd = Self::get_camera_ray_dir(uv, Self::CAMERA_POS, Point::new(0.0, 0.0, 0.0), 1.);
        let d = Self::cast_ray(Self::CAMERA_POS, rd);
        let mut col = 0.0;
        if d < MAX_DIST {
            let p = Self::CAMERA_POS.add(rd.mul_scalar(d));
            col = Self::get_light(p);
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
            let res = Self::estimate_distance(p);
            t += res;
            if res < MIN_DIST || t > MAX_DIST {
                break;
            }
        }
        t
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
        let ey = 0.0;
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
    const CAMERA_POS: Point = Point::new(-2.0, -2.0, -5.0);

    fn estimate_distance(mut z: Point) -> f64 {
        let a1 = Point::new(1., 1., 1.);
        let a2 = Point::new(-1., -1., 1.);
        let a3 = Point::new(1., -1., -1.);
        let a4 = Point::new(-1., 1., -1.);
        let mut c;
        let mut n = 0;
        let mut dist;
        let mut d;
        let scale = 2.;
        let iterations = 10;
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
}

pub struct TrianglesWithFold;

impl Fractal for TrianglesWithFold {
    const CAMERA_POS: Point = Point::new(-2.0, -2.0, -5.0);

    fn estimate_distance(mut z: Point) -> f64 {
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
}
