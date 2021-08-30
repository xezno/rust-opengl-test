// ============================================================================
//
// texture.rs
//
// Purpose:
//
// ============================================================================

use std::{ffi::c_void, fs::File};

use gl::types::*;

#[derive(Default)]
pub struct Texture {
    pub id: GLuint,
}

impl Texture {
    pub fn new(texture_path: &str) -> Texture {
        log::info!("Loading texture: {}", texture_path);

        let image_data_encoded = File::open(texture_path).unwrap();

        let decoder = png::Decoder::new(image_data_encoded);
        let mut reader = decoder.read_info().unwrap();
        let mut buf = vec![0; reader.output_buffer_size()];
        let info = reader.next_frame(&mut buf).unwrap();
        let bytes = &buf[..info.buffer_size()];

        let info = reader.info();

        let bytes_ptr = bytes.as_ptr() as *const c_void;

        // Create gl texture
        let mut id: GLuint = 0;
        unsafe {
            gl::GenTextures(1, &mut id);
            log::info!("Creating GL texture {}", id);
            gl::BindTexture(gl::TEXTURE_2D, id);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                info.width as i32,
                info.height as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                bytes_ptr,
            );
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

            gl::BindTexture(gl::TEXTURE_2D, 0);

            log::info!("Texture loaded: {}", id);
        }

        return Texture { id };
    }

    pub fn use_this(&self) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }
}
