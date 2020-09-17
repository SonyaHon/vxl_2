use super::{loader, shader::Shader, shader_program::ShaderProgram, texture::Texture};
use std::path::Path;

pub struct Manager {
    loader: loader::Loader,
    loaded_shaders: std::collections::HashMap<&'static str, ShaderProgram>,
    loaded_textures: std::collections::HashMap<&'static str, Texture>,
}

impl Manager {
    pub fn new() -> Manager {
        Manager {
            loader: loader::Loader::new(),
            loaded_shaders: std::collections::HashMap::new(),
            loaded_textures: std::collections::HashMap::new(),
        }
    }

    pub fn load_shader_program(
        &mut self,
        shader_touples: Vec<(&str, gl::types::GLenum)>,
        program_name: &'static str,
    ) {
        let shader_prefix = Path::new("shaders/");
        let mut shaders: Vec<Shader> = Vec::with_capacity(shader_touples.len());

        for touple in shader_touples {
            let shader_source = self
                .loader
                .load_file_as_cstring(
                    &shader_prefix
                        .join(Path::new((touple.0.to_string() + ".glsl").as_str()))
                        .to_str()
                        .unwrap(),
                )
                .unwrap();
            let shader = Shader::new(&shader_source, touple.1).unwrap();
            &mut shaders.push(shader);
        }

        let program = ShaderProgram::new(&shaders).unwrap();

        self.loaded_shaders.entry(program_name).or_insert(program);

        println!("Shader \"{}\" loaded: OK", program_name);
    }

    pub fn load_texture(&mut self, asset_name: &str, texture_name: &'static str) {
        let image_prefix = Path::new("img/");
        let img = self.loader.load_image(
            image_prefix
                .join(Path::new((asset_name.to_string() + ".png").as_str()))
                .to_str()
                .unwrap(),
        );
        self.loaded_textures
            .entry(texture_name)
            .or_insert(Texture::new(img));
        println!("Texture \"{}\" loaded: OK", texture_name);
    }

    pub fn get_shader_program(&self, shader_program_name: &'static str) -> &ShaderProgram {
        &self.loaded_shaders[shader_program_name]
    }

    pub fn unbind_shader_program(&self) {
        ShaderProgram::unbind();
    }

    pub fn get_texture(&self, texture_name: &'static str) -> &Texture {
        &self.loaded_textures[texture_name]
    }

    pub fn unbind_texture(&self) {
        Texture::unbind();
    }
}
