extern crate gl;
extern crate image;
extern crate sdl2;
extern crate winapi;

mod engine;

use std::ffi::c_void;

use sdl2::event::Event;
use sdl2::hint::Hint;
use sdl2::keyboard::Keycode;
use sdl2::sys::{
    SDL_GetWindowWMInfo, SDL_SetHint, SDL_WindowFlags,
    SDL_HINT_WINDOW_FRAME_USABLE_WHILE_CURSOR_HIDDEN,
};
use sdl2::video::WindowPos;

const WINDOW_WIDTH: u32 = 40;
const WINDOW_HEIGHT: u32 = 40;

fn find_sdl_gl_driver() -> Option<u32> {
    for (index, item) in sdl2::render::drivers().enumerate() {
        if item.name == "opengl" {
            return Some(index as u32);
        }
    }
    None
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();

    // OpenGL Settings
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 6);

    // sdl2::hint::set(na);
    // SDL_SetHint(SDL_HINT_WINDOW_FRAME_USABLE_WHILE_CURSOR_HIDDEN, false);
    sdl2::hint::set("SDL_HINT_WINDOW_FRAME_USABLE_WHILE_CURSOR_HIDDEN", "0");
    sdl2::hint::set("SDL_HINT_MOUSE_FOCUS_CLICKTHROUGH", "0");

    sdl_context.mouse().show_cursor(false);

    let window_flags = SDL_WindowFlags::SDL_WINDOW_ALWAYS_ON_TOP as u32
        | SDL_WindowFlags::SDL_WINDOW_SKIP_TASKBAR as u32
        | SDL_WindowFlags::SDL_WINDOW_BORDERLESS as u32;

    let mut window = video_subsystem
        .window("", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .set_window_flags(window_flags)
        .borderless()
        .opengl()
        .allow_highdpi()
        .build()
        .unwrap();

    let ctx = window.gl_create_context().unwrap();

    // SDL_GetWindowWMInfo(&mut window, info)

    // Window Transparency and disable Borders
    // glfw.window_hint(glfw::WindowHint::TransparentFramebuffer(true));
    // glfw.window_hint(glfw::WindowHint::Decorated(false));

    // This puts our windows always on top
    // glfw.window_hint(glfw::WindowHint::Floating(true));
    // glfw.window_hint(glfw::WindowHint::Focused(false));
    // glfw.window_hint(glfw::WindowHint::FocusOnShow(false));
    // glfw.window_hint(glfw::WindowHint::CenterCursor(false));
    // glfw.window_hint(glfw::WindowHint::ClientApi(glfw::ClientApiHint::OpenGl));

    // let (mut window, events) = glfw
    //     .create_window(WINDOW_WIDTH, WINDOW_HEIGHT, "", glfw::WindowMode::Windowed)
    //     .expect("Failed to create GLFW Window");

    // window.make_current();
    // window.set_key_polling(true);
    // window.set_scroll_polling(true);
    // window.set_framebuffer_size_polling(true);

    // // window.set_cursor_mode(glfw::CursorMode::Disabled);
    // window.set_cursor_enter_polling(true);
    // window.set_focus_polling(true);
    // window.set_mouse_button_polling(false);

    gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);

    // let monitor = glfw::Monitor::from_primary();
    // let mode = glfw::Monitor::get_video_mode(&monitor).unwrap();

    // let win_center_x = (mode.width / 2) - (WINDOW_WIDTH / 2);
    // let win_center_y = (mode.height / 2) - (WINDOW_HEIGHT / 2);

    // window.set_pos((win_center_x) as i32, (win_center_y) as i32);

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

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut is_running = true;

    while is_running {
        // OpenGL set ups
        unsafe {
            gl::Viewport(0, 0, WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32);

            // gl::ClearColor(1.0, 1.0, 1.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::Clear(gl::DEPTH_TEST);
            gl::Enable(gl::DEPTH_TEST);
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }

        // user inputs
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. }
                | sdl2::event::Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    is_running = false;
                }
                _ => {}
            }
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

        window.gl_swap_window();
    }

    default_buffer.clean();
    default_texture.unbind();
    default_shader.delete_shader();
}

// fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
//     match event {
//         glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
//             window.set_should_close(true);
//         }
//         glfw::WindowEvent::CursorEnter(_) => {
//             println!("nois que voa");
//             window.set_cursor_pos((WINDOW_WIDTH + 40) as f64, WINDOW_HEIGHT as f64);
//         }
//         glfw::WindowEvent::Focus(_) => {
//             println!("here {:?}", window.is_framebuffer_transparent());
//             window.set_cursor_pos((WINDOW_WIDTH + 40) as f64, WINDOW_HEIGHT as f64);
//         }
//         _ => {}
//     }
// }
