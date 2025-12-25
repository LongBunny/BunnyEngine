use crate::renderer::mesh::Mesh;
use crate::Vertex;
use glm::{Mat4, Vec3};
use num_traits::{One, Zero};
use std::cell::{Cell, Ref, RefCell, RefMut};
use std::rc::Rc;

pub struct Transform {
    position: Vec3,
    scale: Vec3, // radians
    rotation: Vec3,
    
    model_matrix: RefCell<Mat4>,
    dirty: Cell<bool>,
}

impl Transform {
    pub fn new(position: Vec3, scale: Vec3, rotation: Vec3) -> Self {
        let mut result = Self { position, scale, rotation, model_matrix: RefCell::new(Mat4::one()), dirty: Cell::new(false) };
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

pub struct Renderable {
    mesh: Rc<RefCell<Mesh<Vertex>>>,
    transform: RefCell<Transform>,
}

impl Renderable {
    pub fn new(mesh: Rc<RefCell<Mesh<Vertex>>>) -> Self {
        Self {
            mesh,
            transform: RefCell::new(Transform::new(Vec3::zero(), Vec3::one(), Vec3::zero())),
        }
    }
    
    pub fn with_transform(mesh: Rc<RefCell<Mesh<Vertex>>>, transform: Transform) -> Self {
        Self { mesh, transform: RefCell::new(transform) }
    }
    
    pub fn transform(&self) -> Ref<'_, Transform> {
        self.transform.borrow()
    }
    
    pub fn transform_mut(&self) -> RefMut<'_, Transform> {
        self.transform.borrow_mut()
    }
    
    pub fn mesh(&self) -> Ref<'_, Mesh<Vertex>> {
        self.mesh.borrow()
    }
    
    pub fn mesh_mut(&self) -> RefMut<'_, Mesh<Vertex>> {
        self.mesh.borrow_mut()
    }
}