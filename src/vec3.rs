use rand::distributions::{Distribution, Uniform};
use rand::Rng;
use std::cmp::PartialEq;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
// Type aliases for Vec3
pub type Point3 = Vec3; // #d point
pub type Color = Vec3; // RGB color

/// Generates a random vector in the unit sphere
pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = Vec3::random_interval(-1., 1.);
        if p.norm_squared() < 1. {
            return p;
        }
    }
}
/// Generates a random unit vector in the unit sphere
pub fn random_unit_in_unit_sphere() -> Vec3 {
    random_in_unit_sphere().unit_vector()
}

/// Uniform scatter for all angles **away** from the hit point
pub fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
    let in_unit_sphere = random_in_unit_sphere();
    if in_unit_sphere.dot(normal) > 0. {
        in_unit_sphere
    } else {
        -in_unit_sphere
    }
}

/// Generates a vector in a unit 2d disk
pub fn random_in_unit_disk() -> Vec3 {
    loop {
        let p = Vec3::new(rand::random::<f64>(), rand::random::<f64>(), 0.);
        if p.norm_squared() < 1. {
            return p;
        }
    }
}

// Derive PartialEq implies 2 vectors are equal if all fields are equal
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    /// Returns the zero vector
    pub fn zero() -> Self {
        Self {
            x: 0.,
            y: 0.,
            z: 0.,
        }
    }
    /// Returns the one vector
    pub fn one() -> Self {
        Self {
            x: 1.,
            y: 1.,
            z: 1.,
        }
    }
    /// Returns a random vector
    pub fn random() -> Self {
        Self {
            x: rand::random::<f64>(),
            y: rand::random::<f64>(),
            z: rand::random::<f64>(),
        }
    }
    /// Returns a random vector with coordinates uniformly sampled between min and max
    pub fn random_interval(min: f64, max: f64) -> Self {
        let d = Uniform::from(min..max);
        let mut rng = rand::thread_rng();
        Self {
            x: d.sample(&mut rng),
            y: d.sample(&mut rng),
            z: d.sample(&mut rng),
        }
    }

    /// Creates a new vector given coordinates x, y, z
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
    /// Gets the x coordinate
    pub fn x(&self) -> f64 {
        self.x
    }
    /// Gets the x coordinate
    pub fn y(&self) -> f64 {
        self.y
    }
    /// Gets the z coordinate
    pub fn z(&self) -> f64 {
        self.z
    }
    /// Gets the euclidean norm of the vector (the length)
    /// sqrt(x^2+y^2+z^2)
    pub fn norm(&self) -> f64 {
        self.norm_squared().sqrt()
    }
    /// Gets the squared euclidean norm of the vector (the length)
    /// x^2 + y^2 + z^2
    pub fn norm_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    /// Dot product between two vectors
    pub fn dot(&self, other: &Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    /// Cros product between two vectors
    pub fn cross(&self, other: &Vec3) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
    /// Returns the unit vector of self
    /// self divided by the norm
    pub fn unit_vector(&self) -> Vec3 {
        *self / self.norm()
    }
    /// Returns true if the vector is near 0 in all dimensions
    pub fn near_zero(&self) -> bool {
        let threshold = 1e-8f64;
        (self.x.abs() < threshold) && (self.y.abs() < threshold) && (self.z.abs() < threshold)
    }
    /// The reflected ray direction is v + 2b.
    /// b = v * n where n is the normal and is a unit vector
    /// Because v points inwards we need a minus sign
    pub fn reflect(&self, normal: &Vec3) -> Vec3 {
        *self - 2. * self.dot(normal) * *normal
    }

    /// Based on refraction equations
    pub fn refract(&self, normal: &Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = -self.dot(normal).min(1.);
        let r_out_perp = etai_over_etat * (*self + cos_theta * *normal);
        let r_out_parallel = -(1. - r_out_perp.norm_squared()).abs().sqrt() * *normal;
        r_out_perp + r_out_parallel
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Self {
            x: 0.,
            y: 0.,
            z: 0.,
        }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul for Vec3 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}
// Should I force the Copy trait here?
impl<T: Into<f64> + Copy> Mul<T> for Vec3 {
    type Output = Vec3;
    fn mul(self, scalar: T) -> Self {
        Self {
            x: self.x * scalar.into(),
            y: self.y * scalar.into(),
            z: self.z * scalar.into(),
        }
    }
}
// Should I force the Copy trait here?
impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, scalar: f64) -> Self {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        *self = *self + other;
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Vec3) {
        *self = *self - other;
    }
}
impl MulAssign for Vec3 {
    fn mul_assign(&mut self, other: Vec3) {
        *self = *self * other;
    }
}
impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, v: Vec3) -> Vec3 {
        v * self
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, scalar: f64) {
        *self = *self * scalar;
    }
}
impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, scalar: f64) {
        *self = *self / scalar;
    }
}

impl std::iter::Sum for Vec3 {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Vec3::zero(), |acc, v| acc + v)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn vec3_default() {
        let v = Vec3::default();
        assert_eq!((v.x, v.z, v.y), (0., 0., 0.));
    }

    #[test]
    fn vec3_new() {
        let v = Vec3::new(1., 2., 3.);
        assert_eq!((v.x, v.y, v.z), (1., 2., 3.));
    }
    #[test]
    fn vec3_operations() {
        let mut v1 = Vec3::new(1., 2., 3.);
        let v2 = Vec3::new(10., 20., 30.);
        assert_eq!(v1 + v2, Vec3::new(11., 22., 33.));
        assert_eq!(-v1, Vec3::new(-1., -2., -3.));
        assert_eq!(v1 - v2, Vec3::new(-9., -18., -27.));
        assert_eq!(v1 * 2., Vec3::new(2., 4., 6.));
        assert_eq!(v1 / 2., Vec3::new(0.5, 1., 1.5));
        assert_eq!(v1 * v2, Vec3::new(10., 40., 90.));

        v1 += v2;
        assert_eq!(v1, Vec3::new(11., 22., 33.));
        v1 -= v2;
        assert_eq!(v1, Vec3::new(1., 2., 3.));
        v1 *= 2.;
        assert_eq!(v1, Vec3::new(2., 4., 6.));
        v1 /= 2.;
        assert_eq!(v1, Vec3::new(1., 2., 3.));

        assert_eq!(v1.dot(&v2), 10. + 40. + 90.);
        assert_eq!(v1.cross(&v2), Vec3::new(0., 0., 0.));

        assert_eq!(
            v1.unit_vector(),
            Vec3::new(1. / 14f64.sqrt(), 2. / 14f64.sqrt(), 3. / 14f64.sqrt())
        )
    }
    #[test]
    fn vec3_random() {
        let v1 = Vec3::random();
        dbg!(v1);
        let v2 = Vec3::random_interval(10., 100.);
        dbg!(v2);
    }
}
