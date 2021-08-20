use std::fs;
use std::ptr;

use gl::types::GLchar;
use gl::types::GLint;

pub struct Shader {
  pub program: u32,
  pub vertex: u32,
  pub fragment: u32
}

impl Shader {
  pub fn new( shader_path: &str ) -> Shader {
    // Load shader from file
    let shader_source = fs::read_to_string( shader_path ).expect( &format!( "Unable to read shader {}", shader_path ).as_str() );

    unsafe {
      // Create gl objects
      let program = gl::CreateProgram();
      let vertex = gl::CreateShader( gl::VERTEX_SHADER );
      let fragment = gl::CreateShader( gl::FRAGMENT_SHADER );

      // Format our shader/vertex sources individually so that they compile right
      let vertex_source = format!( "#version 450\n#define VERTEX\n{}\0", shader_source );
      let fragment_source = format!( "#version 450\n#define FRAGMENT\n{}\0", shader_source );

      // Set up sources
      let vertex_source_ptr = vertex_source.as_ptr() as *const i8;
      let fragment_source_ptr = fragment_source.as_ptr() as *const i8;

      // Compile vertex shader
      gl::ShaderSource( vertex, 1, &vertex_source_ptr, ptr::null() );
      gl::CompileShader( vertex );
      
      {
        let mut is_vertex_compiled: GLint = 0;
        gl::GetShaderiv( vertex, gl::COMPILE_STATUS, &mut is_vertex_compiled );
        if is_vertex_compiled == gl::FALSE.into() {
          let mut max_length = 0;
  
          gl::GetShaderiv( vertex, gl::INFO_LOG_LENGTH, &mut max_length );
  
          // error log as array of GLchar
          let mut error_log = Vec::with_capacity( max_length as usize );
          gl::GetShaderInfoLog( vertex, max_length, &mut max_length, error_log.as_mut_ptr() as *mut GLchar );  
          error_log.set_len( max_length as usize );

          panic!( "VERTEX SHADER COMPILATION FAILED:\n\t{}", String::from_utf8( error_log ).unwrap() );
        }
        else {
          println!( "VERTEX SHADER COMPILATION SUCCEEDED WITH STATE {}", is_vertex_compiled );
        }
      }

      // Compile fragment shader
      gl::ShaderSource( fragment, 1, &fragment_source_ptr, ptr::null() ); 
      gl::CompileShader( fragment );

      {
        let mut is_fragment_compiled: GLint = 0;
        gl::GetShaderiv( fragment, gl::COMPILE_STATUS, &mut is_fragment_compiled );
        if is_fragment_compiled == gl::FALSE.into() {
          let mut max_length = 0;
  
          gl::GetShaderiv( fragment, gl::INFO_LOG_LENGTH, &mut max_length );
  
          let mut error_log = Vec::with_capacity( max_length as usize );
          gl::GetShaderInfoLog( fragment, max_length, &mut max_length, error_log.as_mut_ptr() as *mut GLchar );  
          error_log.set_len( max_length as usize );

          panic!( "FRAGMENT SHADER COMPILATION FAILED:\n\t{}", String::from_utf8( error_log ).unwrap() );
        }
        else {
          println!( "FRAGMENT SHADER COMPILATION SUCCEEDED WITH STATE {}", is_fragment_compiled );
        }
      }

      // Attach to program
      gl::AttachShader( program, vertex );
      gl::AttachShader( program, fragment );

      // Check for errors
      let error = gl::GetError();
      if error != gl::NO_ERROR {
        panic!( "Error compiling shader {}", error );
      }

      gl::LinkProgram( program );

      return Shader {
        program,
        vertex,
        fragment
      }
    }
  }

  pub fn use_this( &self ) {
    unsafe {
      gl::UseProgram( self.program );
    }
  }
}