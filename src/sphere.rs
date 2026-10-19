use glam::{Vec3};

pub struct Hit{

    pub t: f32, //  Ray parameter at the hit 
    pub p: Vec3, // hit point 
    pub n: Vec3, // unit surface normal at p

}

pub struct Sphere{

    pub r: f32, //r = radius 
    pub c: Vec3, // c = center 
    pub col: Vec3, // col = color

}

impl Sphere{

    pub fn new(r: f32, c:Vec3, col:Vec3) -> Self{

        Self{r, c, col}
    }

    pub fn ray_intersect(&self, e: Vec3, d: Vec3, t_min: f32, t_max: f32) -> Option<Hit>{
        let ec = e - self.c;
        let a = d.dot(d);                               // A
        let b = 2.0 * d.dot(ec);                        // B
        let c = ec.dot(ec) - self.r * self.r;
        let disc = b * b - 4.0 * a * c;
        if disc < 0.0 {return None;}                    // ray has missed
        let sq = disc.sqrt();

        let mut t = (-b - sq) / (2.0 * a);              // near the root
        if t < t_min || t > t_max {
            t = (-b + sq) / (2.0 * a);
            if t < t_min || t > t_max {return None; }
        }

        let p = e + t * d;

        Some(Hit {t, p, n: (p - self.c) / self.r})
    }

    
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPS: f32 = 1e-5;
    const T_MIN: f32 = 1e-4;
    const T_MAX: f32 = f32::INFINITY;

    fn assert_vec3_approx(actual: Vec3, expected: Vec3) {
        assert!(
            actual.abs_diff_eq(expected, EPS),
            "expected {expected:?}, got {actual:?}"
        );
    }

    fn unit_sphere() -> Sphere {
        Sphere::new(1.0, Vec3::ZERO, Vec3::ONE)
    }

