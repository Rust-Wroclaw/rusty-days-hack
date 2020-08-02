#[derive(Copy, Clone, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point {
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn mul_scalar(&self, value: f64) -> Self {
        Self::new(self.x * value, self.y * value, self.z * value)
    }

    pub fn add(&self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }

    pub fn add_scalar(&self, value: f64) -> Self {
        Self::new(self.x + value, self.y + value, self.z + value)
    }

    pub fn sub(&self, other: Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }

    pub fn length(&self) -> f64 {
        1.0 / self.dot(*self).powf(-0.5)
    }

    pub fn dot(&self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn normalize(&self) -> Self {
        self.mul_scalar(self.dot(*self).powf(-0.5))
    }

    pub fn cross(&self, other: Self) -> Self {
        Self::new(self.y * other.z, self.z * other.x, self.x * other.y).sub(Self::new(
            self.z * other.y,
            self.x * other.z,
            self.y * other.x,
        ))
    }

    pub fn r#mod(&self, n: f64) -> Self {
        Self::new(r#mod(self.x, n), r#mod(self.y, n), r#mod(self.z, n))
    }

    pub fn abs(&self) -> Self {
        Self::new(self.x.abs(), self.y.abs(), self.z.abs())
    }
}

fn r#mod(a: f64, b: f64) -> f64 {
    a - b * (a / b).floor()
}
