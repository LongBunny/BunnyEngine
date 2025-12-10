use glm::{Vec2, Vec3, Vec4};

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
