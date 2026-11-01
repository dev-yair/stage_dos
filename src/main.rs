mod ray;
mod camera;
mod film;
mod sphere;
mod render;
mod material; 
mod transform;
mod shapes;
mod plane;
use glam::{Vec3, Quat};
use crate::camera::Camera;
use crate::film::Film;
use crate::sphere::Sphere;
use crate::render::render;
use crate::material::Material;
use crate::transform::Transform;
use crate::plane::Plane;

fn main() {
   
    //setting the color of the background 
    let background = Vec3::new(50.0,50.0,50.0);
    
    //making the spherers that will be used in the image

    //setting the vlaues for the new sphers
    let tran1 = Transform::new(Vec3::new(-1.0, -0.2, -5.0), Quat::IDENTITY, Vec3::splat(1.2));
    let tran2 = Transform::new(Vec3::new(1.2, 0.3, -2.5), Quat::IDENTITY, Vec3::splat(1.0));
    let tran3 = Transform::new(Vec3::new(0.0,0.0,0.0), Quat::IDENTITY, Vec3::splat(1.0));
    let mat1 = Material::lambert(Vec3::new(1.0, 0.0, 0.0));
    let mat2 = Material::lambert(Vec3::new(0.0, 1.0, 0.0));
    let mat3 = Material::lambert(Vec3::new(0.0, 0.0, 1.0));

    let spheres = vec![Sphere::new(tran1, mat1), 
    Sphere::new(tran2, mat2),Sphere::new(tran3, mat3)];

    //creating the film 
    let mut film = Film::new(1024, 786);

    //different camera angles to view the spheres from 
    let mut cam = Camera::new(Vec3::new(0.0, 1.0, 6.0), Vec3::new(0.0, 0.0, -2.5), Vec3::Y, 
    60.0, film.aspect());
    let mut cam1 = Camera::new(Vec3::new(0.0, 3.5, 4.0), Vec3::new(0.0, 0.0, -2.5), Vec3::Y, 
    60.0, film.aspect());
    let mut cam2 = Camera::new(Vec3::new(9.0, 1.0, 2.0), Vec3::new(0.0, 0.0, -2.5), Vec3::Y, 
    60.0, film.aspect());

    //render the images, showing all the hits and ray intersects
    render(&cam, &spheres, &mut film, background);
    film.save("render1.png");
    render(&cam1, &spheres, &mut film, background);
    film.save("render2.png");
    render(&cam2, &spheres, &mut film, background);
    film.save("render3.png")
    
}