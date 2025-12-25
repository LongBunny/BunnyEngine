use crate::renderer::vertex_array::VAO;
use glm::{Vec2, Vec3};

#[derive(Debug)]
#[repr(C)]
pub struct Vertex {
    pub v: Vec3,
    pub vn: Vec3,
    pub vt: Vec2,
}

impl Vertex {
    pub fn new(v: Vec3, vn: Vec3, vt: Vec2) -> Self {
        Vertex {
            v,
            vn,
            vt
        }
    }
}

pub trait VertexLayout {
    fn setup_attributes(vao: &VAO);
}

impl VertexLayout for Vertex {
    fn setup_attributes(vao: &VAO) {
        vao.vertex_attrib_pointer(0, 3, 8, 0);
        vao.vertex_attrib_pointer(1, 3, 8, 3);
        vao.vertex_attrib_pointer(2, 2, 8, 6);
    }
}

