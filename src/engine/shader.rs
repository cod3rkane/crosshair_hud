use gl::types::*;
use std::ffi::CString;
use std::fs;

pub struct Shader {
    id: GLuint,
}

impl Shader {
    pub fn use_shader(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    pub fn delete_shader(&self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }

    pub fn get_uniform_loc_from_name(&self, name: &str) -> GLint {
        let location: GLint;
        let str = CString::new(name).expect("CString::new failed!");

        unsafe {
            location = gl::GetUniformLocation(self.id, str.as_ptr());
        }

        location
    }

    pub fn modify_uniform_mat4_value(&self, location: i32, size: i32, data: *const f32) {
        unsafe {
            gl::UniformMatrix4fv(location, size, gl::FALSE, data);
        }
    }

    pub fn set_mat4(&self, uniform_name: &str, size: GLsizei, data: *const GLfloat) {
        let id = self.get_uniform_loc_from_name(uniform_name);
        self.modify_uniform_mat4_value(id, size, data);
    }
}

fn read_from_file(path: &str) -> CString {
    let shader_str =
        fs::read_to_string(path).expect("Something went wrong while reading shader file.");
    let shader_cstr = CString::new(shader_str)
        .unwrap_or_else(|error| panic!("Error to convert string to Cstring {:?}", error));

    shader_cstr
}

fn create_available_space_cstring_with_len(len: usize) -> CString {
    // allocate buffer of correct size
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    // fill it with len spaces
    buffer.extend([b' '].iter().cycle().take(len));
    // convert buffer to CString
    unsafe { CString::from_vec_unchecked(buffer) }
}

fn create_shader(path: &str, shader_type: GLenum) -> u32 {
    let shader: u32;
    let shader_cstr = read_from_file(path);
    let mut success: i32 = 1;

    unsafe {
        shader = gl::CreateShader(shader_type);

        gl::ShaderSource(shader, 1, &shader_cstr.as_ptr(), std::ptr::null());
        gl::CompileShader(shader);

        gl::GetProgramiv(shader, gl::COMPILE_STATUS, &mut success);

        if success == 0 {
            let error = create_available_space_cstring_with_len(512);

            gl::GetShaderInfoLog(
                shader,
                512,
                std::ptr::null_mut(),
                error.as_ptr() as *mut GLchar,
            );

            println!(
                "Shader Compile Status error: {}",
                error.to_string_lossy().into_owned()
            );
        }
    }

    shader
}

fn create_program_from_shaders(vertex_shader: u32, fragment_shader: u32) -> u32 {
    let program: u32;
    let mut success: i32 = 1;

    unsafe {
        program = gl::CreateProgram();

        gl::AttachShader(program, vertex_shader);
        gl::AttachShader(program, fragment_shader);
        gl::LinkProgram(program);

        gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);

        if success == 0 {
            let mut len: i32 = 0;

            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);

            let error = create_available_space_cstring_with_len(len as usize);

            gl::GetProgramInfoLog(
                program,
                len,
                std::ptr::null_mut(),
                error.as_ptr() as *mut GLchar,
            );

            println!(
                "Program Linking Error: {}",
                error.to_string_lossy().into_owned()
            );
        }

        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);
    }

    program
}

pub fn create_shader_from_files(vertex_path: &str, fragment_path: &str) -> Shader {
    let vertex_shader = create_shader(vertex_path, gl::VERTEX_SHADER);
    let fragment_shader = create_shader(fragment_path, gl::FRAGMENT_SHADER);

    let program: u32 = create_program_from_shaders(vertex_shader, fragment_shader);

    Shader { id: program }
}
