use std::sync::Arc;

use jrpt::{
    objects::{
        object_list::{ObjectList, self},
        aa_rectangles::{
            yz_rect::{YzRectangle, self},
            xz_rect::{XzRectangle, self},
            xy_rect::{XyRectangle, self}
        }, 
        rect_prism::{RectangularPrism, self},
        Object,
        affine::{Affine, self}, constant_medium::{ConstantMedium, self}
    }, 
    materials::{
        lambertian::Lambertian, diffuse_light::DiffuseLight
    }, 
    colour::Colour, point3::Point3
};

pub fn build_scene() -> Object {
    let mut world = object_list::new();

    let red = Arc::new(Lambertian::new(Colour::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new(Colour::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new(Colour::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::new(Colour::new(15.0, 15.0, 15.0)));

    // walls
    object_list::add(&mut world, yz_rect::new(0.0, 555.0, 0.0, 555.0, 555.0, green));
    object_list::add(&mut world, yz_rect::new(0.0, 555.0, 0.0, 555.0, 0.0, red));
    object_list::add(&mut world, xz_rect::new(213.0, 343.0, 227.0, 332.0, 554.0, light));
    object_list::add(&mut world, xz_rect::new(0.0, 555.0, 0.0, 555.0, 0.0, white.clone()));
    object_list::add(&mut world, xz_rect::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone()));
    object_list::add(&mut world, xy_rect::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone()));

    // boxes
    let b = rect_prism::new(Point3::zero(), Point3::new(165.0, 330.0, 165.0), white.clone());
    let mut transform = affine::new(b);
    affine::rotate_y(&mut transform, (15.0 as f64).to_radians());
    affine::translate(&mut transform, 265.0, 0.0, 295.0);
    affine::set_inverse(&mut transform);
    object_list::add(&mut world, constant_medium::new(transform, 0.01, Colour::zero()));

    let b = rect_prism::new(Point3::zero(), Point3::from_value(165.0), white.clone());
    let mut transform = affine::new(b);
    affine::rotate_y(&mut transform, (-18.0 as f64).to_radians());
    affine::translate(&mut transform, 130.0, 0.0, 65.0);
    affine::set_inverse(&mut transform);
    object_list::add(&mut world, constant_medium::new(transform, 0.01, Colour::from_value(1.0)));

    world
}
