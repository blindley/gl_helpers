
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BufferUsage {
    StreamDraw, StreamRead, StreamCopy,
    StaticDraw, StaticRead, StaticCopy,
    DynamicDraw, DynamicRead, DynamicCopy,
}

impl BufferUsage {
    pub fn as_gl_enum(self) -> u32 {
        match self {
            BufferUsage::StreamDraw => gl::STREAM_DRAW,
            BufferUsage::StreamRead => gl::STREAM_READ,
            BufferUsage::StreamCopy => gl::STREAM_COPY,
            BufferUsage::StaticDraw => gl::STATIC_DRAW,
            BufferUsage::StaticRead => gl::STATIC_READ,
            BufferUsage::StaticCopy => gl::STATIC_COPY,
            BufferUsage::DynamicDraw => gl::DYNAMIC_DRAW,
            BufferUsage::DynamicRead => gl::DYNAMIC_READ,
            BufferUsage::DynamicCopy => gl::DYNAMIC_COPY,
        }
    }
}

pub fn create_buffer<T>(data : &[T], usage: BufferUsage) -> Result<u32, Box<dyn std::error::Error>> {
    unsafe {
        let mut buffer = 0;
        gl::CreateBuffers(1, &mut buffer);
        named_buffer_data(buffer, data, usage);
        Ok(buffer)
    }
}

pub fn named_buffer_data<T>(buffer: u32, data: &[T], usage: BufferUsage) {
    let size = (data.len() * std::mem::size_of::<T>()) as isize;
    unsafe {
        gl::NamedBufferData(buffer, size, data.as_ptr() as _, usage.as_gl_enum());
    }
}