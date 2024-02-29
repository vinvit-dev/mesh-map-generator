use std::ffi::CStr;

use gl::types::GLuint;
use glm::value_ptr;
use nalgebra_glm as glm;

use crate::shader::{Shader, ShaderType};

pub enum Uniform {
    Float(f32),
    Float3(glm::Vec3),
    Float4(glm::Vec4),
    Mat4(glm::Mat4),
}

pub struct ShaderProgram(pub GLuint);

impl ShaderProgram {
    pub fn new() -> Option<Self> {
        let prog = unsafe { gl::CreateProgram() };
        if prog != 0 {
            Some(Self(prog))
        } else {
            None
        }
    }

    pub fn attach_shader(&self, shader: &Shader) {
        unsafe { gl::AttachShader(self.0, shader.0) };
    }

    pub fn link_program(&self) {
        unsafe { gl::LinkProgram(self.0) };
    }

    pub fn link_success(&self) -> bool {
        let mut success = 0;
        unsafe { gl::GetProgramiv(self.0, gl::LINK_STATUS, &mut success) };
        success == i32::from(gl::TRUE)
    }

    pub fn info_log(&self) -> String {
        let mut needed_len = 0;
        unsafe { gl::GetProgramiv(self.0, gl::INFO_LOG_LENGTH, &mut needed_len) };
        let mut v: Vec<u8> = Vec::with_capacity(needed_len.try_into().unwrap());
        let mut len_written = 0_i32;
        unsafe {
            gl::GetProgramInfoLog(
                self.0,
                v.capacity().try_into().unwrap(),
                &mut len_written,
                v.as_mut_ptr().cast(),
            );
            v.set_len(len_written.try_into().unwrap());
        }
        String::from_utf8_lossy(&v).into_owned()
    }

    pub fn use_program(&self) {
        unsafe { gl::UseProgram(self.0) };
    }

    pub fn delete(self) {
        unsafe { gl::DeleteProgram(self.0) };
    }

    pub fn from_vert_frag(vert: &str, frag: &str) -> Result<Self, String> {
        let p = Self::new().ok_or_else(|| "Couldn't allocate a program".to_string())?;
        let v = Shader::from_source(ShaderType::Vertex, vert)
            .map_err(|e| format!("Vertex Compile Error: {}", e))?;
        let f = Shader::from_source(ShaderType::Fragment, frag)
            .map_err(|e| format!("Fragment Compile Error: {}", e))?;
        p.attach_shader(&v);
        p.attach_shader(&f);
        p.link_program();
        v.delete();
        f.delete();
        if p.link_success() {
            Ok(p)
        } else {
            let out = format!("Program Link Error: {}", p.info_log());
            p.delete();
            Err(out)
        }
    }
    pub fn from_vert_frag_files(vert_path: &str, frag_path: &str) -> Result<Self, String> {
        Self::from_vert_frag(
            Shader::read_shader_file(vert_path).unwrap().as_str(),
            Shader::read_shader_file(frag_path).unwrap().as_str(),
        )
    }

    pub fn set_uniform(&self, name: &str, value: Uniform) {
        let uniform_location = unsafe {
            gl::GetUniformLocation(
                self.0,
                CStr::from_bytes_with_nul_unchecked(format!("{}\0", name).as_bytes()).as_ptr()
                    as *const gl::types::GLchar,
            )
        };
        match value {
            Uniform::Float(value) => unsafe {
                gl::Uniform1f(uniform_location, value);
            },
            Uniform::Float3(value) => unsafe {
                gl::Uniform3f(uniform_location, value.x, value.y, value.z);
            },
            Uniform::Float4(value) => unsafe {
                gl::Uniform4f(uniform_location, value.x, value.y, value.z, value.w);
            },
            Uniform::Mat4(value) => unsafe {
                gl::UniformMatrix4fv(uniform_location, 1, gl::FALSE, value_ptr(&value).as_ptr());
            },
        }
    }
}
