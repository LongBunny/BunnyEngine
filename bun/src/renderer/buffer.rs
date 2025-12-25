
#[derive(Copy, Clone, Debug)]
pub enum BufferUsage {
    StaticDraw,
    DynamicDraw,
    StreamDraw,
}

impl BufferUsage {
    fn to_gl(self) -> u32 {
        match self {
            BufferUsage::StaticDraw => gl::STATIC_DRAW,
            BufferUsage::DynamicDraw => gl::DYNAMIC_DRAW,
            BufferUsage::StreamDraw => gl::STREAM_DRAW,
        }
    }
}

pub struct Buffer<const TARGET: u32> {
    id: u32,
}

impl<const TARGET: u32> Buffer<TARGET> {
    pub fn new() -> Self {
        let mut id = 0;
        unsafe { gl::GenBuffers(1, &mut id) }
        Self { id }
    }

    pub fn bind(&self) {
        unsafe { gl::BindBuffer(TARGET, self.id) }
    }

    pub fn unbind(&self) {
        unsafe { gl::BindBuffer(TARGET, 0) }
    }
    
    pub fn buffer_data<T>(&self, data: &[T]) {
        self.buffer_data_with_usage(data, BufferUsage::StaticDraw);
    }

    pub fn buffer_data_with_usage<T>(&self, data: &[T], usage: BufferUsage) {
        unsafe {
            gl::BufferData(
                TARGET,
                (data.len() * size_of::<T>()) as isize,
                data.as_ptr() as *const _,
                usage.to_gl(),
            );
        }
    }
}

pub type VBO = Buffer<{ gl::ARRAY_BUFFER }>;
pub type EBO = Buffer<{ gl::ELEMENT_ARRAY_BUFFER }>;
pub type UBO = Buffer<{ gl::UNIFORM_BUFFER }>;
pub type SSBO = Buffer<{ gl::SHADER_STORAGE_BUFFER }>;

