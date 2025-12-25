use std::ffi::CString;
use crate::{Shader, Texture};
use glm::{Vec3, Vec4};
use num_traits::{One, Zero};
use std::sync::Arc;
use crate::renderer::buffer::UBO;

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
    
    pub ubo: UBO,
}

impl Material {
    pub(crate) fn apply(&self) {
        
        let ubo_data = MaterialUBO::from_material(self);
        
        if let MaterialProperty::Texture(texture) = &self.albedo {
            texture.bind(0).unwrap();
        };
        
        if let NormalMap::Texture {texture, ..} = &self.normal {
            texture.bind(1).unwrap();
        }
        
        self.ubo.bind();
        unsafe {
            let location = CString::new("MaterialUBO").unwrap();
            let block_index = gl::GetUniformBlockIndex(self.shader.id(), location.as_ptr());
            if block_index != gl::INVALID_INDEX {
                gl::UniformBlockBinding(self.shader.id(), block_index, 2);
                gl::BindBufferBase(gl::UNIFORM_BUFFER, 2, self.ubo.id());
                self.ubo.buffer_data(&[ubo_data]);
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
        let ubo = UBO::new();
        ubo.bind();
        ubo.prepare_data::<MaterialUBO>();
        Self {
            shader: Arc::new(Shader::default()),
            albedo: MaterialProperty::Color(Vec3::one()),
            metallic: MaterialProperty::Value(0.0),
            roughness: MaterialProperty::Value(1.0),
            normal: NormalMap::None,
            emissive: MaterialProperty::Color(Vec3::zero()),
            
            ubo
        }
    }
}

#[repr(C, align(16))]
pub struct MaterialUBO {
    albedo_color: Vec4,
    emissive_color: Vec4,
    
    metallic: f32,
    roughness: f32,
    normal_scale: f32,
    
    albedo_has_texture: i32,
    normal_has_texture: i32,
    emissive_has_texture: i32,
    metallic_has_texture: i32,
    roughness_has_texture: i32,
}

impl MaterialUBO {
    fn from_material(material: &Material) -> Self {
        let (albedo_color, albedo_has_texture) = get_material_property(&material.albedo);
        let (normal_has_texture, normal_scale) = match &material.normal {
            NormalMap::None => (0, 1.0f32),
            NormalMap::Texture { scale, .. } => (1, *scale)
        };
        
        Self {
            albedo_color,
            emissive_color: Vec4::zero(),
            metallic: 0.0,
            roughness: 0.0,
            normal_scale,
            
            albedo_has_texture,
            normal_has_texture,
            emissive_has_texture: 0,
            metallic_has_texture: 0,
            roughness_has_texture: 0,
        }
    }
}

fn get_material_property(property: &MaterialProperty) -> (Vec4, i32) {
    match property {
        MaterialProperty::Color(c) => {
            (Vec4::new(c.x, c.y, c.z, 1.0), 0)
        }
        MaterialProperty::Texture(_) => {
            (Vec4::new(1.0, 1.0, 1.0, 1.0), 1)
        },
        _ => panic!("Albedo should never have a value")
    }
}