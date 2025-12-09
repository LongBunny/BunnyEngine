use gl::types::{GLint, GLuint};
use std::ffi::c_void;
use std::path::Path;
use glm::Vec3;

pub struct Texture {
    texture_id: u32,
    width: i32,
    height: i32,
}

impl Texture {
    pub fn new<P>(path: P) -> Result<Self, String>
    where
        P: AsRef<Path>,
    {
        println!("loading texture: {}", path.as_ref().to_string_lossy());
        let image = image::open(path).unwrap();
        let image = image.flipv();
        let width = image.width() as i32;
        let height = image.height() as i32;
        let data = image.to_rgba8();

        let texture_id = Self::create_and_upload_texture(width, height, data.as_ptr() as *const _)?;

        Ok(Self {
            width,
            height,
            texture_id,
        })
    }
    
    /// not really working rn
    pub fn from_color(color: Vec3) -> Result<Self, String> {
        let width = 1;
        let height = 1;
        if color.x < 0.0 || color.x > 1.0
            || color.y < 0.0 || color.y > 1.0
            || color.z < 0.0 || color.z > 1.0 {
            return Err(String::from(format!("Color has to be in range 0..1: Color: {:?}", color)));
        }
        let r = (color.x * 255.0) as u8;
        let g = (color.y * 255.0) as u8;
        let b = (color.z * 255.0) as u8;
        let a = 255u8;
        let data: [u8; 4] = [r, g, b, a];
        
        println!("data: {data:?}");
        let texture_id = Self::create_and_upload_texture(width, height, data.as_ptr() as *const _)?;
        
        Ok(Self {
            width,
            height,
            texture_id
        })
    }

    pub fn bind(&self) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.texture_id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }
    
    fn create_and_upload_texture(width: i32, height: i32, data: *const c_void) -> Result<u32, String> {
        let mut texture_id: u32 = 0;
        unsafe {
            gl::GenTextures(1, &mut texture_id);
            gl::BindTexture(gl::TEXTURE_2D, texture_id);
            
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as GLint);
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_WRAP_S,
                gl::CLAMP_TO_BORDER as GLint,
            );
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_WRAP_T,
                gl::CLAMP_TO_BORDER as GLint,
            );
            
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as GLint,
                width,
                height,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                data,
            );
            
            let err = gl::GetError();
            if err != gl::NO_ERROR {
                return Err(format!("Error loading texture: {}", err));
            }
        }
        
        Ok(texture_id)
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, [self.texture_id].as_ptr());
        }
    }
}
