use glam::Vec3;
use crate::shapes::{Hit, Shape};
use crate::material::Material;


pub struct PolyMesh{

    positions: Vec<Vec3>,
    normals: Vec<Vec3>, 
    pos_index: Vec<[usize; 3]>,
    norm_index: Vec<[usize; 3]>,
    mat: Material,

}

impl PolyMesh{

    pub fn new(positions: Vec<Vec3>, normals: Vec<Vec3>, pos_index: Vec<[usize; 3]>, 
        norm_index: Vec<[usize; 3]>, mat: Material) -> Self{
            
            assert_eq!(pos_index.len(), norm_index.len());

             Self{positions, normals, pos_index, norm_index, mat}
    
    }

    pub fn num_tri(&self) -> usize {

        self.pos_index.len()

    }

    pub fn fetch_tri(&self, t: usize) -> ([Vec3; 3], [Vec3; 3]){

        let [a, b, c] = self.pos_index[t];
        let [d, e, f] = self.norm_index[t];

        let out1 = [self.positions[a], self.positions[b], self.positions[c]];
        let out2 = [self.normals[d], self.normals[e], self.normals[f]];

        (out1, out2)


    }
    


}

impl Shape for PolyMesh{

    fn hit(&self, e: Vec3, d: Vec3, t_min: f32, t_max: f32) -> Option<Hit<'_>>{

        let mut closest = t_max;
        let mut best: Option<(f32, f32, f32, usize)> = None;
    
        for t in 0..self.num_tri(){
            let ([a, b, c], [_, _, _]) = self.fetch_tri(t);
            if let Some((th, beta, gamma)) = hit_tri(a, b, c, e, d, t_min, closest){
                closest = th;
                best = Some((th, beta, gamma, t));
            }
        }

        let (th, beta, gamma, tri) = best?;
        let ([_, _, _], [na, nb, nc]) = self.fetch_tri(tri);
        
        let alpha = 1.0 - beta - gamma;
        let mut n = (alpha * na + beta * nb + gamma * nc).normalize();
        if n.dot(d) > 0.0 {n = -n};
        let p = e + th * d;



        Some(Hit{t: th, p, n, mat: &self.mat})
    }
}

fn hit_tri(a: Vec3, b: Vec3, c: Vec3, e: Vec3, d: Vec3, t_min: f32, t_max: f32)
            -> Option<(f32, f32, f32)> {

    let e1 = b - a;
    let e2 = c - a;
    let p = d.cross(e2);
    let det = e1.dot(p);
    if det.abs() < 1e-8 {return None;}
    let inv = 1.0 / det;
    let s = e - a;
    let beta = s.dot(p) * inv;
    if beta < 0.0 || beta > 1.0 {return None;}
    let q = s.cross(e1);
    let gamma = d.dot(q) * inv;
    if gamma < 0.0 || beta + gamma > 1.0 {return None;}
    let t = e2.dot(q) * inv;
    if t < t_min || t > t_max {return None;}

    Some((t, beta, gamma))

}

#[cfg(test)]
mod tests {
    use super::*;

    fn gray() -> Material {
        Material::lambert(Vec3::splat(0.5))
    }

    // Two triangles sharing an edge (a flat square in the y = 0 plane),
    // with one shared "up" normal: the simplest smooth case.
    fn square() -> PolyMesh {
        let positions = vec![
            Vec3::new(0.0, 0.0, 0.0),  // 0
            Vec3::new(1.0, 0.0, 0.0),  // 1
            Vec3::new(1.0, 0.0, -1.0), // 2
            Vec3::new(0.0, 0.0, -1.0), // 3
        ];
        let normals = vec![Vec3::Y];
        let pos_index = vec![[0, 1, 2], [0, 2, 3]];
        let norm_index = vec![[0, 0, 0], [0, 0, 0]];
        PolyMesh::new(positions, normals, pos_index, norm_index, gray())
    }

