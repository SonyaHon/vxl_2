use crate::engine::utils::create_whitespace_csting_with_len;
use std::ffi::CStr;

pub fn shader_from_source(
    source: &CStr,
    kind: gl::types::GLenum,
) -> Result<gl::types::GLuint, String> {
    let id = unsafe { gl::CreateShader(kind) };

    unsafe {
        gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
        gl::CompileShader(id);
    }

    let mut result: gl::types::GLint = 1;
    unsafe { gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut result) };

    if result == 0 {
        let mut len: gl::types::GLint = 0;
        unsafe { gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len) };

        let error = create_whitespace_csting_with_len(len as usize);
        unsafe {
            gl::GetShaderInfoLog(
                id,
                len,
                std::ptr::null_mut(),
                error.as_ptr() as *mut gl::types::GLchar,
            )
        };

        return Err(error.to_string_lossy().into_owned());
    }

    Ok(id)
}

pub struct Shader {
    id: gl::types::GLuint,
}

impl Shader {
    pub fn new(source: &CStr, kind: gl::types::GLenum) -> Result<Shader, String> {
        let id = shader_from_source(source, kind)?;
        Ok(Shader { id })
    }

    pub fn get_id(&self) -> gl::types::GLuint {
        self.id
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe { gl::DeleteShader(self.id) }
    }
}
