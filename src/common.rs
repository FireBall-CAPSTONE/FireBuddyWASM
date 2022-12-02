use wasm_bindgen::{JsCast, JsValue};
use web_sys::{HtmlCanvasElement, {WebGl2RenderingContext as GL}, WebGlShader, WebGlProgram};

use crate::register_mouse_events;


/// Get a `WebGl2RenderingContext` from a canvas element with the specified
/// `element_id`
/// 
/// Attempt to find a `HtmlCanvasElement` from the DOM with the passed in
/// `element_id` and then get the `WebGl2RenderingContext`. 
/// 
/// # Panics
/// 
/// There is no `HtmlCanvasElement` with the `element_id`.
/// 
/// The browser does not support webGl 2 and cannot provide a context.
/// 
/// # Examples
/// ```
/// let gl = get_gl_context('glCanvas')?;
/// ```
pub fn get_gl_context(element_id: &str) -> Result<GL, JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id(element_id).unwrap();
    let canvas:HtmlCanvasElement = canvas.dyn_into::<HtmlCanvasElement>()?;

    register_mouse_events(&canvas).unwrap();

    let gl = canvas
        .get_context("webgl2")? // Using webgl2
        .unwrap()
        .dyn_into::<GL>()?;

    Ok(gl)
}

/// Compile a shader from a source string
/// 
/// Takes in a reference to the `WebGl2RenderingContext` and a source string
/// to compile a shader of the specified type (typically either 
/// `FRAGMENT_SHADER` or `VERT_SHADER`)
/// 
/// Returns a `WebGlShader` if compilation was successful, or a string 
/// containing an error if the compilation failed.
/// 
/// # Examples
/// ```
/// let shader_source: str& = r##"
///     #version 330 es
/// 
///     precision mediump float;
///     out vec4 outColor;
/// 
///     void main() {
///         outColor = vec4(1.0, 1.0, 1.0, 1.0);
///     }
/// "##;
/// 
/// let frag_shader = compile_shader(
///     &gl,
///     WebGl2RenderingContext::FRAGMENT_SHADER,
///     shader_source
/// )?;
/// ```
pub fn compile_shader(
    gl: &GL,
    shader_type: u32,
    source: &str
) -> Result<WebGlShader, String> {
    // Return a shader

    let shader = gl
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    gl.shader_source(&shader, source);
    gl.compile_shader(&shader);

    if gl
        .get_shader_parameter(&shader, GL::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(gl
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader"))
        )
    }

}

/// Creates a new shader program and links an existing vertex and 
/// fragment shader.
/// 
/// Takes in a reference to a `WebGl2RenderingContext` and compiled
/// vertex and fragment shaders. Returns a result containing either 
/// a `WebGlProgram` if the linking was successful or a `String` 
/// containing the error message if the program failed to link
/// successfully.
/// 
/// # Examples
/// 
/// ```
/// let frag_shader_source = r##"
///     // fragment shader source
/// "##;
/// 
/// let vert_shader_source = r##"
///     // vertex shader source
/// "##;
/// 
/// let frag_shader = compile_shader(
///     &gl,
///     WebGl2RenderingContext::FRAGMENT_SHADER,
///     frag_shader_source
/// )?;
/// 
/// let vert_shader = compile_shader(
///     &gl,
///     WebGl2RenderingContext::VERTEX_SHADER,
///     vert_shader_source
/// )?;
/// 
/// let program = link_program(
///     &gl
///     &vert_shader,
///     &frag_shader
/// )?;
/// ```
pub fn link_program(
    gl: &GL,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader
) -> Result<WebGlProgram, String> {
    
    let program = gl
        .create_program()
        .ok_or_else(|| String::from("Unable to create program ojbect"))?;
    
    gl.attach_shader(&program, vert_shader);
    gl.attach_shader(&program, frag_shader);
    gl.link_program(&program);

    if gl
        .get_program_parameter(&program, GL::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(gl
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating program object"))
        )
    }
}


