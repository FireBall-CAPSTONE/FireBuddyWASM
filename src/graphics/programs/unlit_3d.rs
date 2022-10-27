use web_sys::{WebGlProgram, WebGlTexture};

use super::material::Material;

pub struct Unlit3D {
    program: WebGlProgram,

}

impl Unlit3D {

}

impl Material for Unlit3D {
    fn use_material(&self) {
        todo!()
    }

    fn init_uniforms(&self) {
        todo!()
    }
}

pub struct UnlitTextured3D {
    program: WebGlProgram,
    texture: WebGlTexture,
}

impl UnlitTextured3D {
    pub fn new() -> Self {
        todo!()
    }
}

impl Material for UnlitTextured3D {
    fn use_material(&self) {
        todo!()
    }

    fn init_uniforms(&self) {
        // load the texture
        // assign texture units???
        todo!()
    }
}