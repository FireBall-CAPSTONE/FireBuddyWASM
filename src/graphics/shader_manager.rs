use std::{collections::HashMap, sync::{Arc, Mutex}, borrow::Borrow};

use lazy_static::lazy_static;
// use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{WebGlShader, WebGl2RenderingContext as GL};

use crate::{common, js_log};

lazy_static! {
    static ref SHADER_MANAGER: Arc<Mutex<HashMap<String, usize>>> = Arc::new(Mutex::new(HashMap::new()));
}

thread_local! { static SHADER_VEC: Vec<WebGlShader> = Vec::new() }

/// Compile a shader and store the result in the resulting shader in the Shader Manager
pub fn expose_shader(gl: &GL, source: &str, shader_type: u32, key: &str) -> WebGlShader {
    let shader = common::compile_shader(
        &gl, 
        shader_type, 
        source
    ).unwrap();

    // Funny pointer magic
    let shader_ptr = std::ptr::addr_of!(shader);

    // let shader_ptr = &shader as *const WebGlShader;
    let shader_addr = shader_ptr as usize; // Get the address of the shader as a usize

    let mut shader_manager = SHADER_MANAGER.lock().unwrap();
    shader_manager.insert(String::from(key), shader_addr);

    shader // Return the shader to prevent deallocation
}

pub fn get_shader(key: &str) -> &WebGlShader {
    let shader_manager = SHADER_MANAGER.lock().unwrap();
    // Just pretend that the key exists, this will certainly not cause problems in the future :)
    // (foreshadowing is a narrative technique... )
    let shader_addr = shader_manager.get(key).unwrap().to_owned();
    let shader_ptr = shader_addr as *const WebGlShader;
    let shader = unsafe{ &*shader_ptr };

    shader // Return the shader
}

pub struct ShaderManager {
    shaders: HashMap<String, WebGlShader>
}

impl ShaderManager {
    pub fn new() -> Self {
        Self { shaders: HashMap::new() }
    }

    pub fn expose_shader(
        &mut self,
        shader: WebGlShader,
        key: &str
    ) {
        self.shaders.insert(String::from(key), shader);
    }

    pub fn get_shader(&self, key: &str) -> &WebGlShader {
        if self.shaders.contains_key(key) {
            self.shaders.get(key).unwrap()
        } else {
            js_log("Could not find shader");
            panic!()
        }
    }
}
