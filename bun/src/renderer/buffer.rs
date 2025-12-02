use crate::renderer::vertex::Vertex;

pub trait Buffer {
    fn new() -> Self;
    fn bind(&self);
    fn unbind(&self);
}

pub struct VBO {
    id: u32,
}

pub struct VAO {
    id: u32,
}

pub struct EBO {
    id: u32,
}

impl VBO {
    pub fn buffer_data(&self, vertices: &Vec<Vertex>) {
        unsafe {
            gl::BufferData(gl::ARRAY_BUFFER, (size_of::<Vertex>() * vertices.len()) as isize, vertices.as_ptr() as *const _, gl::STATIC_DRAW);
        }
    }
}

impl VAO {
    pub fn vertex_attrib_pointer(&self, index: u32, num_components: i32, stride: u32, offset: u32) {
        unsafe {
            gl::VertexAttribPointer(index, num_components, gl::FLOAT, 0, stride as i32 * size_of::<f32>() as i32, (offset * size_of::<f32>() as u32) as *const _);
            gl::EnableVertexAttribArray(index);
        }
    }
}

impl EBO {
    pub fn buffer_data(&self, indices: &Vec<u32>) {
        unsafe {
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, (size_of::<u32>() * indices.len()) as isize, indices.as_ptr() as *const _, gl::STATIC_DRAW);
        }
    }
}


impl Buffer for VBO {
    fn new() -> Self {
        let mut id = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);
        }
        Self {
            id
        }
    }
    
    fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
        }
    }
    
    fn unbind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }
}

impl Buffer for VAO {
    fn new() -> Self {
        let mut id = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut id);
        }
        Self {
            id
        }
    }
    
    fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }
    
    fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }
}

impl Buffer for EBO {
    fn new() -> Self {
        let mut id = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);
        }
        Self {
            id
        }
    }
    
    fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.id);
        }
    }
    
    fn unbind(&self) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        }
    }
}

impl Drop for VBO {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &mut self.id);
        }
    }
}

impl Drop for VAO {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &mut self.id);
        }
    }
}

impl Drop for EBO {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &mut self.id);
        }
    }
}