use std::ops::Add;
use std::ops::Mul;
use std::ops::Rem;
use std::ops::Sub;

#[derive(Copy, Clone, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point {
    pub const ZERO: Point = Point::new(0.0, 0.0, 0.0);

    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn length(self) -> f64 {
        1.0 / self.dot(self).powf(-0.5)
    }

    pub fn dot(self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn normalize(self) -> Self {
        self * self.dot(self).powf(-0.5)
    }

    pub fn cross(self, other: Self) -> Self {
        Point::new(self.y * other.z, self.z * other.x, self.x * other.y)
            - Point::new(self.z * other.y, self.x * other.z, self.y * other.x)
    }

    pub fn abs(&self) -> Self {
        Point::new(self.x.abs(), self.y.abs(), self.z.abs())
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Point::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Add<f64> for Point {
    type Output = Self;

    fn add(self, rhs: f64) -> Self {
        Point::new(self.x + rhs, self.y + rhs, self.z + rhs)
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Point::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}
impl Sub<f64> for Point {
    type Output = Self;

    fn sub(self, val: f64) -> Self {
        Point::new(self.x - val, self.y - val, self.z - val)
    }
}

impl Mul for Point {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Point::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl Mul<f64> for Point {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Point::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Rem<f64> for Point {
    type Output = Self;

    fn rem(self, rhs: f64) -> Self {
        Point::new(r#mod(self.x, rhs), r#mod(self.y, rhs), r#mod(self.z, rhs))
    }
}

fn r#mod(a: f64, b: f64) -> f64 {
    a - b * (a / b).floor()
}
