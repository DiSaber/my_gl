use std::ffi::CString;

use crate::utils;

#[derive(Clone, Copy)]
pub enum ShaderType {
    VertexShader = gl::VERTEX_SHADER as isize,
    FragmentShader = gl::FRAGMENT_SHADER as isize,
}

pub struct Shader {
    id: u32,
}

impl Shader {
    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn from_source(shader_source: &[u8], shader_type: ShaderType) -> Result<Self, CString> {
        let shader_source = CString::new(shader_source).unwrap();
        let shader = Self {
            id: unsafe { gl::CreateShader(shader_type as u32) },
        };
        unsafe {
            gl::ShaderSource(shader.id, 1, &shader_source.as_ptr(), std::ptr::null());
            gl::CompileShader(shader.id);
        }

        let mut success = 0i32;
        unsafe {
            gl::GetShaderiv(shader.id, gl::COMPILE_STATUS, &mut success);
        }

        if success == 0 {
            let mut error_length = 0i32;
            unsafe {
                gl::GetShaderiv(shader.id, gl::INFO_LOG_LENGTH, &mut error_length);
            }

            let error = utils::cstring_with_length(error_length as usize);
            unsafe {
                gl::GetShaderInfoLog(
                    shader.id,
                    error_length,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut i8,
                );
            }

            return Err(error);
        }

        Ok(shader)
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}
