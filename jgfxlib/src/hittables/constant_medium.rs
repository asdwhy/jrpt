use std::{sync::Arc, ops::Range, f64::{NEG_INFINITY, INFINITY}};

use rand::{rngs::SmallRng, Rng};

use crate::{
    hittables::Hittable, 
    materials::{
        Material, 
        isotropic::Isotropic
    }, 
    textures::Texture, 
    colour::Colour, 
    aabb::AABB, 
    ray::Ray,
    vec3::Vec3
};

use super::HitRecord;


pub struct ConstantMedium {
    boundary: Arc<dyn Hittable>,
    phase_function: Arc<dyn Material>,
    neg_inv_density: f64
}

impl ConstantMedium {
    pub fn new(obj: Arc<dyn Hittable>, density: f64, colour: Colour) -> Self {
        Self {
            boundary: obj.clone(),
            phase_function: Arc::new(Isotropic::new(colour)),
            neg_inv_density: -1.0/density
        }
    }

    pub fn from_texture(obj: Arc<dyn Hittable>, density: f64, texture: Arc<dyn Texture>) -> Self {
        Self {
            boundary: obj.clone(),
            phase_function: Arc::new(Isotropic::from_texture(texture)),
            neg_inv_density: -1.0/density
        }
    }
}

impl Hittable for ConstantMedium {
    fn intersect(&self, rng: &mut SmallRng, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut rec1 = if let Some(rec) = self.boundary.intersect(rng, r, NEG_INFINITY, INFINITY) { rec } else {
            return None;
        };

        let mut rec2 = if let Some(rec) = self.boundary.intersect(rng, r, rec1.t + 0.0001, INFINITY) { rec } else {
            return None;
        };

        if rec1.t < t_min { rec1.t = t_min; }
        if rec2.t > t_max { rec2.t = t_max; }

        if rec1.t >= rec2.t {
            return None;
        }

        if rec1.t < 0.0 {
            rec1.t = 0.0;
        }

        let ray_len = r.dir.length();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_len;
        let hit_distance = self.neg_inv_density * rng.gen::<f64>().ln();

        if hit_distance > distance_inside_boundary {
            return None;
        }

        let t = rec1.t + hit_distance / ray_len;
        let p = r.at(t);

        let n = Vec3::new(1.0,0.0,0.0);

        let mut rec = HitRecord::new(t, p, n, &self.phase_function, 0.0, 0.0);
        rec.set_face_normal(r);

        Some(rec)
    }

    fn bounding_box(&self, time: Range<f64>) -> Option<AABB> {
        self.boundary.bounding_box(time)
    }
}