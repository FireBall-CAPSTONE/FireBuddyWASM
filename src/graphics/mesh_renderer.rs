use web_sys::{WebGlBuffer, WebGl2RenderingContext as GL, WebGlVertexArrayObject};
use super::{mesh::Mesh, programs::material::Material};

pub struct MeshRenderer {
    vertex_buffer: WebGlBuffer,
    index_buffer: WebGlBuffer,
    // program: WebGlProgram,
    index_count: i32,
    vao: WebGlVertexArrayObject,
    mat: Box<dyn Material> // TODO Enable this later
}

impl MeshRenderer {
    pub fn new(gl: &GL, mesh: Mesh, mat: Box<dyn Material>) -> Self {
        
        // Create buffers and attributes
        let program = mat.get_program();
        let vbuffer = gl.create_buffer().unwrap();
        let ibuffer = gl.create_buffer().unwrap();
        let position_attrib_location = gl.get_attrib_location(&program, "vertex_position");
        let normal_attrib_location = gl.get_attrib_location(&program, "vertex_normal");
        let uv_attrib_location = gl.get_attrib_location(&program, "vertex_uv_coords");
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&vbuffer));
        gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&ibuffer));

        unsafe {
            // let vert_array_buf_view = js_sys::Float32Array::view(&mesh.verts);
            let vertex_array_buf_view = js_sys::Float32Array::view(&mesh.verts);
            gl.buffer_data_with_array_buffer_view(
                GL::ARRAY_BUFFER, 
                &vertex_array_buf_view,
                GL::STATIC_DRAW
            );

            let index_array_buf_view = js_sys::Uint32Array::view(&mesh.inds);
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
            32,
            0
        );

        gl.vertex_attrib_pointer_with_i32(
            normal_attrib_location as u32, 
            3, 
            GL::FLOAT, 
            false, 
            32, 
            12
        );
        
        gl.vertex_attrib_pointer_with_i32(
            uv_attrib_location as u32,
            2,
            GL::FLOAT,
            false,
            32,
            24
        );

        gl.enable_vertex_attrib_array(position_attrib_location as u32);
        gl.enable_vertex_attrib_array(normal_attrib_location as u32);
        gl.enable_vertex_attrib_array(uv_attrib_location as u32);
        gl.bind_vertex_array(Some(&vao));

        Self {
            index_count: mesh.index_size as i32,
            vertex_buffer: vbuffer,
            index_buffer: ibuffer,
            vao: vao,
            mat: mat
        }
    }

    pub fn render(&self, gl: &GL) {

        self.mat.use_material(gl);

        gl.bind_vertex_array(Some(&self.vao));

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