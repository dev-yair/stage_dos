use glam::Vec3;
use crate::material::Material;

pub struct Hit<'a>{

    pub t: f32,             // ray parameter at the hit 
    pub p: Vec3,            // hit point = e + t * d    
    pub n: Vec3,            // unit surface normal at p 
    pub mat: &'a Material   // from the shape that gets hit
}


//every shpae answers the same question, the same way

pub trait Shape{

    fn hit(&self, e: Vec3, d: Vec3, 
        t_min: f32, t_max: f32,) -> Option<Hit<'_>>;

}

//None = miss, Some(hit) = hit