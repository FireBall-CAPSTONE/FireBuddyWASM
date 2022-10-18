// Hamilton Rice

use graphics::mesh::Mesh;
use graphics::mesh_renderer::MeshRenderer;
use wasm_bindgen::prelude::*;
use web_sys::WebGl2RenderingContext;

use crate::math::quaternion::Quaternion;
use crate::math::vec3::Vector3;
use crate::scene::scene_node::Node;

mod app_state;
mod common;
mod graphics;
mod scene;
mod math;
mod tests;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct App {
    gl: WebGl2RenderingContext,
    // vertex_buffer: WebGlBuffer,
    // vertex_array: WebGlVertexArrayObject,
    // program: WebGlProgram,
    // vert_count: i32,
    root: Node
}

#[wasm_bindgen]
impl App {
    #[wasm_bindgen(constructor)]
    pub fn new(element_id: &str) -> Self {

        let gl = common::get_gl_context(element_id).unwrap();
        
        gl.enable(WebGl2RenderingContext::CULL_FACE); // Cull backfaces
        gl.enable(WebGl2RenderingContext::DEPTH_TEST); // Sort by depth
        gl.cull_face(WebGl2RenderingContext::BACK);
        gl.clear_color(0.0, 0.0, 0.25, 1.0);

        let r = MeshRenderer::new(
            &gl,
            Mesh::unit_cube()
        );

        let mut root_node = Node::new();
        root_node.add_renderer(r);

        for _i in 0..1 {
            let child_r = MeshRenderer::new(
                &gl,
                Mesh::unit_cube()
            );

            let mut child = Node::new();
            child.add_renderer(child_r);
            root_node.add_child(child);
        }

        App{
            gl: gl,
            root: root_node
        }
    }

    pub fn update(&mut self, delta_time: f32) -> Result<(), JsValue> {
        // Update canvas size
        // update view matrix
        // update 

        // self.root.position -= Vector3::up() * delta_time * 0.2;

        self.root.scale += Vector3::new(delta_time * 0.05, delta_time * 0.5, delta_time * 0.1);

        self.root.rotation = Quaternion::euler(
            0.35,
            self.root.scale[1],
            0.0
        );

        // self.root.position = Vector3::new(0.0, (self.root.scale[1] as f32).sin(), 0.0);

        Ok(())
    }

    pub fn render(&self) -> Result<(), JsValue> {
        // traverse the scene tree
        // multiply onto mat stack

        // draw(&self.gl, self.vert_count);
        // log("Rendering");
        // self.gl.clear_depth(-1.0);
        self.gl.clear(WebGl2RenderingContext::DEPTH_BUFFER_BIT);
        self.gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
        self.root.render(&self.gl);
        Ok(())
    }
}

// pub fn draw(context: &WebGl2RenderingContext, vert_count: i32, ind_buff: WebGlBuffer) {
//     context.clear_color(0.0, 0.05, 0.1, 1.0);
//     context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
//     // context.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, vert_count);
//     context.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, Some(&ind_buff));
//     context.draw_elements_with_i32(
//         WebGl2RenderingContext::TRIANGLES,
//         12,
//         WebGl2RenderingContext::UNSIGNED_INT,
//         0
//     );
// }

pub fn js_log(msg: &str) {
    unsafe { log(msg) }
}