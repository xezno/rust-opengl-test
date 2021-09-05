// ============================================================================
//
// shader.rs
//
// Purpose: Shader class
//
// ============================================================================

use std::convert::TryInto;
use std::fs;
use std::ptr;

use gl::types::*;
use std::collections::HashMap;

pub struct Shader {
    pub shader_path: String,

    pub program: u32,
    pub vertex: u32,
    pub fragment: u32,

    program_uniforms: HashMap<String, GLint>,
}

impl Shader {
    pub fn new(shader_path: &str) -> Shader {
        let mut shader = Shader {
            shader_path: shader_path.to_string(),

            program: 0,
            vertex: 0,
            fragment: 0,

            program_uniforms: HashMap::new(),
        };

        shader.load();

        return shader;
    }

    pub fn load(&mut self) {
        log::trace!("Loading shader {}", self.shader_path);

        // Load shader from file
        let shader_source = fs::read_to_string(self.shader_path.as_str())
            .expect(&format!("Unable to read shader {}", self.shader_path).as_str());

        unsafe {
            // Create gl objects
            let program = gl::CreateProgram();
            let vertex = gl::CreateShader(gl::VERTEX_SHADER);
            let fragment = gl::CreateShader(gl::FRAGMENT_SHADER);

            // Format our shader/vertex sources individually so that they compile right
            let vertex_source = format!("#version 330 core\n#define VERTEX\n{}\0", shader_source);
            let fragment_source =
                format!("#version 330 core\n#define FRAGMENT\n{}\0", shader_source);

            // Set up sources
            let vertex_source_ptr = vertex_source.as_ptr() as *const i8;
            let fragment_source_ptr = fragment_source.as_ptr() as *const i8;

            // Compile vertex shader
            gl::ShaderSource(vertex, 1, &vertex_source_ptr, ptr::null());
            gl::CompileShader(vertex);
            Shader::check_shader_errors(vertex, "vertex", self.shader_path.as_str());

            // Compile fragment shader
            gl::ShaderSource(fragment, 1, &fragment_source_ptr, ptr::null());
            gl::CompileShader(fragment);
            Shader::check_shader_errors(fragment, "fragment", self.shader_path.as_str());

            // Attach to program
            gl::AttachShader(program, vertex);
            gl::AttachShader(program, fragment);

            // crate::render::gfx::gfx_check_generic_errors();

            gl::LinkProgram(program);

            self.program = program;
            self.vertex = vertex;
            self.fragment = fragment;

            self.scan_uniforms();
        }
    }

    fn check_shader_errors(shader: GLuint, shader_type: &str, shader_path: &str) {
        let mut is_fragment_compiled: GLint = 0;
        unsafe {
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut is_fragment_compiled);
        }
        if is_fragment_compiled == gl::FALSE as i32 {
            let mut max_length = 0;

            unsafe {
                gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut max_length);
            }

            let mut error_log = Vec::with_capacity(max_length as usize);
            unsafe {
                gl::GetShaderInfoLog(
                    shader,
                    max_length,
                    &mut max_length,
                    error_log.as_mut_ptr() as *mut GLchar,
                );
                error_log.set_len(max_length as usize);
            }

            panic!(
                "Shader {} ('{}') compile failed:\n\t{}",
                shader_type,
                shader_path,
                String::from_utf8(error_log).unwrap()
            );
        } else {
            log::trace!(
                "Shader {} ('{}') compilation success",
                shader_type,
                shader_path
            );
        }
    }

    pub fn scan_uniforms(&mut self) {
        let mut uniforms: GLint = 0;

        unsafe {
            gl::GetProgramiv(self.program, gl::ACTIVE_UNIFORMS, &mut uniforms);

            for i in 0..uniforms {
                const MAX_NAME_LENGTH: i32 = 128;

                let mut name_length = 0;
                let mut num: GLint = 0;
                let mut type_: GLenum = 0;

                let mut name_ = Vec::with_capacity(MAX_NAME_LENGTH as usize);

                gl::GetActiveUniform(
                    self.program,
                    i.try_into().unwrap(),
                    MAX_NAME_LENGTH - 1,
                    &mut name_length,
                    &mut num,
                    &mut type_,
                    name_.as_mut_ptr() as *mut GLchar,
                );

                name_.set_len(name_length as usize);
                let name = String::from_utf8(name_).unwrap();

                log::trace!(
                    "Shader {}, uniform: {}, location: {}",
                    self.program,
                    name,
                    i
                );

                if !self.program_uniforms.contains_key(&name) {
                    self.program_uniforms.insert(format!("{}", name), i);
                }
            }
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::UseProgram(self.program);
        }
    }

    fn get_location(&mut self, name: &str) -> GLint {
        let location: GLint;

        if self.program_uniforms.contains_key(name) {
            location = self.program_uniforms[name];
        } else {
            // log::warn!("Warn: Shader was never scanned for uniform {}", name);
            location = -1;
        }

        return location;
    }

    pub fn set_mat4(&mut self, name: &str, val: &glam::Mat4) -> () {
        let location = self.get_location(name);

        unsafe {
            let mat_ptr: *const GLfloat = &val.to_cols_array()[0];
            gl::ProgramUniformMatrix4fv(self.program, location, 1, gl::FALSE, mat_ptr);
        }
    }

    pub fn set_vec3(&mut self, name: &str, val: &glam::Vec3) -> () {
        let location = self.get_location(name);

        unsafe {
            let vec_ptr: *const GLfloat = &val.to_array()[0];
            gl::ProgramUniform3fv(self.program, location, 1, vec_ptr);
        }
    }

    pub fn set_vec4(&mut self, name: &str, val: &glam::Vec4) -> () {
        let location = self.get_location(name);

        unsafe {
            let vec_ptr: *const GLfloat = &val.to_array()[0];
            gl::ProgramUniform4fv(self.program, location, 1, vec_ptr);
        }
    }

    pub fn set_f32(&mut self, name: &str, val: f32) -> () {
        let location = self.get_location(name);

        unsafe {
            gl::ProgramUniform1f(self.program, location, val);
        }
    }

    pub fn set_i32(&mut self, name: &str, val: i32) -> () {
        let location = self.get_location(name);

        unsafe {
            gl::ProgramUniform1i(self.program, location, val);
        }
    }

    pub fn set_u32(&mut self, name: &str, val: u32) -> () {
        let location = self.get_location(name);

        unsafe {
            gl::ProgramUniform1i(self.program, location, val as i32);
        }
    }
}
