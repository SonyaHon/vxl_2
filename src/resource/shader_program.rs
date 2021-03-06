use crate::utils::funcs::create_whitespace_csting_with_len;

use super::shader::Shader;
use cgmath::prelude::*;

pub struct ShaderProgram {
    id: gl::types::GLuint,
}

impl ShaderProgram {
    pub fn new(shaders: &Vec<Shader>) -> Result<ShaderProgram, String> {
        let id = unsafe { gl::CreateProgram() };

        for shader in shaders {
            unsafe { gl::AttachShader(id, shader.get_id()) };
        }

        unsafe { gl::LinkProgram(id) };

        let mut success: gl::types::GLint = 1;
        unsafe { gl::GetProgramiv(id, gl::LINK_STATUS, &mut success) }

        if success == 0 {
            let mut len: gl::types::GLint = 0;
            unsafe {
                gl::GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let error = create_whitespace_csting_with_len(len as usize);

            unsafe {
                gl::GetProgramInfoLog(
                    id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar,
                )
            }

            return Err(error.to_string_lossy().into_owned());
        }

        for shader in shaders {
            unsafe { gl::DetachShader(id, shader.get_id()) };
        }

        Ok(ShaderProgram { id })
    }

    pub fn bind(&self) {
        unsafe { gl::UseProgram(self.id) }
    }

    pub fn unbind() {
        unsafe { gl::UseProgram(0) }
    }

    pub fn get_unifrom_location(&self, location_name: String) -> i32 {
        unsafe { gl::GetUniformLocation(self.id, location_name.as_ptr() as *const i8) }
    }

    pub fn add_unifrom_matrix4(&self, location_name: String, matrix: cgmath::Matrix4<f32>) {
        let uniform_location = self.get_unifrom_location(location_name);
        unsafe {
            gl::UniformMatrix4fv(uniform_location, 1, gl::FALSE, matrix.as_ptr())
        }
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe { gl::DeleteProgram(self.id) }
    }
}
