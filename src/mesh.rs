
use gl::types::*;

use std::ptr;
use std::ffi::c_void;

pub struct Mesh {
  // vertices: Vec<Vertex>,
  pub vbo: GLuint,
  pub vertexCount: GLint
}

impl Mesh {
  pub fn new(vertices: Vec<GLfloat>) -> Mesh {
    let mut model: Mesh = Mesh {
      vbo: 0,
      vertexCount: (vertices.len() / 3) as GLint
    };

    unsafe {
      gl::CreateBuffers( 1, &mut model.vbo );
      gl::BindBuffer( gl::ARRAY_BUFFER, model.vbo );
      
      // Buffer data
      gl::BufferData( gl::ARRAY_BUFFER,
          (vertices.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr,
          &vertices[0] as *const GLfloat as *const c_void,
          gl::STATIC_DRAW );

      // Attributes
      // Position
      gl::VertexAttribPointer( 0, 3, gl::FLOAT, gl::FALSE, 0, ptr::null() );
      gl::EnableVertexAttribArray( 0 );
    }

    return model;
  }
}