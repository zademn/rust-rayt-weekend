use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::{random_in_unit_sphere, random_unit_in_unit_sphere, Color, Vec3};

/// Trait for Materials
pub trait Material: Sync + Send {
    /// Produce a scattered ray and how much the ray should be attenuated
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
}

impl std::fmt::Debug for dyn Material {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "this is a material")
    }
}

#[derive(Debug)]
pub struct Lambertian {
    albedo: Color,
}
impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}
impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = rec.normal + random_unit_in_unit_sphere();

        // catch degenerate scatter dir
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        let scattered = Ray::new(rec.point, scatter_direction);
        let atteuation = self.albedo;
        Some((atteuation, scattered))
    }
}

#[derive(Debug)]
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}
impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: if fuzz < 1. { fuzz } else { 1. },
        }
    }
}
impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let reflected: Vec3 = r_in.dir().reflect(&rec.normal);

        let scattered: Ray = Ray::new(rec.point, reflected + self.fuzz * random_in_unit_sphere());
        let atteuation: Color = self.albedo;
        if scattered.dir().dot(&rec.normal) > 0. {
            Some((atteuation, scattered))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    ir: f64, // Index of refraction
}

impl Dielectric {
    pub fn new(ir: f64) -> Self {
        Self { ir }
    }
    ///Shlick's approximation for reflectance
    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let mut r0 = (1. - ref_idx) / (1. + ref_idx);
        r0 = r0 * r0;
        r0 = r0 + (1. - r0) * (1. - cosine).powi(5);
        r0
    }
}
impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let attenuation = Color::one();
        let refraction_ratio = if rec.front_face {
            1. / self.ir
        } else {
            self.ir
        };

        let unit_direction: Vec3 = r_in.dir().unit_vector();

        let cos_theta: f64 = -unit_direction.dot(&rec.normal).min(1.);
        let sin_theta: f64 = (1. - cos_theta * cos_theta).sqrt();
        let cannot_refract: bool = refraction_ratio * sin_theta > 1.;
        let direction: Vec3 = if cannot_refract
            || Dielectric::reflectance(cos_theta, refraction_ratio) > rand::random::<f64>()
        {
            unit_direction.reflect(&rec.normal)
        } else {
            unit_direction.refract(&rec.normal, refraction_ratio)
        };
        let scattered: Ray = Ray::new(rec.point, direction);
        Some((attenuation, scattered))
    }
}
