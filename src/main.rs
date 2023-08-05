extern crate gl;
extern crate image;

mod engine;

use std::ffi::c_void;

use glfw::{Action, Context, Key, Monitor};
use glfw_passthrough as glfw;

const WINDOW_WIDTH: u32 = 40;
const WINDOW_HEIGHT: u32 = 40;

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
    glfw.window_hint(glfw::WindowHint::MousePassthrough(true));

    // This puts our windows always on top
    glfw.window_hint(glfw::WindowHint::Floating(true));
    glfw.window_hint(glfw::WindowHint::Focused(false));
    glfw.window_hint(glfw::WindowHint::FocusOnShow(false));
    glfw.window_hint(glfw::WindowHint::CenterCursor(false));
    glfw.window_hint(glfw::WindowHint::ClientApi(glfw::ClientApiHint::OpenGl));

    let (mut window, events) = glfw
        .create_window(WINDOW_WIDTH, WINDOW_HEIGHT, "", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW Window");

    window.make_current();
    window.set_key_polling(true);
    window.set_scroll_polling(true);
    window.set_framebuffer_size_polling(true);

    // window.set_cursor_mode(glfw::CursorMode::Disabled);
    window.set_cursor_enter_polling(true);
    window.set_focus_polling(true);
    window.set_mouse_button_polling(false);

    gl::load_with(|name| window.get_proc_address(name) as *const _);

    let monitor = glfw.with_primary_monitor(|_, m| {
        (
            m.unwrap().get_video_mode().unwrap().width,
            m.unwrap().get_video_mode().unwrap().height,
        )
    });

    let win_center_x = (monitor.0 / 2) - (WINDOW_WIDTH / 2);
    let win_center_y = (monitor.1 / 2) - (WINDOW_HEIGHT / 2);

    window.set_pos((win_center_x) as i32, (win_center_y) as i32);

    // OpenGL set up
    let default_shader = engine::shader::create_shader_from_files(
        "src/engine/resources/vertex_shader.glsl",
        "src/engine/resources/fragment_shader.glsl",
    );

    default_shader.use_shader();

    let default_buffer = engine::buffer::Buffer::new();
    let default_texture = engine::texture::Texture::new();

    let mut vertices: Vec<f32> = vec![
        1.0, 1.0, 0.0, 1.0, -1.0, 0.0, -1.0, -1.0, 0.0, -1.0, 1.0, 0.0,
    ];
    const SCALE: f32 = 0.2;
    vertices = vertices.iter().map(|e| e * SCALE).collect();
    // let vertices: Vec<f32> = vec![
    //     0.05, 0.05, 0.0, 0.05, -0.05, 0.0, -0.05, -0.05, 0.0, -0.05, 0.05, 0.0,
    // ];
    let indices = vec![0, 1, 3, 1, 2, 3];
    let texture_coord: Vec<f32> = vec![1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0];

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

    default_buffer.bind_tbo();
    default_buffer.tbo_data(
        (texture_coord.len() * std::mem::size_of::<f32>()) as isize,
        texture_coord.as_ptr() as *const c_void,
        gl::STATIC_DRAW,
    );

    let mut default_crosshair = image::open("assets/test.png").unwrap();
    default_crosshair = default_crosshair.flipv();

    default_texture.bind();
    default_texture.setup_2d();
    default_texture.tex_image2d(
        default_crosshair.width() as i32,
        default_crosshair.height() as i32,
        default_crosshair.into_bytes().as_ptr() as *const c_void,
    );

    unsafe {
        gl::Uniform1i(default_shader.get_uniform_loc_from_name("ourTexture"), 0);
    }

    while !window.should_close() {
        // OpenGL set ups
        unsafe {
            gl::Viewport(0, 0, WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32);

            // gl::ClearColor(0.1, 0.3, 0.5, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::Clear(gl::DEPTH_TEST);
            gl::Enable(gl::DEPTH_TEST);
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }

        // user inputs
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }

        // Changes and Updates
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
        }
        default_texture.bind();

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
        glfw::WindowEvent::CursorEnter(_) => {
            println!("nois que voa");
            window.set_cursor_pos((WINDOW_WIDTH + 40) as f64, WINDOW_HEIGHT as f64);
        }
        glfw::WindowEvent::Focus(_) => {
            println!("here {:?}", window.is_framebuffer_transparent());
            window.set_cursor_pos((WINDOW_WIDTH + 40) as f64, WINDOW_HEIGHT as f64);
        }
        _ => {}
    }
}
