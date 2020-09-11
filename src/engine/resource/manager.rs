use super::loader;
use std::path::Path;

use crate::engine::graphics::shader_program::ShaderProgram;

pub struct Manager {
    loader: loader::Loader,
    loaded_shaders: std::collections::HashMap<&'static str, ShaderProgram>,
}

impl Manager {
    pub fn new() -> Manager {
        Manager {
            loader: loader::Loader::new(),
            loaded_shaders: std::collections::HashMap::new(),
        }
    }

    pub fn load_shader_program(
        &self,
        shader_touples: Vec<(&str, gl::types::GLenum)>,
        program_name: &str,
    ) {
    }
}
