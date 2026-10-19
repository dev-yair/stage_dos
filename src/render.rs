use glam::{Vec3};
use crate::camera::Camera;
use crate::sphere::Sphere;
use crate::film::Film;

pub fn render(cam: &Camera, spheres: &[Sphere], film: &mut Film, background: Vec3){

    for y in 0..film.height{
        for x in 0..film.width{
            
            let s = (x as f32 + 0.5) / film.width as f32;
            let t = 1.0 - ( y as f32 + 0.5) / film.height as f32;

            let pix_ray = cam.ray(s,t);
            
            let mut closest_t = f32::INFINITY;
            let mut color = background;

            for sphere in spheres{
                if let Some(h) = sphere.ray_intersect(pix_ray.origin(), pix_ray.direction(),
                1e-3, closest_t){
                    closest_t = h.t;
                    color = sphere.col;
                }
            }

            film.set_pixel(x, y, color);
        }
    }
}