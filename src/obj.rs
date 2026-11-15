use glam::Vec3;
use crate::material::Material;
use crate::polymesh::PolyMesh;

pub fn load_obj(path: &str, mat: Material)  -> PolyMesh{

    let mut positions: Vec<Vec3> = Vec::new();
    let mut normals: Vec<Vec3> = Vec::new();
    let mut pos_index: Vec<[usize; 3]> = Vec::new();
    let mut norm_index: Vec<[usize; 3]> = Vec::new();

    let text = std::fs::read_to_string(path).expect("couldn't read obj file");

    for line in text.lines() {

        let mut parts = line.split_whitespace();

        match parts.next() {
            Some("v") => {let p: Vec<f32> =  parts.map(|s| s.parse().unwrap()).collect();
            positions.push(Vec3::new(p[0], p[1], p[2]));
            }
            Some("f") => {
                let mut pi = [0usize;3]; 
                let mut ni = [0usize;3];
                for (k, token) in parts.take(3).enumerate(){
                    let fields: Vec<&str> =  token.split('/').collect();
                    pi[k] = fields[0].parse::<usize>().unwrap() -1;
                    if let Some(n) = fields.get(2) {
                         ni[k] = n.parse::<usize>().unwrap() - 1;
                    } 
                }
                pos_index.push(pi);
                norm_index.push(ni);
            }
            Some("vn") => {let p: Vec<f32> =  parts.map(|s| s.parse().unwrap()).collect();
            normals.push(Vec3::new(p[0], p[1], p[2]));

            }
            _=> {}      // everything else: ignore
            
        }
    }    

    //case where the .obj file did not have any normals 
    if normals.is_empty() {
        norm_index.clear();
        for (k, &[a, b, c]) in pos_index.iter().enumerate(){

            let u = (positions[b] - positions[a]).cross(positions[c] - positions[a]).normalize();
            normals.push(u);
            norm_index.push([k, k, k]);
        }
    };

    //positions for camera angles  

    let mut low = Vec3::splat(f32::INFINITY);
    let mut high = Vec3::splat(f32::NEG_INFINITY);

    for &p in &positions{
        low = low.min(p);
        high = high.max(p);
    }

    println!("{} positions, {} triangles, box {low} to {high}", positions.len(), pos_index.len());


    
    PolyMesh::new(positions, normals, pos_index, norm_index, mat)

}