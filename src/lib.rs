// Hamilton Rice

use graphics::mesh::Mesh;
use graphics::mesh_renderer::MeshRenderer;
use graphics::programs::*;
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

    pub fn update(&mut self, delta_time: f32, canvas_height: i32, canvas_width: i32) -> Result<(), JsValue> {
        // Update canvas size
        // update view matrix
        // update 

        // self.root.position -= Vector3::up() * delta_time * 0.2;
        
        app_state::update_dynamic_data(delta_time, canvas_height as f32, canvas_width as f32);

        self.root.scale += Vector3::new(delta_time * 0.05, delta_time * 0.5, delta_time * 0.1);
        self.gl.viewport(0, 0, canvas_width, canvas_height);

        self.root.rotation = Quaternion::euler(
            self.root.scale[0],
            self.root.scale[1],
            self.root.scale[2]
        );

        // self.root.position = Vector3::new(0.0, (self.root.scale[1] as f32).sin(), 0.0);

        Ok(())
    }

    pub fn render(&self) -> Result<(), JsValue> {
        self.gl.clear(WebGl2RenderingContext::DEPTH_BUFFER_BIT);
        self.gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
        self.root.render(&self.gl);
        Ok(())
    }
}

pub fn js_log(msg: &str) {
    unsafe { log(msg) }
}