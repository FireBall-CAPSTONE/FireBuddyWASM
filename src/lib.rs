// Hamilton Rice

use common::compile_shader;
use graphics::mesh::Mesh;
use graphics::mesh_renderer::MeshRenderer;
use graphics::{programs::*, frag_shaders, vert_shaders};
use graphics::shader_manager::{ShaderManager, ShaderProgramManager};
use wasm_bindgen::prelude::*;
use web_sys::{WebGl2RenderingContext, WebGlShader};

use crate::graphics::programs::unlit_3d::{UnlitTextured3D, Unlit3D};
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
    root: Node,
    shader_manager: ShaderManager,
    program_manager: ShaderProgramManager
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
        gl.clear_color(0.0, 0.0, 0.0, 1.0);
        
        log("Compiling Shaders");
        let shader_manager = precompile_shaders(&gl);
        let program_manager = link_programs(&gl, &shader_manager);

        log("Creating mesh renderer");

        let r = MeshRenderer::new(
            &gl,
            Mesh::normal_cube_unit_sphere_face(32, Vector3::forward()),
            // Mesh::texture_quad(),
            Box::new(
                UnlitTextured3D::new(
                    &gl, 
                    "/res/world_cube_net_strip1.png",
                    &program_manager
                )
            )
        );

        let r2 = MeshRenderer::new(
            &gl, 
            Mesh::normal_cube_unit_sphere_face(32, Vector3::forward()), 
            // Mesh::texture_quad(),
            Box::new(
                UnlitTextured3D::new(
                    &gl, 
                    "/res/world_cube_net_strip4.png",
                    &program_manager
                )
            )
        );

        let r3 = MeshRenderer::new(
            &gl, 
            Mesh::normal_cube_unit_sphere_face(32, Vector3::forward()), 
            // Mesh::texture_quad(),
            Box::new(
                UnlitTextured3D::new(
                    &gl, 
                    "/res/world_cube_net_strip3.png",
                    &program_manager
                )
            )
        );

        let r4 = MeshRenderer::new(
            &gl, 
            Mesh::normal_cube_unit_sphere_face(32, Vector3::forward()), 
            // Mesh::texture_quad(),
            Box::new(
                UnlitTextured3D::new(
                    &gl, 
                    "/res/world_cube_net_strip2.png",
                    &program_manager
                )
            )
        );

        let r5 = MeshRenderer::new(
            &gl, 
            Mesh::normal_cube_unit_sphere_face(32, Vector3::forward()), 
            // Mesh::texture_quad(),
            Box::new(
                UnlitTextured3D::new(
                    &gl, 
                    "/res/world_cube_net_strip5.png",
                    &program_manager
                )
            )
        );

        let r6 = MeshRenderer::new(
            &gl, 
            Mesh::normal_cube_unit_sphere_face(32, Vector3::forward()), 
            // Mesh::texture_quad(),
            Box::new(
                UnlitTextured3D::new(
                    &gl, 
                    "/res/world_cube_net_strip6.png",
                    &program_manager
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
        
        // let mut base_rot = Node::new();
        // let mut base_pos = Node::new();

        App{
            gl: gl,
            root: root_node,
            shader_manager: shader_manager,
            program_manager: program_manager
        }
    }

    pub fn update(&mut self, delta_time: f32, canvas_height: i32, canvas_width: i32) -> Result<(), JsValue> {
        // Update canvas size
        // update view matrix
        // update 

        // self.root.position -= Vector3::up() * delta_time * 0.2;
        
        app_state::update_dynamic_data(delta_time, canvas_height as f32, canvas_width as f32);

        self.root.scale += Vector3::new(delta_time * 0.05, delta_time * 0.5, delta_time * 0.1);
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

    pub fn add_fireball(&mut self, lat: f32, lon: f32, alt: f32) -> Result<(), JsValue> {
        
        // Create a new node with the rotation

        // Create a new node with the translation, scale, and mesh
        let deg_to_rad = std::f32::consts::PI / 180.0;
        let mut base_rot_lon = Node::new(); // y rot
        let mut base_rot_lat = Node::new(); // x rot

        base_rot_lon.rotation = Quaternion::euler(
            0.0, 
            deg_to_rad * (-lon),
            0.0, 
            // 0.0
        );

        base_rot_lat.rotation = Quaternion::euler(
            deg_to_rad * (-lat + 90.0),
            0.0,
            0.0,
        );

        let mut base_pos = Node::new();
        base_pos.position = Vector3::new(
            0.0,
            1.0 + alt,
            0.0
        );
        
        // TODO: Scale
        base_pos.add_renderer(
            MeshRenderer::new(
                &self.gl,
                Mesh::fireball(),
                Box::new(Unlit3D::new(&self.gl, &self.program_manager))
            )
        );

        base_rot_lat.add_child(base_pos);
        base_rot_lon.add_child(base_rot_lat);
        // base_rot.add_child(base_pos);
        let _ = &self.root.add_child(base_rot_lon);
        
        Ok(())
    }
}

fn precompile_shaders(gl: &WebGl2RenderingContext) -> ShaderManager {
    let mut shader_manager = ShaderManager::new();

    let shader = compile_shader(
        &gl, 
        WebGl2RenderingContext::FRAGMENT_SHADER, 
        frag_shaders::simple_unlit::SHADER
    ).unwrap();

    shader_manager.expose_shader(shader, "frag_simple_unlit");

    let shader = compile_shader(
        &gl, 
        WebGl2RenderingContext::FRAGMENT_SHADER, 
        frag_shaders::simple_unlit_shaded::SHADER
    ).unwrap();

    shader_manager.expose_shader(shader, "frag_simple_unlit_shaded");

    let shader = compile_shader(
        &gl, 
        WebGl2RenderingContext::VERTEX_SHADER, 
        vert_shaders::vert_shader_3d::SHADER
    ).unwrap();

    shader_manager.expose_shader(shader, "vert_3d");

    shader_manager
}

fn link_programs(gl: &WebGl2RenderingContext, shader_manager: &ShaderManager) -> ShaderProgramManager {
    let mut program_manager = ShaderProgramManager::new();

    let program = common::link_program(
        &gl, 
        shader_manager.get_shader("vert_3d"), 
        shader_manager.get_shader("frag_simple_unlit")
    ).unwrap();

    program_manager.expose_program(program, "simple_unlit");

    let program = common::link_program(
        &gl,
        shader_manager.get_shader("vert_3d"),
        shader_manager.get_shader("frag_simple_unlit_shaded")
    ).unwrap();

    program_manager.expose_program(program, "textured_lit");

    program_manager
}

pub fn js_log(msg: &str) {
    // log(msg)
}