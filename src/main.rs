use egui::vec2;
use engine::{
    base::{
        config::{poll_events::PollEvents, window_setup::WindowSetup, Config},
        context::AppContext,
        even_handler::EventHandler,
        run,
    },
    camera::Camera,
    clear,
    keyboard_handler::{KeyBoardHandler, KeyBoardHandlerEvent},
    polygon_mode::{polygon_mode, PolygonMode},
    ui::UserInterface,
};
use glfw::{Context, Key};
use terrain::{noise::Noise, Terrain};

pub mod terrain;

struct MainState {
    camera: Camera,
    keyboard_handler: KeyBoardHandler,
    last_frame: f32,
    polygon_mode: PolygonMode,
    noise: Noise,
    terrain: Terrain, // ui: UserInterface,
    ui: UserInterface,
}

fn main() {
    // App::init(WINDOW_WIDTH, WINDOW_HEIGHT, "Heightmap").run();

    let mut conf = Config::default();
    conf.window_setup = WindowSetup {
        title: "Mesh world",
        width: 1368,
        height: 780,
        ..Default::default()
    };
    conf.pull_events = vec![
        PollEvents::Char,
        PollEvents::Key,
        PollEvents::MauseButton,
        PollEvents::CursorPos,
    ];
    let mut ctx = AppContext::new();
    ctx.conf(conf);
    ctx.build();

    ctx.window
        .as_mut()
        .unwrap()
        .set_cursor_mode(glfw::CursorMode::Disabled);

    let camera = Camera::create_camera(
        ctx.config.window_setup.width as f32,
        ctx.config.window_setup.height as f32,
        0.1,
        1.0,
    );
    let keyboard_handler = KeyBoardHandler::new();
    let last_frame = ctx.glfw.clone().unwrap().get_time() as f32;

    let mode = PolygonMode::Fill;
    let mut noise = Noise::new(32, 64, 12, 20.0, 4, 0.2, 4.0, vec2(0.12, 0.4));
    noise.generate();
    let mut terrain = Terrain::init();
    terrain.update(&mut noise);

    let ui = UserInterface::new(ctx.window.as_mut().unwrap());
    let mut state = MainState {
        camera,
        keyboard_handler,
        last_frame,
        polygon_mode: mode,
        noise,
        terrain, // ui,
        ui,
    };

    run(&mut ctx, &mut state);
}

impl EventHandler for MainState {
    fn update(&mut self, context: &mut AppContext) {
        let glfw = context.glfw.as_mut().unwrap();
        let events = context.events.as_mut().unwrap();
        let mut window = context.window.as_mut().unwrap();

        if self.noise.updated {
            self.terrain.update(&mut self.noise);
            self.noise.updated = false;
        }
        let time = glfw.get_time() as f32;
        let delta_time = time - self.last_frame;
        self.last_frame = time;
        for (_, event) in glfw::flush_messages(events) {
            match event {
                glfw::WindowEvent::Key(Key::P, _, glfw::Action::Press, _) => {
                    self.polygon_mode = PolygonMode::turn(self.polygon_mode);
                    // polygon_mode(mode);
                }
                _ => {
                    self.ui.handle_event(&event);
                    MainState::keyboard_input_handler(
                        &mut window,
                        &mut self.keyboard_handler,
                        &event,
                    );
                    self.camera.handle_input(&self.keyboard_handler, delta_time);
                    self.camera.handle_cursor_pos(&event);
                }
            }
        }
        context.glfw.as_mut().unwrap().poll_events();
    }

    fn draw(&mut self, context: &mut AppContext) {
        let window = context.window.as_mut().unwrap();
        clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        polygon_mode(self.polygon_mode);
        self.terrain.render(&self.camera, &window);

        polygon_mode(PolygonMode::Fill);
        self.ui.render(vec![Box::new(&mut self.noise)]);
        window.swap_buffers();
    }
}

impl MainState {
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
