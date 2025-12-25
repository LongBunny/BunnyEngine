
pub struct VAO {
    id: u32,
}

impl VAO {
    pub fn new() -> Self {
        let mut id = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut id);
        }
        Self { id }
    }
    
    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }
    
    pub fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }
    
    pub fn vertex_attrib_pointer(&self, index: u32, num_components: i32, stride: u32, offset: u32) {
        unsafe {
            gl::VertexAttribPointer(
                index,
                num_components,
                gl::FLOAT,
                0,
                stride as i32 * size_of::<f32>() as i32,
                (offset * size_of::<f32>() as u32) as *const _,
            );
            gl::EnableVertexAttribArray(index);
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
