use super::{loader, shader_program::ShaderProgram, shader::Shader};
use std::path::Path;

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
        &mut self,
        shader_touples: Vec<(&str, gl::types::GLenum)>,
        program_name: &'static str,
    ) {
        let shader_prefix = Path::new("shaders\\");
        let mut shaders: Vec<Shader> = Vec::with_capacity(shader_touples.len());

        for touple in shader_touples {

            println!("{}", &shader_prefix.join(Path::new((touple.0.to_string() + ".glsl").as_str())).to_str().unwrap());
            let shader_source = self
                .loader
                .load_file_as_cstring(&shader_prefix.join(Path::new((touple.0.to_string() + ".glsl").as_str())).to_str().unwrap())
                .unwrap();
            let shader = Shader::new(&shader_source, touple.1).unwrap();
            &mut shaders.push(shader);
        }

        let program = ShaderProgram::new(&shaders).unwrap();

        self.loaded_shaders.entry(program_name).or_insert(program);
    }

    pub fn get_shader_program(&self, shader_program_name: &'static str) -> &ShaderProgram {
        &self.loaded_shaders[shader_program_name]
    }

    pub fn unbind_shader_program(&self) {
        ShaderProgram::unbind();
    }
}
