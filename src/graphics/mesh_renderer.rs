use web_sys::{WebGlBuffer, WebGl2RenderingContext as GL, WebGlProgram, WebGlVertexArrayObject};

use crate::{common, math::{mat4::Matrix4, vec3::Vector3}, js_log, app_state::{peek_mat_stack, get_canvas_height, get_canvas_width}};

use super::{mesh::Mesh, frag_shaders, vert_shaders, programs::material::Material};

pub struct MeshRenderer {
    mesh: Mesh,
    vertex_buffer: WebGlBuffer,
    index_buffer: WebGlBuffer,
    program: WebGlProgram,
    index_count: i32,
    vao: WebGlVertexArrayObject,
    // mat: dyn Material // TODO Enable this later
}

impl MeshRenderer {
    pub fn new(gl: &GL, mesh: Mesh) -> Self {

        // Create new program
        let vert_shader = common::compile_shader(
            &gl, 
            GL::VERTEX_SHADER,
            r##"#version 300 es

            in vec4 position;
            uniform mat4 projection_matrix;
            uniform mat4 position_matrix;

            out vec4 v_pos;

            void main() {
                gl_Position = projection_matrix * position_matrix * position;
                v_pos = position;
            }
            "##
            // vert_shaders::vert_shader_3d::SHADER
        ).unwrap();

        let frag_shader = common::compile_shader(
            &gl, 
            GL::FRAGMENT_SHADER, 
            frag_shaders::output_test::SHADER
        ).unwrap();

        let program = common::link_program(&gl, &vert_shader, &frag_shader).unwrap();
        gl.use_program(Some(&program));
        
        // Create buffers and attributes
        let vbuffer = gl.create_buffer().unwrap();
        let ibuffer = gl.create_buffer().unwrap();
        let position_attrib_location = gl.get_attrib_location(&program, "position");
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&vbuffer));
        gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&ibuffer));

        unsafe {
            // let vert_array_buf_view = js_sys::Float32Array::view(&mesh.verts);
            let vertex_array_buf_view = js_sys::Float32Array::view(&mesh.verts);
            let index_array_buf_view = js_sys::Uint32Array::view(&mesh.inds);

            gl.buffer_data_with_array_buffer_view(
                GL::ARRAY_BUFFER, 
                &vertex_array_buf_view,
                GL::STATIC_DRAW
            );

            gl.buffer_data_with_array_buffer_view(
                GL::ELEMENT_ARRAY_BUFFER, 
                &index_array_buf_view,
                GL::STATIC_DRAW
            );
        }

        let vao = gl.create_vertex_array()
            .ok_or("Could not create vertex array").unwrap();
        gl.bind_vertex_array(Some(&vao));

        gl.vertex_attrib_pointer_with_i32(
            position_attrib_location as u32,
            3,
            GL::FLOAT,
            false,
            0,
            0
        );
        gl.enable_vertex_attrib_array(position_attrib_location as u32);
        gl.bind_vertex_array(Some(&vao));

        Self {
            index_count: mesh.index_size as i32,
            mesh: mesh,
            vertex_buffer: vbuffer,
            index_buffer: ibuffer,
            program: program,
            vao: vao
        }
    }

    pub fn render(&self, gl: &GL, trs: Matrix4) {

        gl.use_program(Some(&self.program));
        gl.bind_vertex_array(Some(&self.vao));

        let proj_mat_location = gl.get_uniform_location(
            &self.program,
            "projection_matrix"
        ).unwrap(); // Just assume this exists (shrugging lenny face)
        
        
        let transform_mat_location = gl.get_uniform_location(
            &self.program,
            "position_matrix"
        ).unwrap();

        let height = get_canvas_height();
        let width = get_canvas_width();

        // TODO: Get this from application state
        let proj_mat = Matrix4::perspective(
            0.436 * 2.0, 
            width/height, 
            0.1, 
            100.0
        ).transpose();
        let view_mat = Matrix4::view(
            Vector3::new(0.0, 0.0, -5.0), 
            Vector3::up(), 
            // &Vector3::new(2.5, 0.0, 15.5).normalize()
            -Vector3::forward()
        );

        // let view_mat = Matrix4::view_xz(
        //     &Vector3::new(0.0, 1.5, 5.0),
        //     &Vector3::right(),
        //     &-Vector3::new(0.0, 1.5, 5.0).normalize()
        // ).transpose();

        let view_proj_mat = view_mat * proj_mat;

        // let proj_mat = Matrix4::identity();
        gl.uniform_matrix4fv_with_f32_array(
            Some(&proj_mat_location), 
            false, 
            &view_proj_mat.data
        );

        let world_mat = peek_mat_stack();

        // let pos_matrix = Matrix4::translate(0.0, 0.0, -15.0);
        gl.uniform_matrix4fv_with_f32_array(
            Some(&transform_mat_location),
            false,
            &world_mat.data
        );

        // Bind the vertext buffer for the mesh
        gl.bind_buffer(
            GL::ARRAY_BUFFER,
            Some(&self.vertex_buffer)
        );

        // Bind the index buffer for the mesh
        gl.bind_buffer(
            GL::ELEMENT_ARRAY_BUFFER,
            Some(&self.index_buffer)
        );

        gl.draw_elements_with_i32(
            GL::TRIANGLES, 
            self.index_count, 
            GL::UNSIGNED_INT,
            0
        );
    }
}