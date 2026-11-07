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
mod polymesh;
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
use crate::polymesh::PolyMesh;

use std::f32::consts::PI;

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


    //creating the floor plane
    let tran4 = Transform::new(Vec3::new(0.0, 0.0, 0.0), Quat::IDENTITY, Vec3::splat(3.0));
    let mat4 = Material::lambert(Vec3::new(0.5, 0.5, 0.5));
    scenes.add(Plane::new(tran4, mat4));
    scene2.add(Plane::new(tran4, mat4));
    scene3.add(Plane::new(tran4, mat4));

    //creating the film 
    let mut film = Film::new(1024, 768);

    //creating the mesh
    //making the positions
    let pos = vec![
        Vec3::new(-1.0, 0.0, 1.0),
        Vec3::new(1.0, 0.0, 1.0), 
        Vec3::new(1.0, 0.0, -1.0), 
        Vec3::new(-1.0, 0.0, -1.0),  
        Vec3::new(0.0, 1.5, 0.0)
    ];

    //calculate the normals
    let n0 = (pos[1] - pos[0]).cross(pos[4] - pos[0]).normalize(); //front
    let n1 = (pos[2]- pos[1]).cross(pos[4] - pos[1]).normalize();  //right
    let n2 = (pos[3] - pos[2]).cross(pos[4] - pos[2]).normalize(); //back 
    let n3 = (pos[0]- pos[3]).cross(pos[4] - pos[3]).normalize();  //left

    let norms = vec![n0, n1, n2, n3];

    let pos_index = vec![[0, 1, 4], [1, 2, 4], [2, 3, 4], [3, 0, 4]];
    let norm_index = vec![[0, 0, 0], [1, 1, 1], [2, 2, 2], [3, 3, 3]];

    let mat1 = Material::new(Vec3::new(1.0, 0.0, 0.0), Vec3::splat(0.8), 10000.0);

    let pyramid = PolyMesh::new(pos, norms, pos_index, norm_index, mat1);

    //making a sphere with polymesh to show smooth edges 

    

    let mut sphere_pos: Vec<Vec3> = Vec::new();
    let mut sphere_norms: Vec<Vec3> = Vec::new();

    let center = Vec3::new(2.0, 1.0, 2.0);
    let radius = 1.0;
    let rings  = 8;
    let cols = 12;

    for i in 0..=rings{
        for j  in 0..=cols{
            let theta = PI * (i as f32) / (rings as f32);
            let phi = 2.0 * PI * (j as f32) / (cols as f32);


            let u = Vec3::new(theta.sin() * phi.cos(), theta.cos(), theta.sin() * phi.sin());

            let p = center + radius * u;

            sphere_pos.push(p);
            sphere_norms.push(u);

        }
    }

    let mut sphere_pos_index: Vec<[usize; 3]> = Vec::new();

    for i in 0..rings{
        for j in 0..cols{
            
            let a = i * (cols + 1) + j;
            let b = (i + 1) * (cols + 1) + j;
            let c = i * (cols + 1) + (j + 1);
            let d = (i + 1) * (cols + 1) + (j + 1);

            sphere_pos_index.push([a, c, b]);
            sphere_pos_index.push([c, d, b]);


        }
    }

    let sphere_norm_index = sphere_pos_index.clone();

    //println!("{:?}", &sphere_pos_index[0..2]);

    let mat2 = Material::new(Vec3::new(0.3, 0.5, 0.8), Vec3::splat(0.8), 50.0);

    let smooth_sphere = PolyMesh::new(sphere_pos, sphere_norms, sphere_pos_index, sphere_norm_index, mat2);

    scenes.add(pyramid);
    scenes.add(smooth_sphere);
    //scene2.add(pyramid);
    //scene3.add(pyramid);

    


    //different camera angles to view the mesh from 
    let cam = Camera::new(Vec3::new(1.0, 3.0, 6.0), Vec3::new(0.0, 0.0, -2.5), Vec3::Y, 
    60.0, film.aspect());
    let cam1 = Camera::new(Vec3::new(9.0, 3.5, -3.0), Vec3::new(0.0, 0.0, -2.5), Vec3::Y, 
    60.0, film.aspect());
    let cam2 = Camera::new(Vec3::new(-6.0, 3.5, 4.0), Vec3::new(0.0, 0.0, -2.5), Vec3::Y, 
    60.0, film.aspect());

    //render the images, showing all the hits and ray intersects
    render(&cam, &scenes, &mut film);
    film.save("render1.png");


    render(&cam1, &scenes, &mut film);
    film.save("render2.png");


    render(&cam2, &scenes, &mut film);
    film.save("render3.png")
    
}