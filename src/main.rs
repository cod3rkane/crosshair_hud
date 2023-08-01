extern crate gl;
extern crate glfw;

mod engine;

use std::ffi::c_void;

use glfw::{Action, Context, Key};

const WINDOW_WIDTH: u32 = 200;
const WINDOW_HEIGHT: u32 = 140;

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    // OpenGL Settings
    glfw.window_hint(glfw::WindowHint::ContextVersion(4, 6));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

    // Window Transparency and disable Borders
    glfw.window_hint(glfw::WindowHint::TransparentFramebuffer(true));
    glfw.window_hint(glfw::WindowHint::Decorated(false));

    // This puts our windows always on top
    glfw.window_hint(glfw::WindowHint::Floating(true));

    let (mut window, events) = glfw
        .create_window(WINDOW_WIDTH, WINDOW_HEIGHT, "", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW Window");

    window.make_current();
    window.set_key_polling(true);
    window.set_scroll_polling(true);
    window.set_framebuffer_size_polling(true);

    gl::load_with(|name| window.get_proc_address(name) as *const _);

    let monitor = glfw::Monitor::from_primary();
    let mode = glfw::Monitor::get_video_mode(&monitor).unwrap();

    let win_center_x = (mode.width / 2) - 150;
    let win_center_y = (mode.height / 2) - 150;

    window.set_pos((win_center_x) as i32, (win_center_y) as i32);

    // OpenGL set up
    let default_shader = engine::shader::create_shader_from_files(
        "src/engine/resources/vertex_shader.glsl",
        "src/engine/resources/fragment_shader.glsl",
    );

    default_shader.use_shader();

    let default_buffer = engine::buffer::Buffer::new();
    let default_texture = engine::texture::Texture::new();

    let vertices: Vec<f32> = vec![
        0.5, 0.5, 0.0, 0.5, -0.5, 0.0, -0.5, -0.5, 0.0, -0.5, 0.5, 0.0,
    ];
    let indices = vec![0, 1, 3, 1, 2, 3];
    let texture_coord = vec![0.0, 0.5, 0.0, 1.0, 1.0, 0.0];

    default_buffer.bind_vao();

    default_buffer.bind_ebo();
    default_buffer.ebo_data(
        (indices.len() * std::mem::size_of::<i32>()) as isize,
        indices.as_ptr() as *const c_void,
        gl::STATIC_DRAW,
    );

    default_buffer.bind_vbo();
    default_buffer.vbo_data(
        (vertices.len() * std::mem::size_of::<f32>()) as isize,
        vertices.as_ptr() as *const c_void,
        gl::STATIC_DRAW,
    );

    // @TODO: texture

    while !window.should_close() {
        // OpenGL set ups
        unsafe {
            gl::Viewport(0, 0, WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32);

            // gl::ClearColor(0.1, 0.3, 0.5, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::Clear(gl::DEPTH_TEST);
        }

        // user inputs
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }

        // Changes and Updates

        // OpenGL Render
        unsafe {
            // gl::DrawArrays(gl::TRIANGLES, 0, 3);
            gl::DrawElements(
                gl::TRIANGLES,
                indices.len() as i32,
                gl::UNSIGNED_INT,
                std::ptr::null(),
            );
        }

        window.swap_buffers();
        glfw.poll_events();
    }

    default_buffer.clean();
    default_texture.unbind();
    default_shader.delete_shader();
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true);
        }
        _ => {}
    }
}
