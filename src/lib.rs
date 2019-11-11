use std;
pub use gl;

mod shaders;
pub use shaders::*;

#[derive(Debug, Clone)]
pub struct Error {
    message: String,
}

impl Error {
    pub fn new<S: Into<String>>(message: S) -> Error {
        Error {
            message: message.into()
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for Error{}

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

pub fn create_single_buffer_vertex_array(buffer : u32, components : &[i32]) -> Result<u32, Box<dyn std::error::Error>> {
    unsafe {
        let mut vertex_array = 0;
        gl::GenVertexArrays(1, &mut vertex_array);
        gl::BindVertexArray(vertex_array);
        gl::BindBuffer(gl::ARRAY_BUFFER, buffer);
        let total_components : i32 = components.iter().sum();
        let stride = total_components * std::mem::size_of::<f32>() as i32;
        let mut offset : usize = 0;
        for (index, &comp) in components.iter().enumerate() {
            let index = index as u32;
            let offset_ptr = offset as *mut std::os::raw::c_void;
            gl::VertexAttribPointer(index, comp, gl::FLOAT, gl::FALSE,
                stride, offset_ptr);
            gl::EnableVertexAttribArray(index);
            offset = offset + (comp as usize) * std::mem::size_of::<f32>();
        }

        Ok(vertex_array)
    }
}
