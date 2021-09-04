// ============================================================================
//
// texture.rs
//
// Purpose:
//
// ============================================================================

use std::{ffi::c_void, fs::File};

use gl::types::*;
use image::{EncodableLayout, GenericImageView};

#[derive(Default)]
pub struct Texture {
    pub id: GLuint,
}

impl Texture {
    pub fn new(texture_path: &str) -> Texture {
        log::info!("Loading texture: {}", texture_path);

        let image = image::open(texture_path).unwrap();
        let image_clone = image.clone();
        let image_rgba8 = image_clone.into_rgba8();
        let bytes = image_rgba8.as_bytes();

        let bytes_ptr = bytes.as_ptr() as *const c_void;

        // Create gl texture
        let mut id: GLuint = 0;
        unsafe {
            gl::GenTextures(1, &mut id);
            log::trace!("Creating GL texture {}", id);
            gl::BindTexture(gl::TEXTURE_2D, id);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                image.width() as i32,
                image.height() as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                bytes_ptr,
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

            gl::BindTexture(gl::TEXTURE_2D, 0);

            log::trace!("Texture loaded: {}", id);
        }

        return Texture { id };
    }

    pub fn bind(&self) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }
}
