use std::cell::{Cell, RefCell};
use glm::{Mat4, Vec3};
use num_traits::One;

pub struct Transform {
    position: Vec3,
    scale: Vec3, // radians
    rotation: Vec3,
    
    model_matrix: RefCell<Mat4>,
    dirty: Cell<bool>,
}

impl Transform {
    pub fn new(position: Vec3, scale: Vec3, rotation: Vec3) -> Self {
        let result = Self { position, scale, rotation, model_matrix: RefCell::new(Mat4::one()), dirty: Cell::new(false) };
        result.calculate_model_matrix();
        result
    }
    
    pub fn pos(&self) -> Vec3 {
        self.position
    }
    
    pub fn set_pos(&mut self, position: Vec3) {
        self.position = position;
        self.dirty.set(true);
    }
    
    pub fn scale(&self) -> Vec3 {
        self.scale
    }
    
    pub fn set_scale(&mut self, scale: Vec3) {
        self.scale = scale;
        self.dirty.set(true);
    }
    
    pub fn rotation(&self) -> Vec3 {
        self.rotation
    }
    
    pub fn set_rotation(&mut self, rotation: Vec3) {
        self.rotation = rotation;
        self.dirty.set(true);
    }
    
    pub fn model_matrix(&self) -> Mat4 {
        if self.dirty.get() {
            self.calculate_model_matrix();
            self.dirty.set(false);
        }
        *self.model_matrix.borrow()
    }
    
    fn calculate_model_matrix(&self) {
        let mut m = Mat4::one();
        m = glm::ext::translate(&m, self.position);
        m = glm::ext::rotate(&m, self.rotation.x, glm::vec3(1.0, 0.0, 0.0));
        m = glm::ext::rotate(&m, self.rotation.y, glm::vec3(0.0, 1.0, 0.0));
        m = glm::ext::rotate(&m, self.rotation.z, glm::vec3(0.0, 0.0, 1.0));
        m = glm::ext::scale(&m, self.scale);
        *self.model_matrix.borrow_mut() = m;
    }
}