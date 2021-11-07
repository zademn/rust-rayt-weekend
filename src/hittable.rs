use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use std::rc::Rc;
use std::sync::Arc;

/// A hit record keeps track of a "hit"s details
/// It keeps the point, normal to the surface, the material type, wether the hit is inside or outside the object.
//#[derive(Debug, Copy, Clone)]
pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub material: Arc<dyn Material>,
    pub t: f64,
    pub front_face: bool,
}
impl HitRecord {
    /// Given a ray and a normal that points outside it sets if the we hit the front face or the back face of the surface
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = r.dir().dot(outward_normal) < 0.;
        if self.front_face {
            self.normal = *outward_normal;
        } else {
            self.normal = -*outward_normal;
        }
    }
}

/// A trait that binds hittable objects
pub trait Hittable: Sync + Send {
    /// A hit takes a ray and the min and max timepoints it travels and
    /// returns a record of something hit or None
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
/// A list of Hittable objects
pub struct HittableList {
    // reference list
    objects: Vec<Box<dyn Hittable>>,
}
impl HittableList {
    /// Creates an empty list
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }
    /// Adds a reference of an object to the list
    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
    /// Clears the list
    pub fn clear(&mut self) {
        self.objects.clear();
    }
    /// Length of the list
    pub fn len(&self) -> usize {
        self.objects.len()
    }
}
impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut temp_rec = None;
        let mut closest_so_far = t_max;

        for object in self.objects.iter() {
            if let Some(rec) = object.hit(r, t_min, closest_so_far) {
                closest_so_far = rec.t;
                temp_rec = Some(rec);
            }
        }
        temp_rec
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::material::Metal;
    use crate::sphere::Sphere;
    use crate::vec3::Color;
    #[test]
    fn hittable_list_create_add() {
        let mut hl = HittableList::new();
        let mat = Metal::new(Color::one(), 0.5);
        let mat2 = Metal::new(Color::one(), 1.);
        let s1 = Sphere::new(Point3::zero(), 1., Arc::new(mat));
        let s2 = Sphere::new(Point3::one(), 0.5, Arc::new(mat2));
        hl.add(Box::new(s1));
        hl.add(Box::new(s2));

        assert_eq!(hl.len(), 2);
    }
}