    #[test]
    fn new_stores_fields() {
        let s = Sphere::new(2.5, Vec3::new(1.0, 2.0, 3.0), Vec3::new(1.0, 0.0, 0.0));
        assert_eq!(s.r, 2.5);
        assert_eq!(s.c, Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(s.col, Vec3::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn straight_on_hit() {
        // eye at z = 5 looking toward the origin; front of sphere is at z = 1
        let h = unit_sphere()
            .ray_intersect(Vec3::new(0.0, 0.0, 5.0), Vec3::new(0.0, 0.0, -1.0), T_MIN, T_MAX)
            .expect("should hit");
        assert!((h.t - 4.0).abs() < EPS);
        assert_vec3_approx(h.p, Vec3::new(0.0, 0.0, 1.0));
        assert_vec3_approx(h.n, Vec3::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn non_normalized_direction_hits_same_point() {
        // direction has length 2, so t halves but the hit point is the same
        let h = unit_sphere()
            .ray_intersect(Vec3::new(0.0, 0.0, 5.0), Vec3::new(0.0, 0.0, -2.0), T_MIN, T_MAX)
            .expect("should hit");
        assert!((h.t - 2.0).abs() < EPS);
        assert_vec3_approx(h.p, Vec3::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn hit_on_offset_sphere() {
        let s = Sphere::new(2.0, Vec3::new(2.0, 3.0, -4.0), Vec3::ONE);
        // ray from directly in front of the center, aimed at it
        let h = s
            .ray_intersect(Vec3::new(2.0, 3.0, 6.0), Vec3::new(0.0, 0.0, -1.0), T_MIN, T_MAX)
            .expect("should hit");
        assert!((h.t - 8.0).abs() < EPS); // front surface at z = -2
        assert_vec3_approx(h.p, Vec3::new(2.0, 3.0, -2.0));
        assert_vec3_approx(h.n, Vec3::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn clear_miss() {
        // ray passes above the sphere (y = 2 > radius 1)
        let h = unit_sphere()
            .ray_intersect(Vec3::new(0.0, 2.0, 5.0), Vec3::new(0.0, 0.0, -1.0), T_MIN, T_MAX);
        assert!(h.is_none());
    }

    #[test]
    fn tangent_ray_touches_once() {
        // grazes the top of the sphere: discriminant is exactly 0
        let h = unit_sphere()
            .ray_intersect(Vec3::new(0.0, 1.0, 5.0), Vec3::new(0.0, 0.0, -1.0), T_MIN, T_MAX)
            .expect("tangent should count as a hit");
        assert!((h.t - 5.0).abs() < EPS);
        assert_vec3_approx(h.p, Vec3::new(0.0, 1.0, 0.0));
    }

    #[test]
    fn ray_from_inside_uses_far_root() {
        // near root is t = -1 (behind), so the far root t = 1 is returned
        let h = unit_sphere()
            .ray_intersect(Vec3::ZERO, Vec3::new(0.0, 0.0, -1.0), T_MIN, T_MAX)
            .expect("ray from inside should hit the far wall");
        assert!((h.t - 1.0).abs() < EPS);
        assert_vec3_approx(h.p, Vec3::new(0.0, 0.0, -1.0));
        assert_vec3_approx(h.n, Vec3::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn sphere_behind_ray_is_a_miss() {
        // pointing away from the sphere: both roots are negative
        let h = unit_sphere()
            .ray_intersect(Vec3::new(0.0, 0.0, 5.0), Vec3::new(0.0, 0.0, 1.0), T_MIN, T_MAX);
        assert!(h.is_none());
    }

    #[test]
    fn t_max_cuts_off_hit() {
        // both hits (t = 4 and t = 6) are beyond t_max = 3
        let h = unit_sphere()
            .ray_intersect(Vec3::new(0.0, 0.0, 5.0), Vec3::new(0.0, 0.0, -1.0), T_MIN, 3.0);
        assert!(h.is_none());
    }

    #[test]
    fn t_max_between_roots_still_hits_near() {
        // near hit t = 4 is inside [t_min, 5], far hit t = 6 is not
        let h = unit_sphere()
            .ray_intersect(Vec3::new(0.0, 0.0, 5.0), Vec3::new(0.0, 0.0, -1.0), T_MIN, 5.0)
            .expect("near hit is in range");
        assert!((h.t - 4.0).abs() < EPS);
    }

    #[test]
    fn normal_is_unit_length_and_points_outward() {
        let center = Vec3::new(1.0, -1.0, 0.0);
        let s = Sphere::new(3.0, center, Vec3::ONE);
        let h = s
            .ray_intersect(Vec3::new(1.5, -0.7, 10.0), Vec3::new(0.0, 0.0, -1.0), T_MIN, T_MAX)
            .expect("should hit");
        assert!((h.n.length() - 1.0).abs() < EPS);
        // outward: normal points from center toward the hit point
        assert!(h.n.dot(h.p - center) > 0.0);
    }

    #[test]
    fn hit_point_is_on_sphere_surface() {
        let center = Vec3::new(0.5, 0.2, -3.0);
        let s = Sphere::new(1.5, center, Vec3::ONE);
        let h = s
            .ray_intersect(Vec3::new(0.0, 0.0, 5.0), Vec3::new(0.05, 0.02, -1.0), T_MIN, T_MAX)
            .expect("should hit");
        // |p - center| should equal the radius
        assert!(((h.p - center).length() - 1.5).abs() < 1e-4);
    }

    #[test]
    fn closer_sphere_wins_with_shrinking_t_max() {
        // mimics the render loop's depth sort: far sphere listed first
        let far = Sphere::new(1.0, Vec3::new(0.0, 0.0, -5.0), Vec3::new(0.0, 0.0, 1.0));
        let near = Sphere::new(1.0, Vec3::ZERO, Vec3::new(1.0, 0.0, 0.0));
        let spheres = [far, near];

        let e = Vec3::new(0.0, 0.0, 5.0);
        let d = Vec3::new(0.0, 0.0, -1.0);
        let mut closest_t = T_MAX;
        let mut color = Vec3::ZERO;
        for s in &spheres {
            if let Some(h) = s.ray_intersect(e, d, T_MIN, closest_t) {
                closest_t = h.t;
                color = s.col;
            }
        }
        assert!((closest_t - 4.0).abs() < EPS);
        assert_eq!(color, Vec3::new(1.0, 0.0, 0.0)); // near (red) sphere wins
    }
}