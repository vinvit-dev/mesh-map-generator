use egui::vec2;
use engine::ui::DrawUserInterface;

use super::noise::Noise;

impl DrawUserInterface for Noise {
    fn render(&mut self, ctx: &egui::Context, mut painter: &mut egui_gl_glfw::Painter) {
        if self.texture.is_none() {
            self.to_texture(&mut painter);
        }
        if self.color_map.is_none() {
            self.to_color_map(&mut painter);
        }

        egui::Window::new("Noise Generator").show(ctx, |ui| {
            let mut notify = vec![];
            ui.vertical(|ui| {
                ui.label("Noice params");
                notify.push(ui.add(egui::Slider::new(&mut self.map_width, 1..=256).text("Width")));
                notify
                    .push(ui.add(egui::Slider::new(&mut self.map_height, 1..=256).text("Height")));
                notify.push(ui.add(egui::Slider::new(&mut self.scale, 0.01..=50.0).text("Scale")));
                notify.push(ui.add(egui::Slider::new(&mut self.octaves, 1..=50).text("Octaves")));
                notify.push(
                    ui.add(egui::Slider::new(&mut self.persistance, 0.1..=5.).text("Persistance")),
                );
                notify.push(
                    ui.add(egui::Slider::new(&mut self.lacunarity, 0.1..=5.).text("Lacunarity")),
                );
                notify.push(ui.add(egui::Slider::new(&mut self.seed, 1..=100).text("Seed")));
                ui.horizontal(|ui| {
                    notify.push(
                        ui.add(egui::Slider::new(&mut self.offset.x, 1.0..=10.).text("Offset X")),
                    );
                    notify.push(
                        ui.add(egui::Slider::new(&mut self.offset.y, 1.0..=10.).text("Offset Y")),
                    );
                });
                if ui.button("Generate").clicked() {
                    self.generate();
                }
            });

            if self.texture.is_some() {
                ui.separator();
                ui.label("Result");
                ui.add(egui::Image::new(egui::load::SizedTexture {
                    id: self.texture.unwrap(),
                    size: vec2(self.map_width as f32, self.map_height as f32),
                }));
            }

            ui.separator();

            if self.texture.is_some() {
                ui.add(egui::Image::new(egui::load::SizedTexture {
                    id: self.color_map.unwrap(),
                    size: vec2(self.map_width as f32, self.map_height as f32),
                }));
                ui.separator();
            }

            for i in notify {
                if i.changed() {
                    self.generate();
                }
            }
        });
        if self.texture.is_some() {}
    }
}
