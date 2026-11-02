use glam::Vec3;
use crate::shapes::{Hit, Shape};
use crate::light::Light;

pub struct Scene{

    shapes: Vec<Box<dyn Shape>>, 
    pub lights: Vec<Light>,
    pub ambient: Vec3, 
    pub background: Vec3


}

impl Scene{

    pub fn new(background: Vec3, ambient: Vec3) -> Self{

        Self{shapes: Vec::new(), lights: Vec::new(), background, ambient}

    }

    pub fn add(&mut self, shape: impl Shape + 'static){

       self.shapes.push(Box::new(shape));

    }

    pub fn add_lights(&mut self, light: Light){
        
        self.lights.push(light);

    }

    pub fn closest_hit(&self, e: Vec3, d: Vec3, t_min: f32, t_max: f32,) -> Option<Hit<'_>>{

        let mut closest = t_max; 
        let mut best : Option<Hit> = None;

        for shape in &self.shapes{
            // shape is a &Box<dyn Shape> 

            if let Some(h) = shape.hit(e, d, t_min, closest){

                closest = h.t;
                best = Some(h);

            }
        }

        best

    }

    pub fn any_hit(&self, e: Vec3, d:Vec3, t_min: f32, t_max: f32) -> bool{


        for shape in &self.shapes{
            
            if shape.hit(e, d, t_min, t_max).is_some(){
                return true;
            }

        }

        false
    }

}