    // Square pyramid with hard edges: the apex is ONE position
    // but gets a DIFFERENT normal on each of the four sides.
    fn pyramid_sides() -> PolyMesh {
        let positions = vec![
            Vec3::new(-1.0, 0.0, 1.0),  // 0
            Vec3::new(1.0, 0.0, 1.0),   // 1
            Vec3::new(1.0, 0.0, -1.0),  // 2
            Vec3::new(-1.0, 0.0, -1.0), // 3
            Vec3::new(0.0, 1.5, 0.0),   // 4 apex
        ];
        // one normal per side face (approximate; they only need to differ)
        let normals = vec![
            Vec3::new(0.0, 0.55, 0.83),  // 0 front (+z)
            Vec3::new(0.83, 0.55, 0.0),  // 1 right (+x)
            Vec3::new(0.0, 0.55, -0.83), // 2 back  (-z)
            Vec3::new(-0.83, 0.55, 0.0), // 3 left  (-x)
        ];
        let pos_index = vec![[0, 1, 4], [1, 2, 4], [2, 3, 4], [3, 0, 4]];
        let norm_index = vec![[0, 0, 0], [1, 1, 1], [2, 2, 2], [3, 3, 3]];
        PolyMesh::new(positions, normals, pos_index, norm_index, gray())
    }

    #[test]
    fn num_tri_counts_triangles() {
        assert_eq!(square().num_tri(), 2);
        assert_eq!(pyramid_sides().num_tri(), 4);
    }

    #[test]
    fn empty_mesh_has_zero_triangles() {
        let m = PolyMesh::new(Vec::new(), Vec::new(), Vec::new(), Vec::new(), gray());
        assert_eq!(m.num_tri(), 0);
    }

