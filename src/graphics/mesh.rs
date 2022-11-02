// Hamilton Rice
// Were this a more sophisticated engine, I would separate mesh rendering for to
// support skinned mesh rendering

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

use crate::js_log;
use crate::math::vec3::Vector3;
use crate::math::vec2::Vector2;

#[derive(Default, Clone, Copy)]
pub struct Vertex {
    pos: Vector3,
    nor: Vector3,
    uvs: Vector2
}

impl Vertex {
    pub fn new(pos: Vector3, nor: Vector3, uvs: Vector2) -> Self {
        Self { pos, nor, uvs }
    }

    pub fn new_normailze(pos: Vector3, nor: Vector3, uvs: Vector2) -> Self {
        Self { pos, nor:nor.normalize(), uvs }
    }

    pub fn as_array(self) -> [f32; 8] {
        [self.pos[0], self.pos[1], self.pos[2], self.nor[0], self.nor[1], self.nor[2], self.uvs[0], self.uvs[1]]
        // [self.uvs[1], self.uvs[0], self.nor[2], self.nor[1], self.nor[0], self.pos[2], self.pos[1], self.pos[0]]
    }
}

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
            -0.5, -0.5, -0.5, 1.0, 0.0, 0.0, 0.0, 0.0,
             0.5, -0.5, -0.5, 1.0, 0.0, 0.0, 1.0, 0.0,
             0.5,  0.5, -0.5, 1.0, 0.0, 0.0, 1.0, 1.0,
            -0.5,  0.5, -0.5, 1.0, 0.0, 0.0, 0.0, 1.0,

            -0.5, -0.5,  0.5, 1.0, 0.0, 0.0, 1.0, 1.0,
             0.5, -0.5,  0.5, 1.0, 0.0, 0.0, 0.0, 1.0,
             0.5,  0.5,  0.5, 1.0, 0.0, 0.0, 0.0, 0.0,
            -0.5,  0.5,  0.5, 1.0, 0.0, 0.0, 1.0, 0.0,
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

    pub fn texture_quad() -> Self {
        let verts = vec![
//          x     y     z    norx nory norz u    v
            -0.5, -0.5, -0.5, 0.0, 0.0, 1.0, 0.0, 0.0, // 0
             0.5, -0.5, -0.5, 0.0, 0.0, 1.0, 1.0, 0.0, // 1
             0.5,  0.5, -0.5, 0.0, 0.0, 1.0, 1.0, 1.0, // 2
            -0.5,  0.5, -0.5, 0.0, 0.0, 1.0, 0.0, 1.0  // 3
        ];

        let indices = vec![
            2, 1, 0,
            3, 2, 0,
        ];

        Self { index_size: indices.len(), verts: verts, inds: indices }
    }

    pub fn normal_cube_unit_sphere_face(resolution: u32, direction: Vector3) -> Self {
        js_log("Salutations gamer");
        // Pre allocate the required number of vertices and indices to save time
        // let mut points: Vec<Vertex> = vec![Vertex::default(); (resolution * resolution) as usize];
        let mut points = Vec::with_capacity((resolution * resolution) as usize);
        let mut indices: Vec<u32> = vec![0; (((resolution) * (resolution)) * 6) as usize];
        // let mut indices = Vec::with_capacity(resolution as usize * resolution as usize * 6);
        let mut tri_index = 0;

        for x in 0..(resolution) {
            for y in 0..(resolution) {
                let i = x + resolution * y;
                js_log("percent");
                let percent = Vector2::new(
                    x as f32, 
                    y as f32
                ) / (resolution - 1) as f32;

                js_log("point");
                let point = Vector3::new(
                    percent[0] - 0.5,
                    percent[1] - 0.5,
                    -0.5,
                );

                js_log("adding point to vec");
                // points[i as usize] = Vertex::new(
                //     point, 
                //     point.normalize(), 
                //     percent
                // );

                points.push(Vertex::new(
                    point.normalize(),
                    point.normalize(),
                    percent
                ));

                js_log("indices");
                if x != resolution - 1 && y != resolution - 1 {
                    indices[tri_index as usize] = i;
                    indices[tri_index + 1 as usize] = i + resolution + 1;
                    indices[tri_index + 2 as usize] = i + resolution;

                    indices[tri_index + 3 as usize] = i;
                    indices[tri_index + 4 as usize] = i + 1;
                    indices[tri_index + 5 as usize] = i + resolution + 1;
                    
                    // indices.push(i);
                    // indices.push(i + resolution);
                    // indices.push(i + resolution + 1);
                    // indices.push(i);
                    // indices.push(i + 1);
                    // indices.push(i + resolution);
                    
                    tri_index += 6;
                }
            }
        }
        js_log("Created mesh");

        Self { 
            verts: vec_vertex_to_vec_f32(points),
            index_size: indices.len(), 
            inds: indices 
        }
        // todo!()
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

/// Convert a vector of `Vertex` instances to an array of `f32` values
/// 
/// WebGl expects packed `f32` values, thus `Vertex` cannot be passed in on its own
/// must first be converted into a usable data format
fn vec_vertex_to_vec_f32(vert_vec: Vec<Vertex>) -> Vec<f32> {
    let mut f32_vec = Vec::with_capacity(vert_vec.len() * 8);

    js_log("hello gamer");
    for vert in vert_vec {
        let data = vert.as_array();
        for val in data {
            f32_vec.push(val);
        }
    }
    js_log("goodbye gamer");

    f32_vec
}