use glam::{Mat4, Vec3};
use std::fmt;

const EPS: f32 = 1e-5;//will be used for rotate test

pub struct Ray{
    origin: Vec3,
    direction: Vec3
}

impl Ray{
    pub fn new(origin: Vec3, direction: Vec3) -> Self{
        Ray {origin, direction}
    }

    pub fn origin(&self) -> Vec3{
        self.origin
    }

    pub fn direction(&self) -> Vec3{
        self.direction
    }

    pub fn evaluate_at(&self, t: f32) -> Vec3{

        let value = self.origin + t * self.direction;

        value

    }

    pub fn transform(&self, mat: Mat4) -> Ray{

        let new_origin = mat.transform_point3(self.origin);
        let new_direction = mat.transform_vector3(self.direction);
        let new_ray = Ray::new(new_origin, new_direction);

        new_ray
    }
}

impl fmt::Display for Ray{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        write!(f, "({}, {})", self.origin, self.direction)
    }
}

fn main() {

}

#[cfg(test)]
mod tests{
    use super::*;

    const ORIGIN: Vec3 = Vec3::new(0.0, 0.0, 0.0);

    #[test]
    fn creating_and_accessors(){
        let test_origin = ORIGIN;
        let test_direction = Vec3::new(1.0, 4.0, -2.0);

        let test_ray = Ray::new(test_origin, test_direction);

        assert_eq!(test_ray.origin(), test_origin);
        assert_eq!(test_ray.direction(), test_direction);

    }

    #[test]
    fn display(){

        let test_ray = Ray::new(ORIGIN, Vec3::new(1.0, 3.0, 5.0));

        let text = format!("{}", test_ray);

        assert_eq!(text, "([0, 0, 0], [1, 3, 5])");

    }

    #[test]
    fn evaluation1(){
        let test_ray = Ray::new(ORIGIN, Vec3::new(1.0, 3.0, 5.0));

        let t = 3.0;

        let tester = test_ray.evaluate_at(t);

        assert_eq!(tester, Vec3::new(3.0, 9.0, 15.0));

    }

    #[test]
    fn evaluation(){
        let test_ray = Ray::new(ORIGIN, Vec3::new(1.0, 3.0, 5.0));

        let t = -3.0;

        let tester = test_ray.evaluate_at(t);

        assert_eq!(tester, Vec3::new(-3.0, -9.0, -15.0));

    }

    #[test]
    fn translate(){

        let origin = Vec3::new(1.0, 2.0, 3.0);
        let direction = Vec3::new(4.0, -1.0, 2.0);

        let test_ray = Ray::new(origin, direction);
        let mat = Mat4::from_translation(Vec3::new(5.0, -3.0, 2.0));

        let transformed_ray = test_ray.transform(mat);
        let expected_origin = Vec3::new(6.0, -1.0, 5.0);

        assert_eq!(transformed_ray.origin(),  expected_origin);
        assert_eq!(transformed_ray.direction(), direction);

    }

    fn assert_vec3_approx(actual: Vec3, expected: Vec3) {
        assert!(
            actual.abs_diff_eq(expected, EPS),
            "expected {expected:?}, got {actual:?}"
        );
    }

    #[test]
    fn rotate_z_90() {
        let origin = Vec3::new(2.0, 5.0, -3.0); // origin off the axis so it actually moves
        let direction = Vec3::new(1.0, 4.0, 2.0);
        let test_ray = Ray::new(origin, direction);

        let mat = Mat4::from_rotation_z(std::f32::consts::FRAC_PI_2);
        let transformed_ray = test_ray.transform(mat);

        // 90° about z: (x, y, z) -> (-y, x, z)
        assert_vec3_approx(transformed_ray.origin(), Vec3::new(-5.0, 2.0, -3.0));
        assert_vec3_approx(transformed_ray.direction(), Vec3::new(-4.0, 1.0, 2.0));
    }

    #[test]
    fn rotate_full_turn_returns_original() {
        let origin = Vec3::new(2.0, 5.0, -3.0);
        let direction = Vec3::new(1.0, 4.0, 2.0);
        let test_ray = Ray::new(origin, direction);

        let mat = Mat4::from_rotation_z(std::f32::consts::TAU); // 360°
        let transformed_ray = test_ray.transform(mat);

        assert_vec3_approx(transformed_ray.origin(), origin);
        assert_vec3_approx(transformed_ray.direction(), direction);
    }

    #[test]
    fn rotate_then_inverse_round_trips() {
        let origin = Vec3::new(2.0, 5.0, -3.0);
        let direction = Vec3::new(1.0, 4.0, 2.0);
        let test_ray = Ray::new(origin, direction);

        // an arbitrary rotation about a tilted axis
        let axis = Vec3::new(1.0, 1.0, 0.0).normalize();
        let mat = Mat4::from_axis_angle(axis, 0.7);

        let back = test_ray.transform(mat).transform(mat.inverse());

        assert_vec3_approx(back.origin(), origin);
        assert_vec3_approx(back.direction(), direction);
    }

    #[test]
    fn scale(){
        let origin = Vec3::new(4.0, -2.0, 6.0); //origin not at 0,0,0 
        let direction = Vec3::new(3.0, 5.0, -1.0);//random direction

        let test_ray = Ray::new(origin, direction);
        let mat = Mat4::from_scale(Vec3::new(2.0, 0.5, 3.0));

        let transformed_ray = test_ray.transform(mat);
        let expected_origin = Vec3::new(8.0, -1.0, 18.0);
        let expected_direction = Vec3::new(6.0, 2.5, -3.0);

        assert_eq!(transformed_ray.origin(),  expected_origin);
        assert_eq!(transformed_ray.direction(), expected_direction);
    }
}