use std::sync::Arc;

use web_sys::WebGl2RenderingContext as GL;

use crate::{graphics::{mesh::Mesh, mesh_renderer::MeshRenderer}, math::{mat4::Matrix4, vec3::Vector3, quaternion::Quaternion}, app_state::{multiply_to_mat_stack, pop_from_mat_stack}};

#[derive(Default, Clone)]
pub struct Node {
    renderer: Option<Arc<MeshRenderer>>,
    parent: Option<Arc<Node>>,
    pub children: Vec<Arc<Node>>,
    pub position: Vector3,
    pub rotation: Quaternion,
    pub scale: Vector3,
    transform: Matrix4
}

impl Node {
    pub fn new() -> Self {
        Self {
            renderer: None,
            parent: None,
            children: Vec::new(),
            position: Vector3::zero(),
            rotation: Quaternion::euler(0.0, 0.0, 0.0),
            scale: Vector3::zero(),
            transform: Matrix4::identity()
        }
    }

    pub fn add_renderer(&mut self, mesh: MeshRenderer) {
        self.renderer = Some(Arc::new(mesh))
    }

    fn _add_parent(&mut self, parent: Node) {
        self.parent = Some(Arc::new(parent))
    }

    pub fn add_child(&mut self, mut child: Node) {
        
        // This is probably going to create some issues down the line
        // Consider comsuming self and chaining?
        child._add_parent(self.to_owned());

        self.children.push(
            Arc::new(child)
        )
        
        // TODO: Fix
    }

    pub fn render(&self, gl: &GL) {
        // Multiply my matrix onto the mat stack
        let transform = 
            Matrix4::scale_uniform(1.0) * 
            Matrix4::rotate(self.rotation) *
            Matrix4::translate(self.position[0], self.position[1], self.position[2]);

        multiply_to_mat_stack(transform);

        

        for child in self.to_owned().children {
            child.render(&gl);
        }

        if self.renderer.is_some() {
            // Very good not at all stupid or bad
            self.renderer.as_ref().unwrap().render(&gl, transform)
        }

        // Pop my matrix from the mat stack
        pop_from_mat_stack();
    }
}