use glam::Vec3;
use crate::shapes::Shape;

pub struct Scene{

    shapes: Vec<Box<dyn Shape>>, 
    background: Vec3

}

impl Scene{

    pub fn new(shapes: Vec, background: Vec3) -> Self{

        Self{shapes, bachground}

    }

    pub fn add(&mut self, shape: impl Shape + 'static){
        
       Box::new(shape);

    }

}