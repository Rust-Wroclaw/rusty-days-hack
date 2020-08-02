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

    pub fn abs(self) -> Self {
        Point::new(self.x.abs(), self.y.abs(), self.z.abs())
    }

    pub fn distance(self, other: Self) -> f64 {
        let v = other - self;
        v.dot(v).sqrt()
    }

    pub fn rotate(self, angles: Self) -> Self {
        let Point {
            x: alpha,
            y: beta,
            z: gamma,
        } = angles;
        let (sin_alpha, cos_alpha) = (alpha.sin(), alpha.cos());
        let (sin_beta, cos_beta) = (beta.sin(), beta.cos());
        let (sin_gamma, cos_gamma) = (gamma.sin(), gamma.cos());
        let mut transf_mat = [[0.; 3]; 3];
        transf_mat[0][0] = cos_alpha * cos_beta;
        transf_mat[0][1] = cos_alpha * sin_beta * sin_gamma - sin_alpha * cos_gamma;
        transf_mat[0][2] = cos_alpha * sin_beta * cos_gamma + sin_alpha * sin_gamma;
        transf_mat[1][0] = sin_alpha * cos_beta;
        transf_mat[1][1] = sin_alpha * sin_beta * sin_gamma + cos_alpha * cos_gamma;
        transf_mat[1][2] = sin_alpha * sin_beta * cos_gamma - cos_alpha * sin_gamma;
        transf_mat[2][0] = -sin_beta;
        transf_mat[2][1] = cos_beta * sin_gamma;
        transf_mat[2][2] = cos_beta * cos_gamma;
        self.apply_transformation(transf_mat)
    }

    fn apply_transformation(self, mat: [[f64; 3]; 3]) -> Self {
        Point::new(
            self.x * mat[0][0] + self.y * mat[0][1] + self.z * mat[0][2],
            self.x * mat[1][0] + self.y * mat[1][1] + self.z * mat[1][2],
            self.x * mat[2][0] + self.y * mat[2][1] + self.z * mat[2][2],
        )
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

#[cfg(test)]
mod test {
    use super::*;
    use std::f64::consts::PI;
    const ERROR_THRESHOLD: f64 = 1e-10;
    const P: Point = Point::new(5., 3., 10.);

    fn assert_if_within_error_threshold(actual: &Point, expected: &Point) {
        assert!(
            (actual.x - expected.x).abs() < ERROR_THRESHOLD,
            "the x values have big diff {:?} - {:?}",
            actual,
            expected
        );
        assert!(
            (actual.y - expected.y).abs() < ERROR_THRESHOLD,
            "the y values have big diff {:?} - {:?}",
            actual,
            expected
        );
        assert!(
            (actual.z - expected.z).abs() < ERROR_THRESHOLD,
            "the z values have big diff {:?} - {:?}",
            actual,
            expected
        );
    }

    #[test]
    fn rotation_by_z_axis() {
        let rotated = P.rotate(Point::new(PI / 2., 0., 0.));
        assert_if_within_error_threshold(&rotated, &Point::new(-3., 5., 10.));
    }

    #[test]
    fn rotation_by_y_axis() {
        let rotated = P.rotate(Point::new(0., PI / 2., 0.));
        assert_if_within_error_threshold(&rotated, &Point::new(10., 3., -5.));
    }

    #[test]
    fn rotation_by_x_axis() {
        let rotated = P.rotate(Point::new(0., 0., PI / 2.));
        assert_if_within_error_threshold(&rotated, &Point::new(5., -10., 3.));
    }
}
