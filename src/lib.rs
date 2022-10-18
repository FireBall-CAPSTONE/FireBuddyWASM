// Starter code from:
// https://rustwasm.github.io/wasm-bindgen/examples/webgl.html
//
// Hamilton Rice

use std::panic::RefUnwindSafe;

use graphics::frag_shaders;
use graphics::mesh::Mesh;
use graphics::mesh_renderer::MeshRenderer;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;
use web_sys::WebGlBuffer;
use web_sys::WebGlVertexArrayObject;
use web_sys::{ WebGl2RenderingContext, WebGlProgram, WebGlShader };

use crate::math::mat4::Matrix4;
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
        
        // gl.disable(WebGl2RenderingContext::CULL_FACE);
        gl.enable(WebGl2RenderingContext::CULL_FACE); // Cull backfaces
        gl.enable(WebGl2RenderingContext::DEPTH_TEST); // Sort by depth
        // gl.depth_func(WebGl2RenderingContext::GREATER);
        gl.cull_face(WebGl2RenderingContext::BACK);
        gl.clear_color(0.0, 0.0, 0.25, 1.0);

        let r = MeshRenderer::new(
            &gl,
            Mesh::unit_cube()
        );

        // let child_r = MeshRenderer::new(
        //     &gl,
        //     Mesh::unit_cube()
        // );

        // let child2_r = MeshRenderer::new(
        //     &gl,
        //     Mesh::unit_cube()
        // );

        let mut root_node = Node::new();
        root_node.add_renderer(r);

        // let mut child = Node::new();
        // child.add_renderer(child_r);
        // child.position = Vector3::new(1.5, 0.0, 0.0);
        // child.rotation = Quaternion::euler(0.0, 3.14, 0.0);

        // let mut child2 = Node::new();
        // child2.add_renderer(child2_r);
        // child2.position = Vector3::new(0.0, 1.1, 0.5);

        // child.add_child(child2);

        // root_node.add_child(child);

        for _i in 0..1 {
            let child_r = MeshRenderer::new(
                &gl,
                Mesh::unit_cube()
            );

            let mut child = Node::new();
            child.add_renderer(child_r);
            root_node.add_child(child);
        }



        // let vert_shader = common::compile_shader(&gl,
        //     WebGl2RenderingContext::VERTEX_SHADER, 
        //     r##"#version 300 es

        //     in vec4 position;

        //     out vec4 v_pos;

        //     void main() {
        //         gl_Position = position;
        //         v_pos = position;
        //     }
        //     "##
        // ).unwrap();

        // let frag_shader = common::compile_shader(
        //     &gl, 
        //     WebGl2RenderingContext::FRAGMENT_SHADER, 
        //     frag_shaders::output_test::SHADER
        // ).unwrap();

        
        // let program = common::link_program(&gl, &vert_shader, &frag_shader).unwrap();
        // gl.use_program(Some(&program));

        // // let vertices: Vec<f32> = vec![
        // //     -0.7, -0.7, 0.0,
        // //     0.7, -0.7, 0.0,
        // //     0.7, 0.7, 0.0,
        // //     -0.7, -0.7, 0.0,
        // //     0.7, 0.7, 0.0,
        // //     -0.7, 0.7, 0.0
        // // ];

        // let mesh = Mesh::unit_cube();

        // let vertices = mesh.verts;
        // let indices = mesh.inds;

        // let position_attribute_location = gl.get_attrib_location(&program, "position");
        // let vertex_buffer = gl.create_buffer().ok_or("Failed to create buffer").unwrap();
        // let index_buffer = gl.create_buffer().ok_or("Failed to create index buffer").unwrap();
        // gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&vertex_buffer));
        // gl.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, Some(&index_buffer));

        // unsafe {
        //     let positions_array_buf_view = js_sys::Float32Array::view(&vertices);
        //     let indices_array_buf_view = js_sys::Uint32Array::view(&indices);

        //     gl.buffer_data_with_array_buffer_view(
        //         WebGl2RenderingContext::ARRAY_BUFFER,
        //         &positions_array_buf_view, 
        //         WebGl2RenderingContext::STATIC_DRAW
        //     );

        //     gl.buffer_data_with_array_buffer_view(
        //         WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
        //         &indices_array_buf_view, 
        //         WebGl2RenderingContext::STATIC_DRAW
        //     );
        // }

        // let vao = gl.create_vertex_array()
        //     .ok_or("Could not create vertex array").unwrap();
        // gl.bind_vertex_array(Some(&vao));

        // gl.vertex_attrib_pointer_with_i32(
        //     position_attribute_location as u32,
        //     3,
        //     WebGl2RenderingContext::FLOAT,
        //     false,
        //     0,
        //     0
        // );
        // gl.enable_vertex_attrib_array(position_attribute_location as u32);

        // gl.bind_vertex_array(Some(&vao));

        // let vert_count = (vertices.len() / 3) as i32;
        // draw(&gl, vert_count, index_buffer);

        App{
            gl: gl,
            // program: program,
            // vertex_array: vao,
            // vertex_buffer: vertex_buffer,
            // vert_count: vert_count
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

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    // let document = web_sys::window().unwrap().document().unwrap();
    // let canvas = document.get_element_by_id("canvas").unwrap();
    // let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

    // let context = canvas
    //     .get_context("webgl2")?
    //     .unwrap()
    //     .dyn_into::<WebGl2RenderingContext>()?;
    
    // let vert_shader = common::compile_shader(
    //     &context,
    //     WebGl2RenderingContext::VERTEX_SHADER,
    //     r##"#version 300 es

    //     in vec4 position;

    //     out vec4 v_pos;

    //     void main() {
    //         gl_Position = position;
    //         v_pos = position;
    //     }
    //     "##
    // )?;

    // let frag_shader = common::compile_shader(
    //     &context, 
    //     WebGl2RenderingContext::FRAGMENT_SHADER,
    //     frag_shaders::output_test::SHADER
    // )?;

    // let program = common::link_program(&context, &vert_shader, &frag_shader)?;
    // context.use_program(Some(&program));

    

    // let vertices: Vec<f32> = vec![
    //     -0.7, -0.7, 0.0,
    //     0.7, -0.7, 0.0,
    //     0.7, 0.7, 0.0,
    //     -0.7, -0.7, 0.0,
    //     0.7, 0.7, 0.0,
    //     -0.7, 0.7, 0.0
    // ];

    // let position_attribute_location= context.get_attrib_location(&program, "position");
    // let buffer = context.create_buffer().ok_or("Failed to create buffer")?;
    // context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));

    // unsafe {
    //     let positions_array_buf_view = js_sys::Float32Array::view(&vertices);

    //     context.buffer_data_with_array_buffer_view(
    //         WebGl2RenderingContext::ARRAY_BUFFER,
    //         &positions_array_buf_view,
    //         WebGl2RenderingContext::STATIC_DRAW
    //     );
    // }

    // let vao = context
    //     .create_vertex_array()
    //     .ok_or("Could not create vertext array!")?;
    // context.bind_vertex_array(Some(&vao));

    // context.vertex_attrib_pointer_with_i32(
    //     position_attribute_location as u32, 
    //     3, 
    //     WebGl2RenderingContext::FLOAT, 
    //     false, 
    //     0, 
    //     0
    // );
    // context.enable_vertex_attrib_array(position_attribute_location as u32);

    // context.bind_vertex_array(Some(&vao));

    // let vert_count = (vertices.len() / 3) as i32;
    // draw(&context, vert_count);

    Ok(())
    
}

pub fn draw(context: &WebGl2RenderingContext, vert_count: i32, ind_buff: WebGlBuffer) {
    context.clear_color(0.0, 0.05, 0.1, 1.0);
    context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
    // context.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, vert_count);
    context.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, Some(&ind_buff));
    context.draw_elements_with_i32(
        WebGl2RenderingContext::TRIANGLES,
        12,
        WebGl2RenderingContext::UNSIGNED_INT,
        0
    );
}

pub fn js_log(msg: &str) {
    unsafe { log(msg) }
}