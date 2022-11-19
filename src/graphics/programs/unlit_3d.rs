use std::{rc::Rc};
use wasm_bindgen::{JsValue, prelude::Closure, JsCast};
use web_sys::{WebGlProgram, WebGlTexture, HtmlImageElement, WebGl2RenderingContext as GL};

use crate::{js_log, common, graphics::{shader_manager::{ShaderManager, ShaderProgramManager}}, app_state::{peek_mat_stack, get_canvas_width, get_canvas_height}, math::{mat4::Matrix4, vec3::Vector3}};

use super::material::Material;

pub struct Unlit3D {
    program: WebGlProgram,
}

impl Unlit3D {
    pub fn new(gl: &GL, program_manager: &ShaderProgramManager) -> Self {
        let prgm = program_manager.get_program("simple_unlit").to_owned();
    
        Self {
            program: prgm
        }
    }
    
}

impl Material for Unlit3D {
    fn use_material(&self, gl: &GL) {
        gl.use_program(Some(&self.program));
        self.init_uniforms(&gl);
    }

    fn init_uniforms(&self, gl: &GL) {

        // js_log("init proj");
        let proj_mat_location = gl.get_uniform_location(
            &self.program, 
            "projection_matrix"
        ).unwrap();

        let view_mat_location = gl.get_uniform_location(
            &self.program,
            "view_matrix"
        ).unwrap();

        // js_log("init trans");
        let transform_mat_location = gl.get_uniform_location(
            &self.program, 
            "transform_matrix"
        ).unwrap();
        
        // TODO: Get this from application state
        let height = get_canvas_height();
        let width = get_canvas_width();
        let proj_mat = Matrix4::perspective(
            0.436 * 2.0, 
            width/height, 
            0.1, 
            100.0
        );

        // TODO Get camera pos and rotation
        let view_mat = Matrix4::view(
            Vector3::new(0.0, 0.0, -5.0), 
            Vector3::up(), 
            // &Vector3::new(2.5, 0.0, 15.5).normalize()
            -Vector3::forward()
        );

        let view_proj_mat = view_mat * proj_mat;
        let world_mat = peek_mat_stack();

        gl.uniform_matrix4fv_with_f32_array(
            Some(&transform_mat_location), 
            false, 
            &world_mat.data
        );

        gl.uniform_matrix4fv_with_f32_array(
            Some(&proj_mat_location), 
            false, 
            &proj_mat.data
        );

        gl.uniform_matrix4fv_with_f32_array(
            Some(&view_mat_location), 
            false, 
            &view_mat.data
        );
    }

    fn get_program(&self) -> WebGlProgram {
        self.program.clone()
    }
}

pub struct UnlitTextured3D {
    pub program: WebGlProgram,
    pub texture: Rc<WebGlTexture>,

}

impl UnlitTextured3D {
    pub fn new(gl: &GL, img_src: &str, program_manager: &ShaderProgramManager) -> Self {
        let tex = load_texture(gl, img_src).unwrap();
        let prgm = program_manager.get_program("textured_lit").to_owned();

        Self {
            program: prgm,
            texture: tex,
        }

        // todo!()
    }
}

impl Material for UnlitTextured3D {
    fn use_material(&self, gl: &GL) {
        gl.use_program(Some(&self.program));
        self.init_uniforms(gl);
        // todo!()
    }

    fn init_uniforms(&self, gl: &GL) {

        // js_log("init sampler");
        let sampler_location = gl.get_uniform_location(
            &self.program,
            "tex"
        ).unwrap();

        // js_log("init proj");
        let proj_mat_location = gl.get_uniform_location(
            &self.program, 
            "projection_matrix"
        ).unwrap();

        let view_mat_location = gl.get_uniform_location(
            &self.program,
            "view_matrix"
        ).unwrap();

        // js_log("init trans");
        let transform_mat_location = gl.get_uniform_location(
            &self.program, 
            "transform_matrix"
        ).unwrap();
        
        // TODO: Get this from application state
        let height = get_canvas_height();
        let width = get_canvas_width();
        let proj_mat = Matrix4::perspective(
            0.436 * 2.0, 
            width/height, 
            0.1, 
            100.0
        );

        // TODO Get camera pos and rotation
        let view_mat = Matrix4::view(
            Vector3::new(0.0, 0.0, -5.0), 
            Vector3::up(), 
            // &Vector3::new(2.5, 0.0, 15.5).normalize()
            -Vector3::forward()
        );

        let view_proj_mat = view_mat * proj_mat;
        let world_mat = peek_mat_stack();

        gl.uniform_matrix4fv_with_f32_array(
            Some(&transform_mat_location), 
            false, 
            &world_mat.data
        );

        gl.uniform_matrix4fv_with_f32_array(
            Some(&proj_mat_location), 
            false, 
            &proj_mat.data
        );

        gl.uniform_matrix4fv_with_f32_array(
            Some(&view_mat_location), 
            false, 
            &view_mat.data
        );

        gl.tex_parameteri(
            GL::TEXTURE_2D, 
            GL::TEXTURE_MIN_FILTER, 
            GL::LINEAR as i32
        ); // Set filtering mode to linear (default is nearest)

        gl.active_texture(GL::TEXTURE0);
        gl.bind_texture(GL::TEXTURE_2D, Some(&self.texture));
        gl.uniform1i(Some(&sampler_location), 0);

        // todo!()
    }

    fn get_program(&self) -> WebGlProgram {
        self.program.clone()
    }
}

fn load_texture(
    gl: &GL,
    img_src: &str,
) -> Result<Rc<WebGlTexture>, JsValue> {
    let texture = gl.create_texture().expect("Cannot create gl texture");
    gl.bind_texture(GL::TEXTURE_2D, Some(&texture));
    let level = 0;
    let internal_format = GL::RGBA;
    let width = 1;
    let height = 1;
    let border = 0;
    let src_format = GL::RGBA;
    let src_type = GL::UNSIGNED_BYTE;
    let pixel: [u8; 4] = [200, 95, 10, 255];
    // This is the worst method signature I have ever seen
    gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
        GL::TEXTURE_2D, 
        level, 
        internal_format as i32, 
        width, 
        height, 
        border, 
        src_format, 
        src_type, 
        Some(&pixel),
    )?;


    let img = HtmlImageElement::new().unwrap();

    img.set_cross_origin(Some(""));
    let imgrc = Rc::new(img);
    let texture = Rc::new(texture);

    {

        let img = imgrc.clone();
        let texture = texture.clone();
        let gl = Rc::new(gl.clone());
        let a = Closure::wrap(Box::new(move || {
            gl.bind_texture(GL::TEXTURE_2D, Some(&texture));

            if let Err(e) = gl.tex_image_2d_with_u32_and_u32_and_html_image_element(
                GL::TEXTURE_2D, 
                level, 
                internal_format as i32, 
                src_format, 
                src_type, 
        &img
            ) {
                js_log(&e.as_string().unwrap());
                return;
            }

            gl.generate_mipmap(GL::TEXTURE_2D);
        }) as Box<dyn FnMut()>);
        imgrc.set_onload(Some(a.as_ref().unchecked_ref()));

        // This is a literal memory leak but it's okay :D
        a.forget();
    }

    imgrc.set_src(img_src);

    Ok(texture)

}