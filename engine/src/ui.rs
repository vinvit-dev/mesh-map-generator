use egui::{vec2, Pos2, Rect};
use egui_backend::EguiInputState;
use egui_gl_glfw as egui_backend;
use glfw::PWindow;

pub struct UserInterface {
    pub painter: egui_backend::Painter,
    pub ctx: egui::Context,
    pub input_state: EguiInputState,
}

impl UserInterface {
    pub fn new(mut window: &mut PWindow) -> Self {
        let painter = egui_backend::Painter::new(&mut window);
        let ctx = egui::Context::default();

        let (width, height) = window.get_framebuffer_size();
        let native_pixels_per_point = window.get_content_scale().1;

        let input_state = egui_backend::EguiInputState::new(
            egui::RawInput {
                screen_rect: Some(Rect::from_min_size(
                    Pos2::new(0f32, 0f32),
                    vec2(width as f32, height as f32) / native_pixels_per_point,
                )),
                ..Default::default()
            },
            native_pixels_per_point,
        );

        Self {
            painter,
            ctx,
            input_state,
        }
    }

    pub fn begin(&mut self) {
        self.ctx.begin_frame(self.input_state.input.take());
    }

    pub fn end(&mut self) {
        let egui::FullOutput {
            platform_output,
            textures_delta,
            shapes,
            pixels_per_point,
            viewport_output: _,
        } = self.ctx.end_frame();

        if !platform_output.copied_text.is_empty() {
            egui_backend::copy_to_clipboard(&mut self.input_state, platform_output.copied_text);
        }

        let clipped_shapes = self.ctx.tessellate(shapes, pixels_per_point);
        self.painter
            .paint_and_update_textures(1.0, &clipped_shapes, &textures_delta);
    }

    pub fn handle_event(&mut self, event: &glfw::WindowEvent) {
        egui_backend::handle_event(event.clone(), &mut self.input_state);
    }
}
