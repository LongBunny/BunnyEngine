use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::ffi::CString;
use std::path::PathBuf;
use std::ptr::{null, null_mut};

pub trait UniformValue {
    fn set_uniform(&self, location: i32);
}

pub struct Shader {
    id: Cell<u32>,
    
    uniforms: RefCell<HashMap<String, i32>>,
    vertex_path: Option<PathBuf>,
    fragment_path: Option<PathBuf>,
}

enum ShaderType {
    Vertex,
    Fragment,
}

impl Shader {
    pub fn new(vertex_path: &PathBuf, fragment_path: &PathBuf) -> Result<Self, String> {
        let vertex = Self::create_shader(ShaderType::Vertex, vertex_path)?;
        let fragment = Self::create_shader(ShaderType::Fragment, fragment_path)?;

        let program = Self::create_program(vertex, fragment)?;

        Ok(Self {
            id: Cell::new(program),
            uniforms: RefCell::new(HashMap::new()),
            vertex_path: Some(vertex_path.clone()),
            fragment_path: Some(fragment_path.clone()),
        })
    }
    
    pub fn from_source(vertex_src: String, fragment_src: String) -> Result<Self, String> {
        let vertex = Self::create_shader_from_source(ShaderType::Vertex, vertex_src)?;
        let fragment = Self::create_shader_from_source(ShaderType::Fragment, fragment_src)?;
        
        let program = Self::create_program(vertex, fragment)?;
        
        Ok(Self {
            id: Cell::new(program),
            uniforms: RefCell::new(HashMap::new()),
            vertex_path: None,
            fragment_path: None,
        })
    }

    pub fn id(&self) -> u32 {
        self.id.get()
    }

    pub fn bind(&self) {
        unsafe {
            gl::UseProgram(self.id.get());
        }
    }
    
    pub fn unbind(&self) {
        unsafe {
            gl::UseProgram(0);
        }
    }

    pub fn get_uniform_location(&self, name: &str) -> Option<i32> {
        if let Some(&location) = self.uniforms.borrow().get(name) {
            return Some(location);
        }
        
        let location = unsafe {
            let c_name = CString::new(name).unwrap();
            gl::GetUniformLocation(self.id.get(), c_name.as_ptr())
        };
        
        if location == -1 {
            // eprintln!("Warning: Uniform '{name}' not found.");
            return None;
        }
        
        self.uniforms.borrow_mut().insert(name.to_string(), location);
        Some(location)
    }

    pub fn set_uniform<T: UniformValue>(&self, location: i32, value: T) {
        value.set_uniform(location);
    }
    
    
    pub fn reload(&self) -> Result<(), String> {
        if self.vertex_path == None || self.fragment_path == None {
            return Err("No shader source".to_string());
        }
        let vertex_path = self.vertex_path.clone().unwrap();
        let fragment_path = self.fragment_path.clone().unwrap();
        let vertex = Self::create_shader(ShaderType::Vertex, &vertex_path)?;
        let fragment = Self::create_shader(ShaderType::Fragment, &fragment_path)?;
        let new_id = Self::create_program(vertex, fragment)?;
        unsafe {
            gl::DeleteProgram(self.id.get());
        }
        self.id.set(new_id);
        
        let mut uniforms = self.uniforms.borrow_mut();
        uniforms.clear();
        
        Ok(())
    }

    fn create_shader(shader_type: ShaderType, path: &PathBuf) -> Result<u32, String> {
        let shader_src = std::fs::read_to_string(path)
            .map_err(|e| format!("Failed to read {:?}: {}", path, e))?;
        Self::create_shader_from_source(shader_type, shader_src)
    }
    
    fn create_shader_from_source(shader_type: ShaderType, shader_src: String) -> Result<u32, String> {
        let shader_src = CString::new(shader_src)
            .map_err(|e| format!("Failed to create CString: {}", e))?;
        unsafe {
            let shader_type = match shader_type {
                ShaderType::Vertex => gl::VERTEX_SHADER,
                ShaderType::Fragment => gl::FRAGMENT_SHADER,
            };
            let shader = gl::CreateShader(shader_type);
            gl::ShaderSource(shader, 1, &shader_src.as_ptr(), null());
            gl::CompileShader(shader);
            
            let mut success = 0;
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
            if success != 1 {
                const LOG_SIZE: usize = 512;
                let mut log = [0i8; LOG_SIZE];
                gl::GetShaderInfoLog(shader, LOG_SIZE as i32, null_mut(), log.as_mut_ptr());
                let log_str = std::ffi::CStr::from_ptr(log.as_ptr()).to_string_lossy();
                return Err(std::format!("Could not compile shader: {}", log_str));
            }
            
            Ok(shader)
        }
    }

    fn create_program(vertex: u32, fragment: u32) -> Result<u32, String> {
        unsafe {
            let program = gl::CreateProgram();

            gl::AttachShader(program, vertex);
            gl::AttachShader(program, fragment);
            gl::LinkProgram(program);

            let mut success = 0;
            gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
            if success != 1 {
                const LOG_SIZE: usize = 512;
                let mut log = [0i8; LOG_SIZE];
                gl::GetProgramInfoLog(program, LOG_SIZE as i32, null_mut(), log.as_mut_ptr());
                let log_str = std::ffi::CStr::from_ptr(log.as_ptr()).to_string_lossy();
                return Err(std::format!("Could not link program: {}", log_str));
            }

            gl::DeleteShader(vertex);
            gl::DeleteShader(fragment);

            Ok(program)
        }
    }
}

impl Default for Shader {
    fn default() -> Self {
        Self::from_source(
            include_str!("../res/shaders/default.vert").to_string(),
            include_str!("../res/shaders/default.frag").to_string()
        ).unwrap()
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id.get());
        }
    }
}

// ----------- uniforms -----------
#[allow(unsafe_op_in_unsafe_fn)]
// Mat4
impl UniformValue for glm::Mat4 {
    fn set_uniform(&self, location: i32) {
        unsafe {
            gl::UniformMatrix4fv(location, 1, gl::FALSE, self.as_array().as_ptr() as *const _);
        }
    }
}

#[allow(unsafe_op_in_unsafe_fn)]
// Mat3
impl UniformValue for glm::Mat3 {
    fn set_uniform(&self, location: i32) {
        unsafe {
            gl::UniformMatrix3fv(location, 1, gl::FALSE, self.as_array().as_ptr() as *const _);
        }
    }
}

#[allow(unsafe_op_in_unsafe_fn)]
// Vec3
impl UniformValue for glm::Vec3 {
    fn set_uniform(&self, location: i32) {
        unsafe {
            gl::Uniform3fv(location, 1, self.as_array().as_ptr() as *const _);
        }
    }
}

#[allow(unsafe_op_in_unsafe_fn)]
// Vec4
impl UniformValue for glm::Vec4 {
    fn set_uniform(&self, location: i32) {
        unsafe {
            gl::Uniform4fv(location, 1, self.as_array().as_ptr() as *const _);
        }
    }
}

#[allow(unsafe_op_in_unsafe_fn)]
// f32
impl UniformValue for f32 {
    fn set_uniform(&self, location: i32) {
        unsafe {
            gl::Uniform1f(location, *self);
        }
    }
}

#[allow(unsafe_op_in_unsafe_fn)]
// i32
impl UniformValue for i32 {
    fn set_uniform(&self, location: i32) {
        unsafe {
            gl::Uniform1i(location, *self);
        }
    }
}
