use glam::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Material{

    pub diffuse: Vec3, 
    pub specular: Vec3, 
    pub shininess: f32              //this is the phong exponent

}

impl Material{

    pub fn new(diffuse: Vec3, specular: Vec3, shininess: f32) -> Self{
        
        Self{diffuse, specular, shininess}  
    }

    pub fn lambert(diffuse: Vec3) -> Self{

        Self::new(diffuse, Vec3::ZERO, 1.0)

    }


}