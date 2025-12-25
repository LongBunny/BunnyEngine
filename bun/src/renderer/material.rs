use std::sync::Arc;
use glm::Vec3;
use num_traits::{One, Zero};
use crate::renderer::material::MaterialPropertyType::Value;
use crate::Texture;

pub enum MaterialPropertyType<T> {
    Texture(Arc<Texture>),
    Value(T)
}

pub struct Material {
    albedo: MaterialPropertyType<Vec3>,
    metallic: MaterialPropertyType<f32>,
    roughness: MaterialPropertyType<f32>,
    normal: MaterialPropertyType<Vec3>,
    normal_scale: f32,
    emissive: MaterialPropertyType<Vec3>,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            albedo: Value(Vec3::one()),
            metallic: Value(0.0),
            roughness: Value(1.0),
            normal: Value(Vec3::one()),
            normal_scale: 1.0,
            emissive: Value(Vec3::zero())
        }
    }
}