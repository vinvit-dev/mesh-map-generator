use egui::vec2;
use engine::{
    camera::Camera,
    clear,
    keyboard_handler::{KeyBoardHandler, KeyBoardHandlerEvent},
    polygon_mode::{polygon_mode, PolygonMode},
    set_clear_color,
    ui::UserInterface,
};
use glfw::{Context, Key};

use crate::terrain::{noise::Noise, Terrain};

pub struct App {
    window: glfw::PWindow,
    events: glfw::GlfwReceiver<(f64, glfw::WindowEvent)>,
    glfw: glfw::Glfw,
    camera: Camera,
    keyboard_handler: KeyBoardHandler,
    last_frame: f32,
    ui: UserInterface,
}

impl App {
    pub fn init(s_width: u32, s_height: u32, title: &str) -> Self {
        // Init glfw
        use glfw::fail_on_errors;
        let mut glfw = glfw::init(fail_on_errors!()).unwrap();

        // Init windo
        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 2));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));
        glfw.window_hint(glfw::WindowHint::DoubleBuffer(true));
        glfw.window_hint(glfw::WindowHint::Resizable(false));

        let (mut window, events) = glfw
            .create_window(s_width, s_height, title, glfw::WindowMode::Windowed)
            .expect("Fail to create widnow");
        window.make_current();

        window.set_char_polling(true);
        window.set_cursor_pos_polling(true);
        window.set_key_polling(true);
        window.set_mouse_button_polling(true);
        glfw.set_swap_interval(glfw::SwapInterval::Sync(1));

        window.set_framebuffer_size_callback(move |_, width, height| unsafe {
            gl::Viewport(0, 0, width, height);
        });
        window.set_cursor_mode(glfw::CursorMode::Disabled);

        // Init OpenGL
        gl::load_with(|f_name| glfw.get_proc_address_raw(f_name));

        set_clear_color(0.7, 0.7, 1.0, 1.0);

        let camera = Camera::create_camera(s_width as f32, s_height as f32, 0.1, 1.0);
        let keyboard_handler = KeyBoardHandler::new();
        let last_frame = glfw.get_time() as f32;

        let ui = UserInterface::new(&mut window);

        Self {
            window,
            events,
            glfw,
            camera,
            keyboard_handler,
            last_frame,
            ui,
        }
    }

    pub fn run(&mut self) {
        let mut mode = PolygonMode::Fill;
        let mut noise = Noise::new(128, 128, 12, 27.0, 4, 0.5, 2.0, vec2(0.12, 0.4));
        noise.generate();
        let mut terrain = Terrain::init();
        terrain.update(&mut noise);

        while !self.window.should_close() {
            clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            if noise.updated {
                terrain.update(&mut noise);
                noise.updated = false;
            }

            polygon_mode(mode);
            terrain.render(&self.camera, &self.window);

            polygon_mode(PolygonMode::Fill);
            self.ui.render(vec![Box::new(&mut noise)]);

            let time = self.glfw.get_time() as f32;
            let delta_time = time - self.last_frame;
            self.last_frame = time;
            for (_, event) in glfw::flush_messages(&self.events) {
                match event {
                    glfw::WindowEvent::Key(Key::P, _, glfw::Action::Press, _) => {
                        mode = PolygonMode::turn(mode);
                        // polygon_mode(mode);
                    }
                    _ => {
                        self.ui.handle_event(&event);
                        App::keyboard_input_handler(
                            &mut self.window,
                            &mut self.keyboard_handler,
                            &event,
                        );
                        self.camera.handle_input(&self.keyboard_handler, delta_time);
                        self.camera.handle_cursor_pos(&event);
                    }
                }
            }

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
            Some(KeyBoardHandlerEvent::Pressed(Key::K)) => {
                window.set_cursor_mode(glfw::CursorMode::Normal);
            }
            Some(KeyBoardHandlerEvent::Pressed(Key::H)) => {
                window.set_cursor_mode(glfw::CursorMode::Disabled);
            }
            _ => {}
        }
    }
}
