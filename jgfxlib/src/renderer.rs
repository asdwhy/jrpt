use std::sync::Arc;

use image::buffer::EnumeratePixelsMut;
use image::{ImageBuffer, RgbImage, Rgb};
use rand::{SeedableRng, Rng, thread_rng};
use rand::rngs::SmallRng;
use rayon::prelude::*;

use crate::pdfs::Pdf;
use crate::pdfs::hittable_pdf::HittablePdf;
use crate::pdfs::mixture_pdf::MixturePdf;
use crate::scene::Scene;
use crate::ray::Ray;
use crate::hittables::{Hittable}; 
use crate::constants::{INFINITY, EPSILON};
use crate::utils::max;
use crate::colour::{Colour};

pub struct Renderer {
    num_samples: u32,
    depth: u32,
    multithreading: bool
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            num_samples: 10,
            depth: 10,
            multithreading: false
        }
    }

    /// Set image to take val samples
    /// Will sample minimum of one time
    pub fn set_num_samples(&mut self, num_samples: u32) {
        self.num_samples = max(1, num_samples);
    }

    /// Set the recursion depth
    pub fn set_depth(&mut self, depth: u32) {
        self.depth = max(1, depth);
    }

    /// Allow this render to be multithreaded
    pub fn set_multithreading(&mut self, multithreading: bool) {
        self.multithreading = multithreading;
    }

    pub fn render(&mut self, scene: &Scene, image_height: u32, image_width: u32) -> RgbImage {
        let mut img = ImageBuffer::new(image_width, image_height);

        let f = |(_, cols): (u32, EnumeratePixelsMut<Rgb<u8>>)| {
            let mut rng = SmallRng::from_rng(thread_rng()).unwrap();    
            
            cols.for_each(|(i, j, pixel): (u32, u32, &mut Rgb<u8>)| {
                let j = image_height - j;
                        // because from top down
                
                let col = self.sample_pixel(&mut rng, scene, i, j, image_height, image_width);

                *pixel = col.to_rgb();
            });
        };

        if self.multithreading {
            img.enumerate_rows_mut().par_bridge().for_each(f);
        } else {
            img.enumerate_rows_mut().for_each(f);
        }

        img
    }

    /// Antialias num_samples times on pixel (i,j)
    fn sample_pixel(&self, rng: &mut SmallRng, scene: &Scene, i: u32, j: u32, height: u32, width: u32) -> Colour {
        let mut col = Colour::zero();

        (0..self.num_samples).for_each(|_| {
            let u_ = ((i as f64) + rng.gen::<f64>()) / (width - 1) as f64;
            let v_ = ((j as f64) + rng.gen::<f64>()) / (height - 1) as f64;

            let r = scene.camera.get_ray(rng, u_, v_);
            col += self.path_trace(rng, scene, r, self.depth);
        });

        col / self.num_samples as f64
    }

    fn path_trace(&self, rng: &mut SmallRng, scene: &Scene, r: Ray, depth: u32) -> Colour {
        // max recursion limit reached
        if depth <= 0 {
            return Colour::zero();
        }

        let hit = scene.objects.intersect(rng, &r, EPSILON, INFINITY);

        match hit {
            None => {
                scene.background_colour
            },
            Some(rec) => {
                let emitted = rec.material.emitted(&r, &rec);

                match rec.material.scatter(rng, &r, &rec) {
                    Some(srec) => {
                        if let Some(specular_ray) = srec.specular_ray { // scatted ray is implicitly sampled
                            srec.attenuation * self.path_trace(rng, scene, specular_ray, depth - 1)
                        } else {
                            let light_pdf = Arc::new(HittablePdf::new(scene.lights.clone(),rec.p));
                            let p = MixturePdf::new(light_pdf, srec.pdf.unwrap()); // light should have pdf

                            let scattered = Ray::new(rec.p, p.generate(rng), r.time);
                            let pdf_val = p.value(rng, &scattered.dir);

                            emitted + (srec.attenuation * rec.material.scaterring_pdf(rng, &r, &rec, &scattered)
                            * self.path_trace(rng, scene, scattered, depth - 1)) / pdf_val
                        }
                    },
                    None => emitted // if light doesnt scatter off this object, return the light emitted from it
                }
            }
        }
    }
}
