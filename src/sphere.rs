use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use std::rc::Rc;
use std::sync::Arc;
#[derive(Debug)]
pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Arc<dyn Material>,
}

impl Sphere {
    /// Create a new sphere
    /// # Arguments
    /// * `center` - The center of the sphere
    /// * `radius` - The radius of the sphere
    /// * `material` - The material of the sphere
    pub fn new(center: Point3, radius: f64, material: Arc<dyn Material>) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    /// Check if the sphere is hit by a ray
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc: Vec3 = r.orig() - self.center;
        // quadratic terms are known
        let a = r.dir().norm_squared();
        let half_b = oc.dot(&r.dir());
        let c = oc.norm_squared() - self.radius * self.radius;
        let delta = half_b * half_b - a * c;
        if delta < 0. {
            return None;
        }

        let delta_sqrt = delta.sqrt();

        // Find the nearest root
        let mut root = (-half_b - delta_sqrt) / a;
        if root < t_min || root > t_max {
            root = (-half_b + delta_sqrt) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }


        let mut rec = HitRecord {
            t: root,
            point: r.at(root),
            normal: Vec3::zero(),
            front_face: false,
            material: Arc::clone(&self.material),
        };
        let outward_normal = (rec.point - self.center) / self.radius;
        rec.set_face_normal(r, &outward_normal);

        Some(rec)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::material::Lambertian;
    use crate::vec3::Color;
    #[test]
    fn sphere_hit() {
        let center = Point3::new(10., 10., 10.);
        let radius = 2.;
        let material = Lambertian::new(Color::zero());
        let _sphere = Sphere::new(center, radius, Arc::new(material));
        let r_orig = Point3::zero();
        let r_dir = Point3::one();
        let _r = Ray::new(r_orig, r_dir);
    }
}
