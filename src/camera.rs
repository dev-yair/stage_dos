use glam::{Mat4, Vec3};
use crate::ray::Ray;

//from the slides in class
pub struct Camera {
    eye: Vec3, // eye of the camera
    u: Vec3, 
    v: Vec3, 
    w: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left: Vec3,

}

//from the slides in class 
impl Camera {
    pub fn new(eye: Vec3, target: Vec3, up: Vec3, vfov_deg: f32, 
        aspect: f32) -> Self{
        
        let theta = vfov_deg.to_radians();
        let h = (theta * 0.5).tan();
        let viewport_h = 2.0 * h;
        let viewport_w = aspect * viewport_h;


        let w = (eye -target).normalize();
        let u = up.cross(w).normalize();
        let v = w.cross(u);

        let horizontal = viewport_w * u;
        let vertical = viewport_h * v;
        let lower_left = eye - horizontal * 0.5 - vertical * 0.5 - w;

        Self {eye, u, v, w, horizontal, vertical, lower_left}
    }

    pub fn ray(&self, s: f32, t: f32) -> Ray {

        let p = self.lower_left + s * self.horizontal + t * self.vertical;

        Ray::new(self.eye, p - self.eye)
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    const EPS: f32 = 1e-5;

    fn assert_vec3_approx(actual: Vec3, expected: Vec3) {
        assert!(
            actual.abs_diff_eq(expected, EPS),
            "expected {expected:?}, got {actual:?}"
        );
    }

    // eye at origin, looking down -z, 90 deg vfov, square aspect:
    // chosen so h = tan(45deg) = 1, giving clean expected numbers.
    fn test_camera() -> Camera {
        Camera::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, -1.0),
            Vec3::new(0.0, 1.0, 0.0),
            90.0,
            1.0,
        )
    }

    #[test]
    fn basis_is_orthonormal() {
        let cam = test_camera();

        assert_vec3_approx(cam.u, cam.u.normalize());
        assert_vec3_approx(cam.v, cam.v.normalize());
        assert_vec3_approx(cam.w, cam.w.normalize());

        assert!(cam.u.dot(cam.v).abs() < EPS);
        assert!(cam.v.dot(cam.w).abs() < EPS);
        assert!(cam.u.dot(cam.w).abs() < EPS);
    }

    #[test]
    fn center_ray_points_at_target() {
        let cam = test_camera();

        let ray = cam.ray(0.5, 0.5);

        assert_vec3_approx(ray.origin(), Vec3::new(0.0, 0.0, 0.0));
        assert_vec3_approx(ray.direction(), Vec3::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn corner_rays_diverge_symmetrically() {
        let cam = test_camera();

        let bottom_left = cam.ray(0.0, 0.0);
        let top_right = cam.ray(1.0, 1.0);

        assert_vec3_approx(bottom_left.direction(), Vec3::new(-1.0, -1.0, -1.0));
        assert_vec3_approx(top_right.direction(), Vec3::new(1.0, 1.0, -1.0));
    }

    #[test]
    fn all_rays_share_the_eye_as_origin() {
        let cam = test_camera();

        for &(s, t) in &[(0.0, 0.0), (0.25, 0.8), (1.0, 1.0)] {
            assert_vec3_approx(cam.ray(s, t).origin(), cam.eye);
        }
    }
}

