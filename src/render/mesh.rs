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

use super::texture::Texture;

pub struct Mesh {
    pub vbo: GLuint,
    pub vao: GLuint,
    pub ebo: GLuint,
    pub vertex_count: GLint,
    pub index_count: GLint,

    pub diffuse_texture: Texture,
}

impl Mesh {
    pub fn new(
        vertices: Vec<GLfloat>,
        normals: Vec<GLfloat>,
        texcoords: Vec<GLfloat>,
        indices: Vec<GLuint>,
        diffuse_texture: Texture,
    ) -> Mesh {
        let mut model: Mesh = Mesh {
            vbo: 0,
            vao: 0,
            ebo: 0,
            vertex_count: (vertices.len() / 3) as GLint,
            index_count: (indices.len()) as GLint,
            diffuse_texture,
        };

        unsafe {
            gl::GenVertexArrays(1, &mut model.vao);
            gl::GenBuffers(1, &mut model.vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, model.vbo);
            gl::BindVertexArray(model.vao);

            log::trace!("Created VBO: {}", model.vbo);

            // Pack vertices & normals into single vec
            let mut gl_data: Vec<GLfloat> = Vec::new();
            for i in 0..(vertices.len() / 3) {
                gl_data.push(vertices[i * 3]);
                gl_data.push(vertices[i * 3 + 1]);
                gl_data.push(vertices[i * 3 + 2]);

                gl_data.push(normals[i * 3]);
                gl_data.push(normals[i * 3 + 1]);
                gl_data.push(normals[i * 3 + 2]);

                gl_data.push(texcoords[i * 2]);
                gl_data.push(texcoords[i * 2 + 1]);
            }

            // Buffer data
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (gl_data.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr,
                &gl_data[0] as *const GLfloat as *const c_void,
                gl::STATIC_DRAW,
            );

            let stride = (8 * std::mem::size_of::<GLfloat>()) as GLsizei;

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

            // Texture Coordinates
            gl::VertexAttribPointer(
                2,
                2,
                gl::FLOAT,
                gl::FALSE,
                stride,
                (6 * std::mem::size_of::<GLfloat>()) as *const c_void,
            );
            gl::EnableVertexAttribArray(2);

            // Indices
            gl::GenBuffers(1, &mut model.ebo);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, model.ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * std::mem::size_of::<GLuint>()) as GLsizeiptr,
                &indices[0] as *const GLuint as *const c_void,
                gl::STATIC_DRAW,
            );
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }

        return model;
    }

    pub fn render(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);

            gl::DrawElements(
                gl::TRIANGLES,
                self.index_count,
                gl::UNSIGNED_INT,
                ptr::null(),
            );

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }
    }
}