    #[test]
    fn fetch_tri_returns_corners_in_order() {
        let m = square();
        let (p, _) = m.fetch_tri(1);
        assert_eq!(p[0], Vec3::new(0.0, 0.0, 0.0));
        assert_eq!(p[1], Vec3::new(1.0, 0.0, -1.0));
        assert_eq!(p[2], Vec3::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn fetch_tri_returns_matching_normals() {
        let m = square();
        let (_, n) = m.fetch_tri(0);
        assert_eq!(n, [Vec3::Y, Vec3::Y, Vec3::Y]);
    }

    #[test]
    fn shared_edge_returns_same_positions() {
        // triangles 0 and 1 share vertices 0 and 2
        let m = square();
        let (p0, _) = m.fetch_tri(0);
        let (p1, _) = m.fetch_tri(1);
        assert_eq!(p0[0], p1[0]);
        assert_eq!(p0[2], p1[1]);
    }

    #[test]
    fn apex_has_different_normal_on_each_face() {
        // the "one vertex, many normals" requirement from the spec
        let m = pyramid_sides();
        let mut apex_normals = Vec::new();
        for t in 0..m.num_tri() {
            let (p, n) = m.fetch_tri(t);
            assert_eq!(p[2], Vec3::new(0.0, 1.5, 0.0)); // same apex position every time
            apex_normals.push(n[2]);
        }
        for i in 0..apex_normals.len() {
            for j in (i + 1)..apex_normals.len() {
                assert_ne!(apex_normals[i], apex_normals[j]);
            }
        }
    }

    #[test]
    fn every_triangle_can_be_fetched() {
        // iterating 0..num_tri() must never index out of range
        let m = pyramid_sides();
        for t in 0..m.num_tri() {
            let _ = m.fetch_tri(t);
        }
    }

    #[test]
    #[should_panic]
    fn fetch_past_the_end_panics() {
        let m = square();
        m.fetch_tri(2);
    }

    #[test]
    #[should_panic]
    fn mismatched_index_tables_panic() {
        let positions = vec![Vec3::ZERO, Vec3::X, Vec3::Z];
        let normals = vec![Vec3::Y];
        let pos_index = vec![[0, 1, 2], [0, 1, 2]]; // 2 triangles
        let norm_index = vec![[0, 0, 0]];           // only 1
        PolyMesh::new(positions, normals, pos_index, norm_index, gray());
    }

    #[test]
    fn material_is_stored() {
        let m = square();
        assert_eq!(m.mat.diffuse, Vec3::splat(0.5));
    }

        // ---------- hit_tri tests ----------
    // Triangle in the z = -5 plane, facing the camera at the origin:
    //   a = (0,0,-5), b = (2,0,-5), c = (0,2,-5)
    // so a point a + beta*(b-a) + gamma*(c-a) = (2*beta, 2*gamma, -5).
    const EPS: f32 = 1e-5;

    fn tri() -> (Vec3, Vec3, Vec3) {
        (
            Vec3::new(0.0, 0.0, -5.0),
            Vec3::new(2.0, 0.0, -5.0),
            Vec3::new(0.0, 2.0, -5.0),
        )
    }

    fn close(x: f32, y: f32) -> bool {
        (x - y).abs() < EPS
    }

    #[test]
    fn tri_hit_inside_gives_t_and_barycentrics() {
        let (a, b, c) = tri();
        // aim at (0.5, 0.5, -5): beta = 0.25, gamma = 0.25, t = 1 (d reaches it exactly)
        let e = Vec3::ZERO;
        let d = Vec3::new(0.5, 0.5, -5.0);
        let (t, beta, gamma) = hit_tri(a, b, c, e, d, 1e-3, f32::INFINITY).expect("should hit");
        assert!(close(t, 1.0), "t = {t}");
        assert!(close(beta, 0.25), "beta = {beta}");
        assert!(close(gamma, 0.25), "gamma = {gamma}");
    }

    #[test]
    fn tri_hit_point_matches_barycentric_point() {
        // e + t d must equal a + beta (b - a) + gamma (c - a)
        let (a, b, c) = tri();
        let e = Vec3::new(0.3, -0.2, 1.0);
        let d = Vec3::new(0.2, 0.5, -3.0);
        let (t, beta, gamma) = hit_tri(a, b, c, e, d, 1e-3, f32::INFINITY).expect("should hit");
        let ray_p = e + t * d;
        let tri_p = a + beta * (b - a) + gamma * (c - a);
        assert!((ray_p - tri_p).length() < 1e-4);
    }

    #[test]
    fn tri_t_does_not_depend_on_normalizing_d() {
        // same direction, different length -> same hit point and barycentrics
        let (a, b, c) = tri();
        let e = Vec3::ZERO;
        let d = Vec3::new(0.5, 0.5, -5.0);
        let (t1, b1, g1) = hit_tri(a, b, c, e, d, 1e-3, f32::INFINITY).unwrap();
        let (t2, b2, g2) = hit_tri(a, b, c, e, d.normalize(), 1e-3, f32::INFINITY).unwrap();
        assert!(((e + t1 * d) - (e + t2 * d.normalize())).length() < 1e-4);
        assert!(close(b1, b2) && close(g1, g2));
    }

    #[test]
    fn tri_hit_at_corner_a() {
        let (a, b, c) = tri();
        let (_, beta, gamma) =
            hit_tri(a, b, c, Vec3::ZERO, a, 1e-3, f32::INFINITY).expect("corner counts as inside");
        assert!(close(beta, 0.0) && close(gamma, 0.0));
    }

    #[test]
    fn tri_miss_beyond_edge_ab() {
        // gamma < 0: below the triangle (y < 0)
        let (a, b, c) = tri();
        let d = Vec3::new(0.5, -0.5, -5.0);
        assert!(hit_tri(a, b, c, Vec3::ZERO, d, 1e-3, f32::INFINITY).is_none());
    }

    #[test]
    fn tri_miss_beyond_edge_ac() {
        // beta < 0: left of the triangle (x < 0)
        let (a, b, c) = tri();
        let d = Vec3::new(-0.5, 0.5, -5.0);
        assert!(hit_tri(a, b, c, Vec3::ZERO, d, 1e-3, f32::INFINITY).is_none());
    }

    #[test]
    fn tri_miss_beyond_hypotenuse() {
        // beta + gamma > 1: past edge bc, at (1.5, 1.5)
        let (a, b, c) = tri();
        let d = Vec3::new(1.5, 1.5, -5.0);
        assert!(hit_tri(a, b, c, Vec3::ZERO, d, 1e-3, f32::INFINITY).is_none());
    }

    #[test]
    fn tri_miss_parallel_ray() {
        // parallel to the triangle's plane, at z = 0
        let (a, b, c) = tri();
        let d = Vec3::new(1.0, 0.0, 0.0);
        assert!(hit_tri(a, b, c, Vec3::ZERO, d, 1e-3, f32::INFINITY).is_none());
    }

    #[test]
    fn tri_miss_when_behind_ray() {
        // triangle is at z = -5, ray points toward +z: t would be negative
        let (a, b, c) = tri();
        let d = Vec3::new(-0.5, -0.5, 5.0);
        assert!(hit_tri(a, b, c, Vec3::ZERO, d, 1e-3, f32::INFINITY).is_none());
    }

    #[test]
    fn tri_miss_when_farther_than_t_max() {
        // hit is at t = 1, so t_max = 0.5 must reject it (this is what makes closest-hit work)
        let (a, b, c) = tri();
        let d = Vec3::new(0.5, 0.5, -5.0);
        assert!(hit_tri(a, b, c, Vec3::ZERO, d, 1e-3, 0.5).is_none());
    }

    #[test]
    fn tri_hit_from_back_side() {
        // no back-face culling: the backdrop must be hittable from behind
        let (a, b, c) = tri();
        let e = Vec3::new(0.5, 0.5, -10.0);
        let d = Vec3::new(0.0, 0.0, 1.0);
        let (t, _, _) = hit_tri(a, b, c, e, d, 1e-3, f32::INFINITY).expect("back side should hit");
        assert!(close(t, 5.0));
    }

    #[test]
    fn tri_works_in_pyramid_mesh() {
        // straight down near the apex must hit one of the side faces
        let m = pyramid_sides();
        let e = Vec3::new(0.1, 5.0, 0.3);
        let d = Vec3::new(0.0, -1.0, 0.0);
        let mut found = false;
        for t in 0..m.num_tri() {
            let (p, _) = m.fetch_tri(t);
            if hit_tri(p[0], p[1], p[2], e, d, 1e-3, f32::INFINITY).is_some() {
                found = true;
            }
        }
        assert!(found);
    }

        // ---------- PolyMesh::hit (Shape) tests ----------

    // Two parallel triangles facing the camera, one at z = -3 (near) and
    // one at z = -6 (far). The far one is listed FIRST, so a mesh that
    // returned the first hit instead of the nearest would fail.
    fn two_layers() -> PolyMesh {
        let positions = vec![
            Vec3::new(-1.0, -1.0, -6.0), Vec3::new(1.0, -1.0, -6.0), Vec3::new(0.0, 1.0, -6.0), // far
            Vec3::new(-1.0, -1.0, -3.0), Vec3::new(1.0, -1.0, -3.0), Vec3::new(0.0, 1.0, -3.0), // near
        ];
        let normals = vec![Vec3::Z];
        let pos_index = vec![[0, 1, 2], [3, 4, 5]];
        let norm_index = vec![[0, 0, 0], [0, 0, 0]];
        PolyMesh::new(positions, normals, pos_index, norm_index, gray())
    }

    // One triangle at z = -5 whose three vertex normals differ,
    // like a patch of a sphere: a -> +x, b -> +y, c -> +z.
    fn tilted_normals() -> PolyMesh {
        let positions = vec![
            Vec3::new(-1.0, -1.0, -5.0),
            Vec3::new(1.0, -1.0, -5.0),
            Vec3::new(0.0, 1.0, -5.0),
        ];
        let normals = vec![Vec3::X, Vec3::Y, Vec3::Z];
        PolyMesh::new(positions, normals, vec![[0, 1, 2]], vec![[0, 1, 2]], gray())
    }

    #[test]
    fn mesh_hit_returns_nearest_triangle() {
        let m = two_layers();
        let h = m.hit(Vec3::ZERO, Vec3::new(0.0, 0.0, -1.0), 1e-3, f32::INFINITY)
            .expect("should hit");
        assert!(close(h.t, 3.0), "t = {} (6.0 means the far triangle won)", h.t);
        assert!(close(h.p.z, -3.0));
    }

    #[test]
    fn mesh_hit_respects_t_max() {
        // t_max between the two layers: only the near one counts
        let m = two_layers();
        let h = m.hit(Vec3::ZERO, Vec3::new(0.0, 0.0, -1.0), 1e-3, 4.0).expect("should hit");
        assert!(close(h.t, 3.0));
        // t_max before both layers: nothing
        assert!(m.hit(Vec3::ZERO, Vec3::new(0.0, 0.0, -1.0), 1e-3, 2.0).is_none());
    }

    #[test]
    fn mesh_miss_returns_none() {
        let m = two_layers();
        assert!(m.hit(Vec3::ZERO, Vec3::new(0.0, 1.0, 0.0), 1e-3, f32::INFINITY).is_none());
    }

    #[test]
    fn empty_mesh_never_hits() {
        let m = PolyMesh::new(Vec::new(), Vec::new(), Vec::new(), Vec::new(), gray());
        assert!(m.hit(Vec3::ZERO, Vec3::new(0.0, 0.0, -1.0), 1e-3, f32::INFINITY).is_none());
    }

    #[test]
    fn mesh_hit_point_is_on_the_ray() {
        let m = two_layers();
        let e = Vec3::new(0.1, 0.2, 0.0);
        let d = Vec3::new(0.0, -0.1, -2.0); // not normalized, like camera rays
        let h = m.hit(e, d, 1e-3, f32::INFINITY).expect("should hit");
        assert!((h.p - (e + h.t * d)).length() < 1e-5);
        assert!(close(h.p.z, -3.0));
    }

    #[test]
    fn mesh_normal_is_unit_length() {
        let m = tilted_normals();
        let h = m.hit(Vec3::ZERO, Vec3::new(0.1, -0.2, -1.0), 1e-3, f32::INFINITY)
            .expect("should hit");
        assert!(close(h.n.length(), 1.0), "|n| = {}", h.n.length());
    }

    #[test]
    fn mesh_normal_near_a_corner_matches_that_corner() {
        // aim almost exactly at corner c = (0, 1, -5), whose normal is +z
        let m = tilted_normals();
        let d = Vec3::new(0.0, 0.99, -5.0);
        let h = m.hit(Vec3::ZERO, d, 1e-3, f32::INFINITY).expect("should hit");
        assert!(h.n.dot(Vec3::Z) > 0.99, "n = {:?}", h.n);
    }

    #[test]
    fn mesh_normal_is_blended_in_the_middle() {
        // at the centroid alpha = beta = gamma = 1/3, so n = normalize(x + y + z)
        let m = tilted_normals();
        let centroid = Vec3::new(0.0, -1.0 / 3.0, -5.0);
        let h = m.hit(Vec3::ZERO, centroid, 1e-3, f32::INFINITY).expect("should hit");
        let expected = Vec3::new(1.0, 1.0, 1.0).normalize();
        assert!((h.n - expected).length() < 1e-4, "n = {:?}", h.n);
    }

    #[test]
    fn mesh_normal_flips_to_face_the_ray() {
        // flat triangles with normal +z, seen from behind (ray going +z)
        let m = two_layers();
        let e = Vec3::new(0.0, 0.0, -10.0);
        let d = Vec3::new(0.0, 0.0, 1.0);
        let h = m.hit(e, d, 1e-3, f32::INFINITY).expect("back side should hit");
        assert!(close(h.t, 4.0)); // from behind, the z = -6 layer is nearest
        assert!(h.n.dot(d) < 0.0, "normal should face the ray, n = {:?}", h.n);
        assert!(close(h.n.z, -1.0));
    }

    #[test]
    fn mesh_normal_not_flipped_from_front() {
        let m = two_layers();
        let h = m.hit(Vec3::ZERO, Vec3::new(0.0, 0.0, -1.0), 1e-3, f32::INFINITY).unwrap();
        assert!(close(h.n.z, 1.0));
    }

    #[test]
    fn pyramid_faces_shade_flat() {
        // hard edges: two hits on the same face give the same normal
        let m = pyramid_sides();
        let d = Vec3::new(0.0, 0.0, -1.0);
        let h1 = m.hit(Vec3::new(-0.3, 0.3, 5.0), d, 1e-3, f32::INFINITY).expect("front face");
        let h2 = m.hit(Vec3::new(0.4, 0.2, 5.0), d, 1e-3, f32::INFINITY).expect("front face");
        assert!((h1.n - h2.n).length() < 1e-5);
        assert!(h1.n.z > 0.0); // front face, pointing at the camera side
    }

    #[test]
    fn mesh_hit_borrows_its_material() {
        let m = two_layers();
        let h = m.hit(Vec3::ZERO, Vec3::new(0.0, 0.0, -1.0), 1e-3, f32::INFINITY).unwrap();
        assert!(std::ptr::eq(h.mat, &m.mat));
    }

    #[test]
    fn mesh_works_as_boxed_shape() {
        // the way Scene stores it: Vec<Box<dyn Shape>>
        let shapes: Vec<Box<dyn Shape>> = vec![Box::new(two_layers())];
        let h = shapes[0].hit(Vec3::ZERO, Vec3::new(0.0, 0.0, -1.0), 1e-3, f32::INFINITY);
        assert!(h.is_some());
    }
}