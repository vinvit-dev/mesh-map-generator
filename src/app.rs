use engine::{
    camera::Camera,
    clear,
    keyboard_handler::{KeyBoardHandler, KeyBoardHandlerEvent},
    polygon_mode::{polygon_mode, PolygonMode},
    set_clear_color,
};
use glfw::{Context, Key};

use crate::terrain::Terrain;

pub struct App {
    window: glfw::PWindow,
    events: glfw::GlfwReceiver<(f64, glfw::WindowEvent)>,
    glfw: glfw::Glfw,
    camera: Camera,
    keyboard_handler: KeyBoardHandler,
    last_frame: f32,
}

impl App {
    pub fn init(s_width: u32, s_height: u32, title: &str) -> Self {
        // Init glfw
        use glfw::fail_on_errors;
        let mut glfw = glfw::init(fail_on_errors!()).unwrap();

        // Set OpenGlLproperties
        glfw.window_hint(glfw::WindowHint::ContextVersionMajor(3));
        glfw.window_hint(glfw::WindowHint::ContextVersionMinor(3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));

        // Init window
        let (mut window, events) = glfw
            .create_window(s_width, s_height, title, glfw::WindowMode::Windowed)
            .expect("Fail to create widnow");
        window.make_current();

        window.set_key_polling(true);
        window.set_cursor_pos_polling(true);
        window.set_framebuffer_size_callback(move |_, width, height| unsafe {
            gl::Viewport(0, 0, width, height);
        });
        window.set_cursor_mode(glfw::CursorMode::Disabled);

        // Init OpenGL
        gl::load_with(|f_name| glfw.get_proc_address_raw(f_name));

        unsafe { gl::Enable(gl::DEPTH_TEST) }

        set_clear_color(0.1, 0.1, 0.1, 1.0);

        let camera = Camera::create_camera(s_width as f32, s_height as f32, 0.1, 1.0);
        let keyboard_handler = KeyBoardHandler::new();
        let last_frame = glfw.get_time() as f32;

        Self {
            window,
            events,
            glfw,
            camera,
            keyboard_handler,
            last_frame,
        }
    }

    pub fn run(&mut self) {
        let terrain = Terrain::init(100, 100);

        let mut mode = PolygonMode::Fill;
        while !self.window.should_close() {
            let time = self.glfw.get_time() as f32;
            let delta_time = time - self.last_frame;
            self.last_frame = time;
            for (_, event) in glfw::flush_messages(&self.events) {
                match event {
                    glfw::WindowEvent::Key(Key::P, _, glfw::Action::Press, _) => {
                        mode = PolygonMode::turn(mode);
                        polygon_mode(mode);
                    }
                    _ => {}
                }
                App::keyboard_input_handler(&mut self.window, &mut self.keyboard_handler, &event);
                self.camera.handle_input(&self.keyboard_handler, delta_time);
                self.camera.handle_cursor_pos(&event);
            }

            clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            terrain.render(&self.camera);

            self.window.swap_buffers();
            self.glfw.poll_events();
        }
    }

    fn keyboard_input_handler(
        window: &mut glfw::PWindow,
        keyboard_handler: &mut KeyBoardHandler,
        event: &glfw::WindowEvent,
    ) {
        let keyboard_event = keyboard_handler.handle_event(event.clone());
        match keyboard_event {
            Some(KeyBoardHandlerEvent::Pressed(Key::Escape)) => {
                window.set_should_close(true);
            }
            _ => {}
        }
    }
}
