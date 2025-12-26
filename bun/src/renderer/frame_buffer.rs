use std::ptr::null;
use gl::types::{GLint, GLsizei};

pub struct Framebuffer {
    id: u32,
    screen_buffer_id: u32,
    depth_buffer_id: u32,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Result<Self, String> {
        let mut id = 0u32;
        let mut screen_buffer_id = 0u32;
        let mut depth_buffer_id = 0u32;
        unsafe {
            gl::GenFramebuffers(1, &mut id);
            gl::BindFramebuffer(gl::FRAMEBUFFER, id);
            
            gl::GenTextures(1, &mut screen_buffer_id);
            gl::BindTexture(gl::TEXTURE_2D, screen_buffer_id);
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA16F as GLint, width as GLsizei, height as GLsizei, 0, gl::RGBA, gl::FLOAT, null());
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as GLint);
            
            gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D, screen_buffer_id, 0);
            
            gl::GenRenderbuffers(1, &mut depth_buffer_id);
            gl::BindRenderbuffer(gl::RENDERBUFFER, depth_buffer_id);
            gl::RenderbufferStorage(gl::RENDERBUFFER, gl::DEPTH24_STENCIL8, width as GLsizei, height as GLsizei);
            
            gl::FramebufferRenderbuffer(gl::FRAMEBUFFER, gl::DEPTH_STENCIL_ATTACHMENT, gl::RENDERBUFFER, depth_buffer_id);
            
            // let draw_buffers = [gl::COLOR_ATTACHMENT0];
            // gl::DrawBuffers(1, draw_buffers.as_ptr());
            
            let framebuffer_status = gl::CheckFramebufferStatus(gl::FRAMEBUFFER);
            if framebuffer_status != gl::FRAMEBUFFER_COMPLETE {
                return Err(format!("Could not create framebuffer: {:#X?}", framebuffer_status));
            }
            
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }
        
        Ok(Self {
            id,
            screen_buffer_id,
            depth_buffer_id
        })
    }
    
    pub fn resize(&mut self, new_width: usize, new_height: usize) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.screen_buffer_id);
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA16F as GLint, new_width as GLsizei, new_height as GLsizei, 0, gl::RGBA, gl::FLOAT, null());
            
            gl::BindRenderbuffer(gl::RENDERBUFFER, self.depth_buffer_id);
            gl::RenderbufferStorage(gl::RENDERBUFFER, gl::DEPTH24_STENCIL8, new_width as GLsizei, new_height as GLsizei);
        }
    }
    
    pub fn bind(&self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.id);
        }
    }
    
    pub fn screen_texture_id(&self) -> u32 { self.screen_buffer_id }
    
    pub fn bind_default() {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }
    }
}

impl Drop for Framebuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteFramebuffers(1, &self.id);
        }
    }
}
