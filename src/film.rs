use glam::{Vec3};
use image::{RgbImage, Rgb};


pub struct Film{
    pub width: u32,
    pub height: u32, 
    pixels: Vec<Vec3>, 
    
}

impl Film{

    pub fn new(width: u32, height: u32) -> Self {

        let count = (width * height) as usize;
        let pixels = vec![Vec3::ZERO; count];

        Self{width, height, pixels}
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, c:Vec3){

        //making sure that the x and y are withing the proper range of the window
        assert!(x < self.width && y < self.height);
        let i = (y * self.width + x) as usize;

        self.pixels[i] = c;


    }

    pub fn get_pixel(&self, x: u32, y: u32) -> Vec3{

        assert!(x < self.width && y < self.height);
        let i = (y * self.width + x) as usize;

        self.pixels[i]

    }

    pub fn aspect(&self) -> f32{

        self.width as f32 / self.height as f32

    }

    pub fn save(&self, path: &str){

       let mut pic = RgbImage::new(self.width, self.height);

       for y in 0..self.height{
        for x in 0..self.width{
            let col = self.get_pixel(x, y);

            let r = to_u8(col.x);
            let g = to_u8(col.y);
            let b = to_u8(col.z);

            pic.put_pixel(x, y, Rgb([r, g, b]));
        }
       }

       pic.save(path).expect("failed to save PNG");

    }

   
}

fn to_u8(c: f32) -> u8{
        (c.clamp(0.0, 1.0) * 255.0).round() as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_film_is_all_black() {
        let film = Film::new(4, 3);
        for y in 0..3 {
            for x in 0..4 {
                assert_eq!(film.get_pixel(x, y), Vec3::ZERO);
            }
        }
    }

    #[test]
    fn set_then_get_round_trips() {
        let mut film = Film::new(5, 5);
        let red = Vec3::new(1.0, 0.0, 0.0);
        film.set_pixel(4, 3, red);

        assert_eq!(film.get_pixel(4, 3), red);
        // neighbors are untouched
        assert_eq!(film.get_pixel(3, 3), Vec3::ZERO);
        assert_eq!(film.get_pixel(0, 4), Vec3::ZERO);
    }

    #[test]
    fn corners_are_independent() {
        let mut film = Film::new(4, 3);
        let colors = [
            (0, 0, Vec3::new(1.0, 0.0, 0.0)),
            (3, 0, Vec3::new(0.0, 1.0, 0.0)),
            (0, 2, Vec3::new(0.0, 0.0, 1.0)),
            (3, 2, Vec3::new(1.0, 1.0, 1.0)),
        ];
        for &(x, y, c) in &colors {
            film.set_pixel(x, y, c);
        }
        for &(x, y, c) in &colors {
            assert_eq!(film.get_pixel(x, y), c);
        }
    }

    #[test]
    #[should_panic]
    fn get_out_of_bounds_x_panics() {
        let film = Film::new(4, 3);
        film.get_pixel(4, 0); // x == width would otherwise wrap to (0, 1)
    }

    #[test]
    #[should_panic]
    fn set_out_of_bounds_y_panics() {
        let mut film = Film::new(4, 3);
        film.set_pixel(0, 3, Vec3::ONE);
    }

    #[test]
    fn aspect_is_float_ratio() {
        let film = Film::new(1024, 768);
        assert!((film.aspect() - 1024.0 / 768.0).abs() < 1e-6);
    }

    #[test]
    fn to_u8_converts_and_clamps() {
        assert_eq!(to_u8(0.0), 0);
        assert_eq!(to_u8(1.0), 255);
        assert_eq!(to_u8(0.5), 128); // 127.5 rounds up
        assert_eq!(to_u8(-0.3), 0);
        assert_eq!(to_u8(1.7), 255);
    }

    #[test]
    fn save_writes_png_with_correct_pixels() {
        let mut film = Film::new(4, 3);
        film.set_pixel(0, 0, Vec3::new(1.0, 0.0, 0.0)); // top-left red
        film.set_pixel(3, 2, Vec3::new(0.0, 0.0, 1.0)); // bottom-right blue
        film.set_pixel(1, 1, Vec3::new(0.5, 0.5, 0.5)); // gray

        let path = std::path::PathBuf::from("film_test_save.png");
        let path_str = path.to_str().unwrap();
        film.save(path_str);

        let img = image::open(&path).expect("could not reopen PNG").to_rgb8();
        assert_eq!(img.dimensions(), (4, 3));
        assert_eq!(img.get_pixel(0, 0), &Rgb([255, 0, 0]));
        assert_eq!(img.get_pixel(3, 2), &Rgb([0, 0, 255]));
        assert_eq!(img.get_pixel(1, 1), &Rgb([128, 128, 128]));
        assert_eq!(img.get_pixel(2, 0), &Rgb([0, 0, 0])); // untouched stays black

        //std::fs::remove_file(&path).ok();
    }
}