use std::{ops::Range, sync::Arc};
use rand::rngs::SmallRng;
use crate::{
    aabb::AABB,
    ray::Ray,
    point3::Point3,
    materials::Material,
    hittables::{
        hittable_list::HittableList, 
        aa_rectangles::{xy_rect::XyRectangle, xz_rect::XzRectangle, yz_rect::YzRectangle}, 
        Hittable
    }
};

use super::HitRecord;

// Rust has "box" keyword reserved so... rectangular prism!
pub struct RectangularPrism {
    // corners of the prism
    min: Point3,
    max: Point3,
    sides: HittableList
}

impl RectangularPrism {
    /// Create rectangular prism defined by the corners p0, p1
    pub fn new(p0: Point3, p1: Point3, material: Arc<dyn Material>) -> Self {
        let mut sides = HittableList::new();

        sides.add(Arc::new(XyRectangle::new(p0.x, p1.x, p0.y, p1.y, p1.z, material.clone())));
        sides.add(Arc::new(XyRectangle::new(p0.x, p1.x, p0.y, p1.y, p0.z, material.clone())));

        sides.add(Arc::new(XzRectangle::new(p0.x, p1.x, p0.z, p1.z, p1.y, material.clone())));
        sides.add(Arc::new(XzRectangle::new(p0.x, p1.x, p0.z, p1.z, p0.y, material.clone())));

        sides.add(Arc::new(YzRectangle::new(p0.y, p1.y, p0.z, p1.z, p1.x, material.clone())));
        sides.add(Arc::new(YzRectangle::new(p0.y, p1.y, p0.z, p1.z, p0.x, material.clone())));

        Self {
            min: p0,
            max: p1,
            sides
        }
    }

    /// Create canonical rectangular prism defined by the corners P0(0.0,0.0,0.0), P1(1.0,1.0,1.0)
    pub fn canonical(material: Arc<dyn Material>) -> Self {
        let p0 = Point3::zero();
        let p1 = Point3::from_value(1.0);

        let mut sides = HittableList::new();

        sides.add(Arc::new(XyRectangle::new(p0.x, p1.x, p0.y, p1.y, p1.z, material.clone())));
        sides.add(Arc::new(XyRectangle::new(p0.x, p1.x, p0.y, p1.y, p0.z, material.clone())));

        sides.add(Arc::new(XzRectangle::new(p0.x, p1.x, p0.z, p1.z, p1.y, material.clone())));
        sides.add(Arc::new(XzRectangle::new(p0.x, p1.x, p0.z, p1.z, p0.y, material.clone())));

        sides.add(Arc::new(YzRectangle::new(p0.y, p1.y, p0.z, p1.z, p1.x, material.clone())));
        sides.add(Arc::new(YzRectangle::new(p0.y, p1.y, p0.z, p1.z, p0.x, material.clone())));

        Self {
            min: p0,
            max: p1,
            sides
        }
    }
}

impl Hittable for RectangularPrism {
    fn bounding_box(&self, _: Range<f64>) -> Option<AABB> {
        Some(AABB::new(self.min, self.max))
    }

    fn intersect(&self, rng: &mut SmallRng, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.sides.intersect(rng, r, t_min, t_max)
    }
}