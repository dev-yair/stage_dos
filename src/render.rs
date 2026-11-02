use glam::Vec3;
use crate::camera::Camera;
use crate::film::Film;
use crate::scene::Scene;
use crate::shapes::Hit;


pub fn render(cam: &Camera, scene: &Scene, film: &mut Film){

    for y in 0..film.height{
        for x in 0..film.width{
            
            let s = (x as f32 + 0.5) / film.width as f32;
            let t = 1.0 - ( y as f32 + 0.5) / film.height as f32;

            let pix_ray = cam.ray(s,t);

            let maybe_hit = scene.closest_hit(pix_ray.origin(), 
                    pix_ray.direction(), 1e-3, f32::INFINITY);

            let color = match maybe_hit{
                Some(h) => shade_pixel(&h, scene, pix_ray.direction()),
                None => scene.background,
            };

            film.set_pixel(x, y, color);
        }
    }
}

fn shade_pixel(hit: &Hit, scene: &Scene, d: Vec3) -> Vec3{


    // calculating v
    let v = -d.normalize();

    let mut color = hit.mat.diffuse * scene.ambient;

    for light in &scene.lights {

        let direc = (light.position - hit.p);
        let direc_length = direc.length();
        let l = direc.normalize();

        //new work for shadow

        if scene.any_hit(hit.p, l, 1e-3, direc_length){

            continue; 
            
        }

        let diffuse_fac = hit.n.dot(l).max(0.0);

        let h = (v + l).normalize();

        let spec_fac = hit.n.dot(h).max(0.0).powf(hit.mat.shininess);
        
        color += (hit.mat.diffuse * light.intensity * diffuse_fac) + 
            (hit.mat.specular * light.intensity * spec_fac);

        
    }

    color

}