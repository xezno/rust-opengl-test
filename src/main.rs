extern crate sdl2;
extern crate gl;

use gl::types::*;
use core::ffi::c_void;
use std::ptr;

pub mod shader;
use shader::Shader;

fn main() {
    let _sdl = sdl2::init().unwrap();
    let _video_subsystem = _sdl.video().unwrap();
    let _window = _video_subsystem.window( "My Game", 1280, 720 ).opengl().resizable().build().unwrap();

    let _gl_context = _window.gl_create_context().unwrap();
    
    let _gl = gl::load_with( |s| _video_subsystem.gl_get_proc_address( s ) as *const _ );
    let _viewport = gl::Viewport::load_with( |s| _video_subsystem.gl_get_proc_address( s ) as *const _ );

    gfx_clear_color( 0.34, 0.12, 0.56, 1.0 );
    
    //
    // Create triangle
    //
    let _shader = Shader::new( "content/shaders/triangle.glsl" );
    let mut vbo: GLuint = 0;
    unsafe {

        // Triangle vertices
        let vertices: [GLfloat; 9] = [
            -0.5, -0.5, 0.0,
            0.5, -0.5, 0.0,
            0.0, 0.5, 0.0
        ];
    
        gl::CreateBuffers( 1, &mut vbo );
        gl::BindBuffer( gl::ARRAY_BUFFER, vbo );
        
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
    
    let mut _event_pump = _sdl.event_pump().unwrap();
    'main: loop {
        if !input_event_poll( &mut _event_pump ) {
            break 'main;
        }

        gfx_clear();

        // Draw code here

        unsafe {
            _shader.use_this();

            gl::BindBuffer( gl::ARRAY_BUFFER, vbo );
            gl::DrawArrays( gl::TRIANGLES, 0, 3 );
        }

        _window.gl_swap_window();
    }
}

fn input_event_poll( event_pump: &mut sdl2::EventPump ) -> bool {
    for _event in event_pump.poll_iter() {
        match _event {
            sdl2::event::Event::Quit { .. } => return false,
            _ => {}
        }
    }

    return true;
}


fn gfx_clear_color( red: f32, green: f32, blue: f32, alpha: f32 ) { 
    unsafe {
        gl::ClearColor( red, green, blue, alpha );
    }
}

fn gfx_clear() {
    unsafe {
        gl::Clear( gl::COLOR_BUFFER_BIT );
    }
}