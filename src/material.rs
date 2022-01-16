use crate::Vec3;
use crate::{Color, HitRecord, Ray};
use rand;
use rand::Rng;

#[derive(Copy, Clone)]
pub enum Materials {
    Lambertian{
        albedo: Color
    },
    Metal {
        albedo: Color,
        fuzz: f64
    },
    DiElectric {
        ir: f64
    }
}

impl Materials {
    pub(crate) fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        return match self {
            Materials::Lambertian { albedo } => {
                let mut scatter_direction = rec.normal + Vec3::random_unit_vector();
                if scatter_direction.near_zero() {
                    scatter_direction = rec.normal;
                }

                *scattered = Ray::new(rec.p, scatter_direction);
                *attenuation = *albedo;

                true
            }
            Materials::Metal { albedo, fuzz } => {
                let reflected = Vec3::reflect(&r_in.dir().normalized(), &rec.normal);
                *scattered = Ray::new(rec.p, reflected + Vec3::random_in_unit_sphere() * (*fuzz));
                *attenuation = *albedo;

                scattered.dir().dot(&rec.normal) > 0.0
            }
            Materials::DiElectric { ir } => {
                *attenuation = Color::new(1.0, 1.0, 1.0);

                let refraction_ratio = if rec.front_face {
                    (1.0 / *ir)
                } else { *ir };

                let unit_direction = r_in.dir().normalized();
                let cos_theta = (-unit_direction).dot(&rec.normal).min(1.0);
                let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
                let cannot_refract = refraction_ratio * sin_theta > 1.0;
                let mut direction = Vec3::new_empty();

                if cannot_refract
                    || dieelectric_reflectance(cos_theta, refraction_ratio) > rand::thread_rng().gen::<f64>() {
                    direction = Vec3::reflect(&unit_direction, &rec.normal);
                } else {
                    direction = Vec3::refract(&unit_direction, &rec.normal, refraction_ratio);
                }

                *scattered = Ray::new(rec.p, direction);

                true
            }
        }
    }
}

pub fn dieelectric_reflectance(cosine: f64, ref_idx: f64) -> f64 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    return r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0);
}