use std::sync::Arc;
use crate::{Mesh, Vertex};
use crate::renderer::material::Material;
use crate::renderer::transform::Transform;

pub struct RenderObject {
    transform: Transform,
    mesh: Arc<Mesh<Vertex>>,
    material: Arc<Material>
}

impl RenderObject {
    pub fn new(transform: Transform, mesh: Arc<Mesh<Vertex>>, material: Arc<Material>) -> Self {
        Self {
            transform,
            mesh,
            material
        }
    }
    
    pub fn transform(&self) -> &Transform {
        &self.transform
    }
    
    pub fn transform_mut(&mut self) -> &mut Transform {
        &mut self.transform
    }
    
    pub fn mesh(&self) -> &Mesh<Vertex> {
        &self.mesh
    }
    
    pub fn material(&self) -> &Material {
        self.material.as_ref()
    }
}