use gl::types::GLint;
use glm::Vec3;
use std::ffi::c_void;
use std::path::Path;

pub struct Texture {
    texture_id: u32,
    width: i32,
    height: i32,
}

pub struct ImageData {
    pub width: i32,
    pub height: i32,
    pub channels: u8,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Copy)]
pub enum TextureUsage {
    Albedo,      // color, sRGB
    Normal,      // vector data, linear
    Data,        // roughness, metallic, AO, etc. (linear)
    Emissive,    // usually sRGB
}

#[derive(Debug, Clone, Copy)]
pub struct TextureSpec {
    pub usage: TextureUsage,
    pub min_filter: u32,
    pub mag_filter: u32,
    pub wrap_s: u32,
    pub wrap_t: u32,
    pub generate_mipmaps: bool,
}

impl Texture {
    pub fn new<P>(path: P, spec: TextureSpec) -> Result<Self, String>
    where
        P: AsRef<Path>,
    {
        let file_name = String::from(path.as_ref().to_string_lossy());
        println!("loading texture: {}", file_name);
        let image = image::open(path).unwrap();
        let image = image.flipv();
        
        let rgba = image.to_rgba8();
        let width = rgba.width() as i32;
        let height = rgba.height() as i32;
        
        let image_data = ImageData {
            width,
            height,
            channels: 4,
            data: rgba.into_raw(),
        };

        let texture_id = Self::create_texture(image_data, spec)?;

        Ok(Self {
            width,
            height,
            texture_id
        })
    }
    
    pub fn empty(width: usize, height: usize, spec: TextureSpec) -> Result<Self, String> {
        let width = width as i32;
        let height = height as i32;
        let data = vec![0; (width * height * 4) as usize];
        let image_data = ImageData {
            width,
            height,
            channels: 4,
            data
        };
        
        let texture_id = Self::create_texture(image_data, spec)?;
        
        Ok(Self {
            width,
            height,
            texture_id
        })
    }

    pub fn bind(&self, unit: u32) -> Result<(), String> {
        const MAX_TEXTURE_UNIT: u32 = 31;
        if unit > MAX_TEXTURE_UNIT {
            return Err(format!("Texture unit is too big. {unit} > {MAX_TEXTURE_UNIT}"))
        }
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + unit);
            gl::BindTexture(gl::TEXTURE_2D, self.texture_id);
        }
        Ok(())
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }
    
    fn create_texture(image_data: ImageData, spec: TextureSpec) -> Result<u32, String> {
        let mut texture_id: u32 = 0;
        unsafe {
            gl::GenTextures(1, &mut texture_id);
            gl::BindTexture(gl::TEXTURE_2D, texture_id);
            
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, spec.min_filter as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, spec.mag_filter as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, spec.wrap_s as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, spec.wrap_t as i32);
            gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                spec.internal_format(true) as GLint,
                image_data.width,
                image_data.height,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                image_data.data.as_ptr() as *const c_void,
            );
            
            gl::GenerateMipmap(gl::TEXTURE_2D);
            
            let err = gl::GetError();
            if err != gl::NO_ERROR {
                return Err(format!("Error loading texture: {:#X}", err));
            }
        }
        
        Ok(texture_id)
    }
    
    pub fn id(&self) -> u32 { self.texture_id }
    pub fn width(&self) -> i32 { self.width }
    pub fn height(&self) -> i32 { self.height }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, [self.texture_id].as_ptr());
        }
    }
}


impl TextureSpec {
    pub fn albedo() -> Self {
        Self {
            usage: TextureUsage::Albedo,
            min_filter: gl::LINEAR_MIPMAP_LINEAR,
            mag_filter: gl::LINEAR,
            wrap_s: gl::REPEAT,
            wrap_t: gl::REPEAT,
            generate_mipmaps: true,
        }
    }
    
    pub fn normal() -> Self {
        Self {
            usage: TextureUsage::Normal,
            min_filter: gl::LINEAR_MIPMAP_LINEAR,
            mag_filter: gl::LINEAR,
            wrap_s: gl::REPEAT,
            wrap_t: gl::REPEAT,
            generate_mipmaps: true,
        }
    }
    
    pub fn data() -> Self {
        Self {
            usage: TextureUsage::Data,
            min_filter: gl::LINEAR_MIPMAP_LINEAR,
            mag_filter: gl::LINEAR,
            wrap_s: gl::REPEAT,
            wrap_t: gl::REPEAT,
            generate_mipmaps: true,
        }
    }
    
    pub fn emissive() -> Self {
        Self {
            usage: TextureUsage::Emissive,
            min_filter: gl::LINEAR_MIPMAP_LINEAR,
            mag_filter: gl::LINEAR,
            wrap_s: gl::REPEAT,
            wrap_t: gl::REPEAT,
            generate_mipmaps: true,
        }
    }
    
    pub fn internal_format(&self, has_alpha: bool) -> u32 {
        match self.usage {
            TextureUsage::Albedo | TextureUsage::Emissive => {
                if has_alpha {
                    gl::SRGB8_ALPHA8
                } else {
                    gl::SRGB8
                }
            }
            TextureUsage::Normal | TextureUsage::Data => {
                if has_alpha {
                    gl::RGBA8
                } else {
                    gl::RGB8
                }
            }
        }
    }
}

fn pixel_format(channels: u8) -> u32 {
    match channels {
        1 => gl::RED,
        2 => gl::RG,
        3 => gl::RGB,
        4 => gl::RGBA,
        _ => panic!("Unsupported channel count"),
    }
}