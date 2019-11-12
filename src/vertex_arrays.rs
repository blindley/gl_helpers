
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
