type Result<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Clone, Default)]
pub struct ShaderCode {
    code: [Option<String>;6],
}

#[derive(Debug, Clone, Copy)]
pub enum ShaderType {
    Vertex, Fragment, TessControl, TessEval, Geometry, Compute,
}

impl ShaderType {
    pub fn each() -> [ShaderType;6] {
        use self::ShaderType::*;
        [Vertex, Fragment, TessControl, TessEval, Geometry, Compute,]
    }

    pub fn as_gl_enum(self) -> u32 {
        use self::ShaderType::*;
        match self {
            Vertex => gl::VERTEX_SHADER,
            Fragment => gl::FRAGMENT_SHADER,
            TessControl => gl::TESS_CONTROL_SHADER,
            TessEval => gl::TESS_EVALUATION_SHADER,
            Geometry => gl::GEOMETRY_SHADER,
            Compute => gl::COMPUTE_SHADER,
        }
    }

    pub fn short_name(self) -> &'static str {
        use self::ShaderType::*;
        match self {
            Vertex => "vertex",
            Fragment => "fragment",
            TessControl => "tess control",
            TessEval => "tess eval",
            Geometry => "geometry",
            Compute => "compute",
        }
    }
}

impl ShaderCode {
    pub fn new() -> ShaderCode {
        ShaderCode::default()
    }
}

impl std::ops::Index<ShaderType> for ShaderCode {
    type Output = Option<String>;
    fn index(&self, ty: ShaderType) -> &Option<String> {
        &self.code[ty as usize]
    }
}

impl std::ops::IndexMut<ShaderType> for ShaderCode {
    fn index_mut(&mut self, ty: ShaderType) -> &mut Option<String> {
        &mut self.code[ty as usize]
    }
}

pub struct ProgramBuilder {
    code: ShaderCode,
}

impl ProgramBuilder {
    pub fn new() -> ProgramBuilder {
        ProgramBuilder {
            code: ShaderCode::new(),
        }
    }

    pub fn add_vertex_shader_code<T>(&mut self, code : T) -> &mut ProgramBuilder
        where T : Into<String>
    {
        self.code[ShaderType::Vertex] = Some(code.into());
        self
    }

    pub fn add_fragment_shader_code<T>(&mut self, code : T) -> &mut ProgramBuilder
        where T : Into<String>
    {
        self.code[ShaderType::Fragment] = Some(code.into());
        self
    }

    pub fn add_geometry_shader_code<T>(&mut self, code : T) -> &mut ProgramBuilder
        where T : Into<String>
    {
        self.code[ShaderType::Geometry] = Some(code.into());
        self
    }

    pub fn add_tess_control_shader_code<T>(&mut self, code : T) -> &mut ProgramBuilder
        where T : Into<String>
    {
        self.code[ShaderType::TessControl] = Some(code.into());
        self
    }

    pub fn add_tess_eval_shader_code<T>(&mut self, code : T) -> &mut ProgramBuilder
        where T : Into<String>
    {
        self.code[ShaderType::TessEval] = Some(code.into());
        self
    }

    pub fn add_compute_shader_code<T>(&mut self, code : T) -> &mut ProgramBuilder
        where T : Into<String>
    {
        self.code[ShaderType::Compute] = Some(code.into());
        self
    }

    pub fn add_shader_code(&mut self, code : &ShaderCode) -> &mut ProgramBuilder {
        for i in ShaderType::each().iter() {
            if let Some(ref code) = code[*i] {
                self.code[*i] = Some(code.clone());
            }
        }
        self
    }

    pub fn build(&self) -> Result<u32> {
        let mut shaders = Vec::new();
        for shader_type in ShaderType::each().iter() {
            if let Some(ref code) = self.code[*shader_type] {
                shaders.push(compile_shader(&code, *shader_type)?);
            }
        }

        create_program(&shaders, true)
    }
}

pub fn compile_shader(code : &str, shader_type : ShaderType) -> Result<u32> {
    unsafe {
        let gl_type = shader_type.as_gl_enum();
        let shader = gl::CreateShader(gl_type);
        let len = code.len() as i32;
        let code_ptr = code.as_ptr() as *const std::os::raw::c_char;
        gl::ShaderSource(shader, 1, &code_ptr, &len);
        gl::CompileShader(shader);

        let mut success = 0;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
        if success == 0 {
            let mut loglen = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut loglen);
            let buffer_len = (loglen - 1) as usize;
            let mut buffer : Vec<u8> = Vec::with_capacity(buffer_len + 1);
            gl::GetShaderInfoLog(shader, loglen, std::ptr::null_mut(),
                buffer.as_ptr() as *mut std::os::raw::c_char);
            buffer.set_len(loglen as usize - 1);
            let log = String::from_utf8(buffer).unwrap();            
            let shader_name = shader_type.short_name();
            let message = format!("error in {} shader : {}", shader_name, log);
            gl::DeleteShader(shader);
            Err(message.into())
        } else {
            Ok(shader)
        }
    }
}

pub fn create_program(shaders : &[u32], delete_shaders : bool) -> Result<u32> {
    unsafe {
        let program = gl::CreateProgram();
        for &shader in shaders {
            gl::AttachShader(program, shader);
        }
        gl::LinkProgram(program);

        for &shader in shaders {
            gl::DetachShader(program, shader);
            if delete_shaders {
                gl::DeleteShader(shader);
            }
        }

        let mut success = 0;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
        if success == 0 {
            let mut loglen = 0;
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut loglen);
            let buffer_len = (loglen - 1) as usize;
            let mut buffer : Vec<u8> = Vec::with_capacity(buffer_len + 1);
            gl::GetProgramInfoLog(program, loglen, std::ptr::null_mut(),
                buffer.as_ptr() as *mut std::os::raw::c_char);
            buffer.set_len(loglen as usize - 1);
            let log = String::from_utf8(buffer).unwrap();
            let message = format!("shader program linking error: {}", log);
            gl::DeleteProgram(program);
            Err(message.into())
        } else {
            Ok(program)
        }
    }
}

pub fn get_attribute_location(program: u32, name: &str) -> Result<i32> {
    let name_cstr = std::ffi::CString::new(name)?;
    get_attribute_location_cstr(program, &name_cstr)
}

pub fn get_attribute_location_cstr(program: u32, name: &std::ffi::CStr) -> Result<i32> {
    let loc = unsafe { gl::GetAttribLocation(program, name.as_ptr()) };
    if loc == -1 {
        Err(format!("could not find attribute {}", name.to_string_lossy()))
    } else {
        Ok(loc)
    }
}

pub fn get_uniform_location(program: u32, name: &str) -> Result<i32> {
    let name_cstr = std::ffi::CString::new(name)?;
    get_uniform_location_cstr(program, &name_cstr)
}

pub fn get_uniform_location_cstr(program: u32, name: &std::ffi::CStr) -> Result<i32> {
    let loc = unsafe { gl::GetUniformLocation(program, name.as_ptr()) };
    if loc == -1 {
        Err(format!("could not find uniform {}", name.to_string_lossy()))
    } else {
        Ok(loc)
    }
}

