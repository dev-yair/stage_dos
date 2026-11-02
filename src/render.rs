use glam::{Vec3};
use crate::camera::Camera;
use crate::film::Film;
use crate::scene::Scene;

pub fn render(cam: &Camera, scene: &Scene, film: &mut Film){

    for y in 0..film.height{
        for x in 0..film.width{
            
            let s = (x as f32 + 0.5) / film.width as f32;
            let t = 1.0 - ( y as f32 + 0.5) / film.height as f32;

            let pix_ray = cam.ray(s,t);

            let maybe_hit = scene.closest_hit(pix_ray.origin(), 
                    pix_ray.direction(), 1e-3, f32::INFINITY);

            let color = match maybe_hit{
                Some(h) => h.mat.diffuse,
                None => scene.background,
            };

            film.set_pixel(x, y, color);
        }
    }
}