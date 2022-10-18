// Hamilton Rice
// I am going to combine rendering and mesh data into one struct for simplicity.
// Were this a more sophisticated engine, I would separate mesh rendering for to
// support skinned mesh rendering


use nalgebra::{Vector3, Vector2};
use web_sys::WebGlBuffer;

// // type aliases (for convenience)
// type Vec3 = Vector3<f32>;
// type Vec2 = Vector2<f32>;

// #[derive(Default, Clone, Copy)]
// pub struct Vertex {
//     // position
//     pos: Vec3,
//     // normal
//     nor: Vec3,
//     // color
//     col: Vec3,
//     // uv
//     uvs: Vec2
// }

// impl Vertex {
//     pub fn new(pos: Vec3, nor: Vec3, col: Vec3, uvs: Vec2) -> Self {
//         Self {
//             pos: pos,
//             nor: nor,
//             col: col,
//             uvs: uvs
//         }
//     }
// }

pub struct Mesh {
    pub verts: Vec<f32>,
    pub inds: Vec<u32>,
    pub index_size: usize,

}

impl Mesh {
    pub fn new() -> Self {
        // Empty
        Self {
            verts: Vec::new(),
            inds: Vec::new(),
            index_size: 0
        }
    }

    pub fn unit_cube() -> Self {
        let verts = vec![
            -0.5, -0.5, -0.5,
             0.5, -0.5, -0.5,
             0.5,  0.5, -0.5,
            -0.5,  0.5, -0.5,

            -0.5, -0.5,  0.5,
             0.5, -0.5,  0.5,
             0.5,  0.5,  0.5,
            -0.5,  0.5,  0.5,
        ];

        let indices = vec![
            
            // Back Face
            2, 1, 0,
            3, 2, 0,

            // Front Face
            4, 5, 6,
            6, 7, 4,

            // Bottom Face
            0, 1, 5,
            5, 4, 0,

            // Top Face
            7, 6, 2,
            2, 3, 7,

            // Right Face
            5, 1, 2,
            2, 6, 5,

            // Left Face
            0, 4, 7,
            7, 3, 0


        ];
        
        Self { index_size: indices.len(), verts: verts, inds: indices, }
    }

    // pub fn normal_cube_unit_sphere_face(resolution: i32) -> Self {
    //     // Create a subdivided cube
    //     // Normalize the points
    //     // return self
    //     // stick everything in one mesh, IDGAF
        
    //     let mut points: Vec<Vertex> = vec![Vertex::default(); (resolution * resolution) as usize];
    //     let mut indices: Vec<i32> = vec![0; resolution as usize];
    //     let mut tri_index = 0;

    //     // Create points grid
    //     for x in 0..(resolution) {
    //         for y in 0..(resolution) {
    //             let i = x * y * resolution;
    //             let percent = Vec2::new(
    //                 x as f32,
    //                 y as f32
    //             ) / (resolution - 1) as f32;

    //             let point = Vec3::new(
    //                 percent.x - 0.5,
    //                 1.0,
    //                 percent.y - 0.5
    //             );

    //             points[i as usize] = Vertex::new(
    //                 point, 
    //                 point, 
    //                 Vec3::new(1.0, 1.0, 1.0), 
    //                 percent
    //             );
                
    //             // do triangles
    //             if x != resolution - 1 && y != resolution - 1 {
    //                 indices[tri_index as usize] = i;
    //                 indices[tri_index + 1 as usize] = i + resolution + 1;
    //                 indices[tri_index + 2 as usize] = i + resolution;

    //                 indices[tri_index + 3 as usize] = i;
    //                 indices[tri_index + 4 as usize] = i + 1;
    //                 indices[tri_index + 5 as usize] = i + resolution;
    //                 tri_index += 6;
    //             }
    //         }
    //     }

    //     // load into buffer

    //     Self {
    //         verts: points,
    //         inds: indices,
    //         indexSize: (tri_index + 1) as i32
    //     }
    // }
}