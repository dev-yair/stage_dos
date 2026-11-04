mod ray;
mod camera;
mod film;
mod sphere;
mod render;
mod material; 
mod transform;
mod shapes;
mod plane;
mod scene;
mod light;
use glam::{Vec3, Quat};
use crate::camera::Camera;
use crate::film::Film;
use crate::sphere::Sphere;
use crate::render::render;
use crate::material::Material;
use crate::transform::Transform;
use crate::plane::Plane;
use crate::scene::Scene;
use crate::light::Light;

fn main() {
   
    //setting the color of the background 
    let background = Vec3::new(0.2,0.2,0.2);

    //creating the scene based on our background
    let mut scenes = Scene::new(background, Vec3::splat(0.1));
    let mut scene2 = Scene::new(background, Vec3::splat(0.2));
    let mut scene3 = Scene::new(background, Vec3::splat(0.15));
    
    //creating the light or lights that will be used in the image
    let light1 = Light::new(Vec3::new(5.0, 5.0, -2.0), Vec3::splat(0.6));
    let light2 = Light::new(Vec3::new(-5.0, 13.0, 2.0), Vec3::splat(0.4));
    let light3 = Light::new(Vec3::new(2.5, 8.0, 3.0), Vec3::splat(0.5));

    scenes.add_lights(light1);
    scenes.add_lights(light2);

    scene2.add_lights(light3);
    scene2.add_lights(light1);

    scene3.add_lights(light2);
    scene3.add_lights(light3);


    //making the spherers that will be used in the image

    //setting the vlaues for the new sphers
    let tran1 = Transform::new(Vec3::new(-1.0, 0.5, -3.0), Quat::IDENTITY, Vec3::splat(0.6));
    let tran2 = Transform::new(Vec3::new(1.2, 1.0, -2.5), Quat::IDENTITY, Vec3::splat(1.0));
    let tran3 = Transform::new(Vec3::new(0.0,1.0,0.0), Quat::IDENTITY, Vec3::splat(1.0));
    let tran5 = Transform::new(Vec3::new(1.0, 1.0, 1.0), Quat::IDENTITY, Vec3::splat(1.0));
    let mat1 = Material::new(Vec3::new(1.0, 0.0, 0.0), Vec3::splat(0.8), 30.0);
    let mat2 = Material::new(Vec3::new(0.0, 1.0, 0.0), Vec3::ZERO, 1.0);
    let mat3 = Material::new(Vec3::new(0.0, 0.0, 1.0), Vec3::ONE, 50.0);
    let mat5 = Material::new(Vec3::new(0.4, 0.2, 0.6), Vec3::ONE, 10000.0);

    scenes.add(Sphere::new(tran1, mat1)); //red sphere
    scenes.add(Sphere::new(tran2, mat2)); //green sphere
    scenes.add(Sphere::new(tran3, mat3)); //blue sphere
    scenes.add(Sphere::new(tran5, mat5)); //purple sphere


    //creating the floor plane
    let tran4 = Transform::new(Vec3::new(0.0, 0.0, 0.0), Quat::IDENTITY, Vec3::splat(3.0));
    let mat4 = Material::lambert(Vec3::new(0.5, 0.5, 0.5));
    scenes.add(Plane::new(tran4, mat4));
    scene2.add(Plane::new(tran4, mat4));
    scene3.add(Plane::new(tran4, mat4));

    //creating the film 
    let mut film = Film::new(1024, 768);

    //different camera angles to view the spheres from 
    let cam = Camera::new(Vec3::new(1.0, 7.0, 6.0), Vec3::new(0.0, 0.0, -2.5), Vec3::Y, 
    60.0, film.aspect());
    let cam1 = Camera::new(Vec3::new(9.0, 6.5, -3.0), Vec3::new(0.0, 0.0, -2.5), Vec3::Y, 
    60.0, film.aspect());
    let cam2 = Camera::new(Vec3::new(-6.0, 7.5, 4.0), Vec3::new(0.0, 0.0, -2.5), Vec3::Y, 
    60.0, film.aspect());

    //render the images, showing all the hits and ray intersects
    render(&cam, &scenes, &mut film);
    film.save("render1.png");

    let tran6 = Transform::new(Vec3::new(-3.0, 1.2, -2.0), Quat::IDENTITY, Vec3::splat(1.2));
    let tran7 = Transform::new(Vec3::new(2.0, 1.0, 2.5), Quat::IDENTITY, Vec3::splat(1.0));
    let tran8 = Transform::new(Vec3::new(0.0,1.0,0.0), Quat::IDENTITY, Vec3::splat(1.0));
    let tran9 = Transform::new(Vec3::new(1.5, 1.3, -2.5), Quat::IDENTITY, Vec3::splat(1.5));
    let mat6 = Material::new(Vec3::new(1.0, 0.0, 0.0), Vec3::splat(0.8), 100.0);
    let mat7 = Material::new(Vec3::new(0.0, 1.0, 0.0), Vec3::ZERO, 1.0);
    let mat8 = Material::new(Vec3::new(0.0, 0.0, 1.0), Vec3::ZERO, 1.0);
    let mat9 = Material::new(Vec3::new(0.7, 0.6, 0.2), Vec3::new(1.0, 0.85, 0.45), 10000.0);

    scene2.add(Sphere::new(tran6, mat6));
    scene2.add(Sphere::new(tran7, mat7));
    scene2.add(Sphere::new(tran8, mat8));
    scene2.add(Sphere::new(tran9, mat9));

    render(&cam1, &scene2, &mut film);
    film.save("render2.png");

    let tran10 = Transform::new(Vec3::new(1.0, 0.5, 1.0), Quat::IDENTITY, Vec3::splat(0.5));
    let tran11 = Transform::new(Vec3::new(-2.0, 1.0, 2.5), Quat::IDENTITY, Vec3::splat(1.0));
    let tran12 = Transform::new(Vec3::new(0.0,1.0,0.0), Quat::IDENTITY, Vec3::splat(1.0));
    let tran13 = Transform::new(Vec3::new(-1.5, 1.5, -1.5), Quat::IDENTITY, Vec3::splat(1.5));
    let mat10 = Material::new(Vec3::new(0.3, 0.3, 3.0), Vec3::new(1.0, 0.85, 0.45), 20.0);
    let mat11 = Material::new(Vec3::new(0.3, 0.3, 3.0), Vec3::ZERO, 1.0);
    let mat12 = Material::new(Vec3::new(0.3, 0.3, 3.0), Vec3::ONE, 60.0);
    let mat13 = Material::new(Vec3::new(0.3, 0.3, 3.0), Vec3::ONE, 10000.0);

    scene3.add(Sphere::new(tran10, mat10));
    scene3.add(Sphere::new(tran11, mat11));
    scene3.add(Sphere::new(tran12, mat12));
    scene3.add(Sphere::new(tran13, mat13));


    render(&cam2, &scene3, &mut film);
    film.save("render3.png")
    
}