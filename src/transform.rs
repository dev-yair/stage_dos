use glam::{Vec3, Quat};

#[derive(Debug, Clone, Copy)]
pub struct Transform{
    
    pub translate: Vec3, 
    pub rotate: Quat, 
    pub scale: Vec3

}

impl Transform{

    pub fn new(translate: Vec3, rotate: Quat, scale: Vec3) -> Self{

        Self{translate, rotate, scale}

    }

    

}