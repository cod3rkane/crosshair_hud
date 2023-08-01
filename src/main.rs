extern crate gl;
extern crate glfw;

use glfw::{Action, Context, Key};

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
        .create_window(300, 300, "", glfw::WindowMode::Windowed)
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

    while !window.should_close() {
        // OpenGL set ups
        unsafe {
            gl::Viewport(0, 0, 300, 300);

            gl::ClearColor(0.1, 0.3, 0.5, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::Clear(gl::DEPTH_TEST);
        }

        // user inputs
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }

        // Changes and Updates
        if !window.is_focused() {
            window.focus();
        }

        // OpenGL Render

        window.swap_buffers();
        glfw.poll_events();
    }
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true);
        }
        _ => {}
    }
}
