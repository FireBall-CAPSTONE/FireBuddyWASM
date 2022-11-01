// Hamilton Rice

use graphics::mesh::Mesh;
use graphics::mesh_renderer::MeshRenderer;
use graphics::programs::*;
use wasm_bindgen::prelude::*;
use web_sys::WebGl2RenderingContext;

use crate::graphics::programs::unlit_3d::UnlitTextured3D;
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
        // TODO: Actually throw an error here instead of just assuming it's going to work

        gl.enable(WebGl2RenderingContext::CULL_FACE); // Cull backfaces
        gl.enable(WebGl2RenderingContext::DEPTH_TEST); // Sort by depth
        gl.cull_face(WebGl2RenderingContext::BACK);
        gl.clear_color(0.0, 0.0, 0.25, 1.0);
        
        log("Creating mesh renderer");

        let r = MeshRenderer::new(
            &gl,
            Mesh::normal_cube_unit_sphere_face(32, Vector3::forward()),
            Box::new(
                UnlitTextured3D::new(
                    &gl, 
                    "https://preview.redd.it/xutn1rchs7161.gif?format=png8&s=7a91c8875d50b402053e5fea9c0f7f747dcd3c9e"
                )
            )
        );

        let r2 = MeshRenderer::new(
            &gl, 
            Mesh::normal_cube_unit_sphere_face(32, Vector3::forward()), 
            Box::new(
                UnlitTextured3D::new(
                    &gl, 
                    "https://preview.redd.it/xutn1rchs7161.gif?format=png8&s=7a91c8875d50b402053e5fea9c0f7f747dcd3c9e"
                )
            )
        );

        let r3 = MeshRenderer::new(
            &gl, 
            Mesh::normal_cube_unit_sphere_face(32, Vector3::forward()), 
            Box::new(
                UnlitTextured3D::new(
                    &gl, 
                    "https://preview.redd.it/xutn1rchs7161.gif?format=png8&s=7a91c8875d50b402053e5fea9c0f7f747dcd3c9e"
                )
            )
        );

        let r4 = MeshRenderer::new(
            &gl, 
            Mesh::normal_cube_unit_sphere_face(32, Vector3::forward()), 
            Box::new(
                UnlitTextured3D::new(
                    &gl, 
                    "https://preview.redd.it/xutn1rchs7161.gif?format=png8&s=7a91c8875d50b402053e5fea9c0f7f747dcd3c9e"
                )
            )
        );

        let r5 = MeshRenderer::new(
            &gl, 
            Mesh::normal_cube_unit_sphere_face(32, Vector3::forward()), 
            Box::new(
                UnlitTextured3D::new(
                    &gl, 
                    "https://preview.redd.it/xutn1rchs7161.gif?format=png8&s=7a91c8875d50b402053e5fea9c0f7f747dcd3c9e"
                )
            )
        );

        let r6 = MeshRenderer::new(
            &gl, 
            Mesh::normal_cube_unit_sphere_face(32, Vector3::forward()), 
            Box::new(
                UnlitTextured3D::new(
                    &gl, 
                    "https://preview.redd.it/xutn1rchs7161.gif?format=png8&s=7a91c8875d50b402053e5fea9c0f7f747dcd3c9e"
                )
            )
        );

        log("Created mesh renderer");
        let deg_to_rad = std::f32::consts::PI / 180.0;

        // TODO: find a better way of doing this
        let mut root_node = Node::new();
        let mut quad_sphere_node = Node::new();
        let mut quad_sphere_node_2 = Node::new();
        let mut quad_sphere_node_3 = Node::new();
        let mut quad_sphere_node_4 = Node::new();
        let mut quad_sphere_node_5 = Node::new();
        let mut quad_sphere_node_6 = Node::new();
        quad_sphere_node.add_renderer(r);
        quad_sphere_node_2.add_renderer(r2);
        quad_sphere_node_3.add_renderer(r3);
        quad_sphere_node_4.add_renderer(r4);
        quad_sphere_node_5.add_renderer(r6);
        quad_sphere_node_6.add_renderer(r5);
        quad_sphere_node_2.rotation = Quaternion::euler(
            0.0, 
            90.0 * deg_to_rad, 
            0.0
        );
        quad_sphere_node_3.rotation = Quaternion::euler(
            0.0, 
            180.0 * deg_to_rad, 
            0.0
        );
        quad_sphere_node_4.rotation = Quaternion::euler(
            0.0, 
            270.0 * deg_to_rad, 
            0.0
        );
        quad_sphere_node_5.rotation = Quaternion::euler(
            270.0 * deg_to_rad, 
            0.0, 
            0.0
        );
        quad_sphere_node_6.rotation = Quaternion::euler(
            90.0 * deg_to_rad, 
            0.0, 
            0.0
        );
        // root_node.add_renderer(r);
        root_node.add_child(quad_sphere_node);
        root_node.add_child(quad_sphere_node_2);
        root_node.add_child(quad_sphere_node_3);
        root_node.add_child(quad_sphere_node_4);
        root_node.add_child(quad_sphere_node_5);
        root_node.add_child(quad_sphere_node_6);

        // for _i in 0..1 {
        //     let child_r = MeshRenderer::new(
        //         &gl,
        //         Mesh::unit_cube()
        //     );

        //     let mut child = Node::new();
        //     child.add_renderer(child_r);
        //     root_node.add_child(child);
        // }

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

        self.root.scale += Vector3::new(delta_time * 0.05, delta_time * 0.05, delta_time * 0.1);
        // self.root.position -= Vector3::new(0.0, 0.0, delta_time * 0.01);
        self.gl.viewport(0, 0, canvas_width, canvas_height);

        self.root.rotation = Quaternion::euler(
            0.0,//self.root.scale[0],
            self.root.scale[1],
            0.0//self.root.scale[2]
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