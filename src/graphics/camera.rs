use crate::math::{vec3::Vector3, mat4::Matrix4};

pub struct Camera {
    pub position: Vector3,
    pub fov: f32,
    pub near: f32,
    pub far: f32,
    pub view_matrix: Matrix4,
    pub proj_matrix: Matrix4
}

impl Camera {
    pub fn new() -> Self {
        Self { 
            position: Vector3::zero(), 
            fov: 0.25, 
            near: 0.01,
            far: 1000.0,
            view_matrix: Matrix4::identity(),
            proj_matrix: Matrix4::identity()
        }
    }
}