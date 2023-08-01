use std::ffi::c_void;

use gl;

pub struct Buffer {
    pub vao: u32, // Vertex Array Object holds all OpenGL data
    pub vbo: u32, // Vertices Buffer Object
    pub ebo: u32, // Element Buffer Object
    pub tbo: u32, // Texture Buffer Object
    pub mbo: u32, // Matrix Buffer Object
}

impl Buffer {
    pub fn new() -> Buffer {
        let mut vao: u32 = 0;
        let mut vbo: u32 = 0;
        let mut ebo: u32 = 0;
        let mut tbo: u32 = 0;
        let mut mbo: u32 = 0;

        unsafe {
            gl::CreateVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            gl::GenBuffers(1, &mut ebo);
            gl::GenBuffers(1, &mut tbo);
            gl::GenBuffers(1, &mut mbo);
        }

        Buffer {
            vao,
            vbo,
            ebo,
            tbo,
            mbo,
        }
    }

    /* binds the Vertex Array Object */
    pub fn bind_vao(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
        }
    }

    pub fn unbind_vao(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }

    pub fn unbind_array_buffer() {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }

    pub fn unbind_element_buffer() {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        }
    }

    /* binds Vertices Buffer Object */
    pub fn bind_vbo(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
        }
    }

    /* binds Element Buffer Object */
    pub fn bind_ebo(&self) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
        }
    }

    /* binds Texture Buffer Object */
    pub fn bind_tbo(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.tbo);
        }
    }

    /* binds Matrix Buffer Object */
    pub fn bind_mbo(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.mbo);
        }
    }

    pub fn vbo_data(&self, size: isize, data: *const c_void, usage: u32) {
        self.bind_vbo();

        unsafe {
            gl::BufferData(gl::ARRAY_BUFFER, size, data, usage);
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                (3 * std::mem::size_of::<f32>()) as i32,
                std::ptr::null(),
            );
            gl::EnableVertexAttribArray(0); // Vertex Attribute number depends on the Shader declaration
        }
    }

    pub fn ebo_data(&self, size: isize, data: *const c_void, usage: u32) {
        self.bind_ebo();

        unsafe {
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, size, data, usage);
        }
    }

    pub fn tbo_data(&self, size: isize, data: *const c_void, usage: u32) {
        self.bind_tbo();

        unsafe {
            gl::BufferData(gl::ARRAY_BUFFER, size, data, usage);
            gl::VertexAttribPointer(
                1,
                2,
                gl::FLOAT,
                gl::FALSE,
                (2 * std::mem::size_of::<f32>()) as i32,
                std::ptr::null(),
            );
            gl::EnableVertexAttribArray(1); // Vertex Attribute number depends on the Shader declaration
        }
    }

    pub fn mbo_data(&self, size: isize, data: *const c_void, usage: u32) {
        self.bind_mbo();

        unsafe {
            gl::BufferData(gl::ARRAY_BUFFER, size, data, usage);
            gl::VertexAttribPointer(
                2,
                4,
                gl::FLOAT,
                gl::FALSE,
                (16 * std::mem::size_of::<f32>()) as i32,
                std::ptr::null(),
            );
            gl::EnableVertexAttribArray(2); // Vertex Attribute number depends on the Shader declaration

            gl::VertexAttribPointer(
                3,
                4,
                gl::FLOAT,
                gl::FALSE,
                (16 * std::mem::size_of::<f32>()) as i32,
                (4 * std::mem::size_of::<f32>()) as *const c_void,
            );
            gl::EnableVertexAttribArray(3);

            gl::VertexAttribPointer(
                4,
                4,
                gl::FLOAT,
                gl::FALSE,
                (16 * std::mem::size_of::<f32>()) as i32,
                (8 * std::mem::size_of::<f32>()) as *const c_void,
            );
            gl::EnableVertexAttribArray(4);

            gl::VertexAttribPointer(
                5,
                4,
                gl::FLOAT,
                gl::FALSE,
                (16 * std::mem::size_of::<f32>()) as i32,
                (12 * std::mem::size_of::<f32>()) as *const c_void,
            );
            gl::EnableVertexAttribArray(5);

            gl::VertexAttribDivisor(2, 1);
            gl::VertexAttribDivisor(3, 1);
            gl::VertexAttribDivisor(4, 1);
            gl::VertexAttribDivisor(5, 1);
        }
    }

    pub fn clean(&self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteBuffers(1, &self.ebo);
            gl::DeleteBuffers(1, &self.tbo);
        }
    }
}
