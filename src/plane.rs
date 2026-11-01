use glam::Vec3;
use crate::shapes::{Hit, Shape};
use crate::transform::Transform;
use crate::material::Material;

pub struct Plane{

    transform:  Transform, 
    mat:        Material

}

impl Plane{

    pub fn new(transform: Transform, mat: Material) -> Self{

        Self{transform, mat}

    }
}

impl Shape for Plane{


    fn hit(&self, e: Vec3, d: Vec3, t_min: f32, t_max: f32) -> Option<Hit<'_>>{
        
        let a = self.transform.translate;           //point on the plane
        let mut n = self.transform.rotate * Vec3::Y;    //unit normal, pointing up 

        let denom = d.dot(n);
        if denom.abs() < 1e-6 {return None; }       // parallel to the plane

        let t = (a - e).dot(n)  / denom;            // the whole solution 
        if t < t_min || t > t_max {return None; }   //behind us, or too far

        let n = if denom < 0.0 { n } else {-n};     //face the ray

        Some(Hit{t, p: e + t * d, n, mat: &self.mat})

    }


}