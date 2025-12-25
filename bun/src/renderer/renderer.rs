use crate::renderer::render_object::RenderObject;
use crate::renderer::transform::Transform;
use crate::{Camera, Shader};
use glm::Vec4;

pub struct Renderer {
    current_shader: Option<u32>
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            current_shader: None
        }
    }
    
    pub fn begin_frame(&mut self, clear_color: Vec4) {
        unsafe {
            gl::ClearColor(clear_color.x, clear_color.y, clear_color.z, clear_color.w);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
        
    }
    
    pub fn render(
        &mut self,
        object: &RenderObject,
        camera: &Camera
    ) {
        let material = object.material();
        
        let shader_id = material.shader.id();
        if self.current_shader != Some(shader_id) {
            material.shader.bind();
            self.current_shader = Some(material.shader.id());
        }
        
        self.set_camera_uniforms(camera, material.shader.as_ref());
        self.set_model_uniforms(&object.transform(), material.shader.as_ref());
        
        
        material.apply();
        
        object.mesh().render();
    }
    
    pub fn end_frame(&mut self) {
    
    }
    
    fn set_camera_uniforms(&self, camera: &Camera, shader: &Shader) {
        if let Some(loc) = shader.get_uniform_location("proj_mat") {
            shader.set_uniform(loc, camera.projection());
        }
        if let Some(loc) = shader.get_uniform_location("view_mat") {
            shader.set_uniform(loc, camera.view());
        }
        if let Some(loc) = shader.get_uniform_location("camera_pos") {
            shader.set_uniform(loc, camera.position());
        }
    }
    
    fn set_model_uniforms(&self, transform: &Transform, shader: &Shader) {
        if let Some(loc) = shader.get_uniform_location("model_mat") {
            shader.set_uniform(loc, transform.model_matrix());
        }
    }
}