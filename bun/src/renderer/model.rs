use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;
use glm::{Mat4, Vec3, Vec4};
use num_traits::{One, Zero};
use crate::renderer::mesh::Mesh;
use crate::renderer::shader::Shader;

pub struct Transform {
    position: Vec3,
    scale: Vec3, // radians
    rotation: Vec3,
    
    model_matrix: Mat4
}

impl Transform {
    pub fn new(position: Vec3, scale: Vec3, rotation: Vec3) -> Self {
        
        let mut result = Self { position, scale, rotation, model_matrix: Mat4::one() };
        
        result.calculate_model_matrix();
        
        result
    }
    
    pub fn pos(&self) -> Vec3 {
        self.position.clone()
    }
    
    pub fn set_pos(&mut self, position: Vec3) {
        self.position = position;
        self.calculate_model_matrix();
    }
    
    pub fn scale(&self) -> Vec3 {
        self.scale.clone()
    }
    
    pub fn set_scale(&mut self, scale: Vec3) {
        self.scale = scale;
        self.calculate_model_matrix();
    }
    
    pub fn rotation(&self) -> Vec3 {
        self.rotation.clone()
    }
    
    pub fn set_rotation(&mut self, rotation: Vec3) {
        self.rotation = rotation;
        self.calculate_model_matrix();
    }
    
    pub fn model_matrix(&self) -> Mat4 {
        self.model_matrix.clone()
    }
    
    fn calculate_model_matrix(&mut self) {
        let mut m = Mat4::one();
        m = glm::ext::translate(&m, self.position);
        m = glm::ext::rotate(&m, self.rotation.x, glm::vec3(1.0, 0.0, 0.0));
        m = glm::ext::rotate(&m, self.rotation.y, glm::vec3(0.0, 1.0, 0.0));
        m = glm::ext::rotate(&m, self.rotation.z, glm::vec3(0.0, 0.0, 1.0));
        m = glm::ext::scale(&m, self.scale);
        self.model_matrix = m;
    }
}

pub struct Model {
    mesh: Rc<RefCell<Mesh>>,
    shader: Rc<RefCell<Shader>>,
    
    transform: RefCell<Transform>,
    tint: Vec4,
}

impl Model {
    pub fn new(mesh: Rc<RefCell<Mesh>>, shader: Rc<RefCell<Shader>>) -> Self {
        Self {
            mesh,
            shader,
            transform: RefCell::new(Transform::new(Vec3::zero(), Vec3::one(), Vec3::zero())),
            tint: Vec4::one()
        }
    }
    
    pub fn with_transform(mesh: Rc<RefCell<Mesh>>, shader: Rc<RefCell<Shader>>, transform: Transform) -> Self {
        Self { mesh, shader, transform: RefCell::new(transform), tint: Vec4::one() }
    }
    
    pub fn render(&self, proj_mat: Mat4, view_mat: Mat4) {
        self.shader.borrow().bind();
        
        let mut shader = self.shader.borrow_mut();
        let proj_mat_loc = shader.get_uniform_location("proj_mat").unwrap();
        let view_mat_loc = shader.get_uniform_location("view_mat").unwrap();
        let model_mat_loc = shader.get_uniform_location("model_mat").unwrap();
        
        shader.set_uniform(proj_mat_loc, proj_mat);
        shader.set_uniform(view_mat_loc, view_mat);
        shader.set_uniform(model_mat_loc, self.transform().model_matrix());
        
        if let Some(camera_pos_loc) = shader.get_uniform_location("camera_pos") {
            shader.set_uniform(camera_pos_loc, self.transform().position);
        }
        if let Some(tint_loc) = shader.get_uniform_location("tint") {
            shader.set_uniform(tint_loc, self.tint);
        }
        
        
        self.mesh.borrow().render();
    }
    
    pub fn transform(&self) -> Ref<'_, Transform> {
        self.transform.borrow()
    }
    
    pub fn transform_mut(&self) -> RefMut<'_, Transform> {
        self.transform.borrow_mut()
    }
    
    pub fn tint(&self) -> Vec4 {
        self.tint
    }
    
    pub fn set_tint(&mut self, tint: Vec4) {
        self.tint = tint
    }
    
    pub fn mesh(&self) -> Ref<'_, Mesh> {
        self.mesh.borrow()
    }
    
    pub fn mesh_mut(&self) -> RefMut<'_, Mesh> {
        self.mesh.borrow_mut()
    }
    
    pub fn shader(&self) -> Ref<'_, Shader> {
        self.shader.borrow()
    }
    
    pub fn shader_mut(&self) -> RefMut<'_, Shader> {
        self.shader.borrow_mut()
    }
}