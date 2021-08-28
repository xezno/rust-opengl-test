// ============================================================================
//
// mesh.rs
//
// Purpose: Mesh class
//
// ============================================================================

use gl::types::*;

use std::ffi::c_void;
use std::ptr;

pub struct Mesh {
    pub vbo: GLuint,
    pub vertex_count: GLint,
}

impl Mesh {
    pub fn new(vertices: Vec<GLfloat>, normals: Vec<GLfloat>) -> Mesh {
        let mut model: Mesh = Mesh {
            vbo: 0,
            vertex_count: (vertices.len() / 3) as GLint,
        };

        unsafe {
            gl::CreateBuffers(1, &mut model.vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, model.vbo);

            // Pack vertices & normals into single vec
            let mut gl_data: Vec<GLfloat> = Vec::new();
            for i in 0..(vertices.len() / 3) {
                gl_data.push(vertices[i * 3]);
                gl_data.push(vertices[i * 3 + 1]);
                gl_data.push(vertices[i * 3 + 2]);

                gl_data.push(normals[i * 3]);
                gl_data.push(normals[i * 3 + 1]);
                gl_data.push(normals[i * 3 + 2]);
            }

            // Buffer data
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (gl_data.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr,
                &gl_data[0] as *const GLfloat as *const c_void,
                gl::STATIC_DRAW,
            );

            let stride = (6 * std::mem::size_of::<GLfloat>()) as GLsizei;

            // Attributes
            // Position
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
            gl::EnableVertexAttribArray(0);

            // Normal
            gl::VertexAttribPointer(
                1,
                3,
                gl::FLOAT,
                gl::FALSE,
                stride,
                (3 * std::mem::size_of::<GLfloat>()) as *const c_void,
            );
            gl::EnableVertexAttribArray(1);
        }

        return model;
    }

    pub fn draw_this(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::DrawArrays(gl::TRIANGLES, 0, self.vertex_count);
        }
    }
}
