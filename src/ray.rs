use crate::vec3::{Point3, Vec3};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {
    /// New ray that has an origin and a direction
    pub fn new(orig: Point3, dir: Vec3) -> Self {
        Self { orig, dir }
    }
    /// Returns the point at time t.
    /// P(t) = A + t*b
    pub fn at(&self, t: f64) -> Point3 {
        self.orig + self.dir * t
    }

    pub fn orig(&self) -> Point3 {
        self.orig
    }
    pub fn dir(&self) -> Vec3 {
        self.dir
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ray_new() {
        let orig = Point3::new(0., 0., 0.);
        let dir = Vec3::new(1., 1., 1.);
        let ray = Ray::new(orig, dir);
        assert_eq!((ray.orig, ray.dir), (orig, dir));
    }
    #[test]
    fn ray_at_t() {
        let orig = Point3::new(0., 0., 0.);
        let dir = Vec3::new(1., 1., 1.);
        let ray = Ray::new(orig, dir);
        let t = 64.;
        assert_eq!(ray.at(t), orig + dir * t);
    }
}
