pub trait Material {
    // use the program
    fn use_material(&self);

    // set uniform values
    fn init_uniforms(&self);
}