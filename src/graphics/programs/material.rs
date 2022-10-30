use web_sys::WebGl2RenderingContext;

pub trait Material {
    // use the program
    fn use_material(&self, gl: &WebGl2RenderingContext);

    // set uniform values
    fn init_uniforms(&self, gl: &WebGl2RenderingContext);
}