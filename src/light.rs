use glam::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Light{

    pub position : Vec3, 
    pub intensity: Vec3
}

impl Light{

    pub fn new(position: Vec3, intensity: Vec3) -> Self{

        Self{position, intensity}
    }


}