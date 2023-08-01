extern crate glfw;

use glfw::{Action, Context, Key};

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    glfw.window_hint(glfw::WindowHint::TransparentFramebuffer(true));
    // glfw.window_hint(glfw::WindowHint::Decorated(false));

    let (mut window, events) = glfw
        .create_window(300, 300, "", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW Window");

    window.set_key_polling(true);
    window.make_current();

    let monitor = glfw::Monitor::from_primary();
    let mode = glfw::Monitor::get_video_mode(&monitor).unwrap();

    let win_center_x = (mode.width / 2) - 150;
    let win_center_y = (mode.height / 2) - 150;

    window.set_pos((win_center_x) as i32, (win_center_y) as i32);

    while !window.should_close() {
        glfw.poll_events();

        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }
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
