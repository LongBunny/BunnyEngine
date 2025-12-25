use crate::{Shader, Texture};
use glm::{Vec3, Vec4};
use num_traits::{One, Zero};
use std::sync::Arc;

pub enum MaterialProperty {
    Value(f32),
    Color(Vec3),
    Texture(Arc<Texture>),
}

pub enum NormalMap {
    None,
    Texture {
        texture: Arc<Texture>,
        scale: f32,
    }
}

pub struct Material {
    pub shader: Arc<Shader>,
    pub albedo: MaterialProperty,
    pub metallic: MaterialProperty,
    pub roughness: MaterialProperty,
    pub normal: NormalMap,
    pub emissive: MaterialProperty,
}

impl Material {
    pub(crate) fn apply(&self) {
        match &self.albedo {
            MaterialProperty::Value(_) => todo!(),
            MaterialProperty::Color(_) => todo!(),
            MaterialProperty::Texture(texture) => {
                texture.bind(0).unwrap();
            }
        }
        
        if let Some(specular_intensity_loc) = self.shader.get_uniform_location("specular_intensity") {
            self.shader.set_uniform(specular_intensity_loc, 1.0);
        }
        if let Some(tint_loc) = self.shader.get_uniform_location("tint") {
            self.shader.set_uniform(tint_loc, Vec4::one());
        }
    }
}

impl Default for Material {
    fn default() -> Self {
        Self {
            shader: Arc::new(Shader::default()),
            albedo: MaterialProperty::Color(Vec3::one()),
            metallic: MaterialProperty::Value(0.0),
            roughness: MaterialProperty::Value(1.0),
            normal: NormalMap::None,
            emissive: MaterialProperty::Color(Vec3::zero())
        }
    }
}