use na::Matrix4;
use std::ffi::CString;

use crate::{shader::Shader, utils};

pub struct ShaderProgram {
    id: u32,
}

impl ShaderProgram {
    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    pub fn set_value(&self, name: &str, value: ProgramValue) {
        self.use_program();
        let name = CString::new(name).unwrap();
        let location = unsafe { gl::GetUniformLocation(self.id, name.as_ptr()) };
        match value {
            ProgramValue::Int(int) => unsafe {
                gl::Uniform1i(location, int);
            },
            ProgramValue::Mat4(mat) => unsafe {
                gl::UniformMatrix4fv(location, 1, gl::FALSE, mat.as_ptr())
            },
        };
    }

    pub fn from_shaders(vertex_shader: &Shader, fragment_shader: &Shader) -> Result<Self, CString> {
        let program = Self {
            id: unsafe { gl::CreateProgram() },
        };

        unsafe {
            gl::AttachShader(program.id, vertex_shader.get_id());
            gl::AttachShader(program.id, fragment_shader.get_id());
        }

        unsafe {
            gl::LinkProgram(program.id);
        }

        let mut success = 0i32;
        unsafe {
            gl::GetProgramiv(program.id, gl::LINK_STATUS, &mut success);
        }

        if success == 0 {
            let mut error_length = 0i32;
            unsafe {
                gl::GetProgramiv(program.id, gl::INFO_LOG_LENGTH, &mut error_length);
            }

            let error = utils::cstring_with_length(error_length as usize);
            unsafe {
                gl::GetProgramInfoLog(
                    program.id,
                    error_length,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut i8,
                )
            }

            return Err(error);
        }

        unsafe {
            gl::DetachShader(program.id, vertex_shader.get_id());
            gl::DetachShader(program.id, fragment_shader.get_id());
        }

        Ok(program)
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

#[derive(Clone, Copy)]
pub enum ProgramValue {
    Int(i32),
    Mat4(Matrix4<f32>),
}
