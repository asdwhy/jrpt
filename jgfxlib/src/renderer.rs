use image::{ImageBuffer, RgbImage, Rgb};
use rayon::prelude::*;

use crate::scene::Scene;
use crate::ray::Ray;
use crate::hittables::{Hittable}; 
use crate::constants::{INFINITY, EPSILON};
use crate::utils::max;
use crate::colour::{Colour};
use crate::random::random_f64;

pub struct Renderer {
    antialiasing: u32,
    depth: u32,
    debug: bool,
    multithreading: bool,
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            antialiasing: 10,
            depth: 10,
            debug: false,
            multithreading: false,
        }
    }

    /// Set image to take val samples
    /// If val is 0, will sample once
    pub fn set_antialiasing(&mut self, val: u32) {
        self.antialiasing = max(1, val);
    }

    /// Print debug messages
    pub fn set_debug(&mut self, val: bool) {
        self.debug = val;
    }

    /// Set the recursion depth
    pub fn set_depth(&mut self, val: u32) {
        self.depth = max(1, val);
    }

    /// Allow this render to be multithreaded
    pub fn set_multithreading(&mut self, val: bool) {
        self.multithreading = val;
    }

    pub fn render(&mut self, scene: &Scene, image_height: u32, image_width: u32) -> RgbImage {
        let mut img = ImageBuffer::new(image_width, image_height);

        let f = |(i, j, pixel): (u32, u32, &mut Rgb<u8>) | {
            let j = image_height - j;
                        // because from top down

            let col = self.antialias(scene, i, j, image_height, image_width);

            *pixel = col.to_rgb();
        };

        if self.multithreading {
            img.enumerate_pixels_mut().par_bridge().for_each(f);
        } else {
            img.enumerate_pixels_mut().for_each(f);
        }

        img
    }

    /// Antialias num_samples times on pixel (i,j)
    fn antialias(&self, scene: &Scene, i: u32, j: u32, height: u32, width: u32) -> Colour {
        let num_samples = self.antialiasing;
        let mut col = Colour::zero();

        (0..num_samples).into_iter().for_each(|_| {

            let u_ = ((i as f64) + random_f64()) / (width - 1) as f64;
            let v_ = ((j as f64) + random_f64()) / (height - 1) as f64;

            let r = scene.camera.get_ray(u_, v_);
            col += self.path_trace(scene, r, self.depth);
        });

        col / num_samples as f64
    }

    fn path_trace(&self, scene: &Scene, r: Ray, depth: u32) -> Colour {
        // max recursion limit reached
        if depth <= 0 {
            return Colour::zero();
        }

        let hit = scene.objects.intersect(&r, EPSILON, INFINITY);

        match hit {
            None => {
                let unit_dir = r.dir.normalized();
    
                let t = 0.5 * (unit_dir.y + 1.0);
                
                (1.0 - t) * Colour::new(1.0,1.0,1.0) + t*Colour::new(0.5, 0.7, 1.0)
            },
            Some(rec) => {
                match rec.material.scatter(r, &rec) {
                    Some((attenuation, scattered)) => {
                        attenuation * self.path_trace(scene, scattered, depth - 1)
                    },
                    None => Colour::zero()
                }
            }
        }
    }
}
