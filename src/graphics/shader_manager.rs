use std::{collections::HashMap, sync::{Arc, Mutex}};

use lazy_static::lazy_static;
// use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{WebGlShader, WebGlProgram};

use crate::{js_log};

lazy_static! {
    static ref SHADER_MANAGER: Arc<Mutex<HashMap<String, usize>>> = Arc::new(Mutex::new(HashMap::new()));
}

thread_local! { static SHADER_VEC: Vec<WebGlShader> = Vec::new() }

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

pub struct ShaderProgramManager {
    programs: HashMap<String, WebGlProgram>
}

impl ShaderProgramManager {
    pub fn new() -> Self {
        Self { programs: HashMap::new() }
    }

    pub fn expose_program(&mut self, program: WebGlProgram, key: &str) {
        self.programs.insert(String::from(key), program);
    }

    pub fn get_program(&self, key: &str) -> &WebGlProgram {
        if self.programs.contains_key(key) {
            self.programs.get(key).unwrap()
        } else {
            js_log("Could not find program");
            panic!()
        }
    }
}
