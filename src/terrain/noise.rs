use egui::{emath::inverse_lerp, lerp, vec2, Color32, TextureId, Vec2};
use image::{ImageBuffer, Rgb};
use rand::{thread_rng, Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

#[derive(Default, Debug)]
pub struct NoiseMap {
    pub data: Vec<Vec<f32>>,
    pub min: f32,
    pub max: f32,
    pub width: u32,
    pub height: u32,
}

impl NoiseMap {
    pub fn to_image(&self) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        let mut noise = ImageBuffer::<Rgb<u8>, Vec<u8>>::new(self.width, self.height);
        for x in 0..self.width {
            for y in 0..self.height {
                let color_val = lerp(0.0..=255.0, self.data[x as usize][y as usize]).round() as u8;
                noise.get_pixel_mut(x, y).0 = [color_val, color_val, color_val];
            }
        }
        noise
    }
}

#[derive(Default)]
pub struct Noise {
    pub image: Option<ImageBuffer<Rgb<u8>, Vec<u8>>>,
    pub noise_map: NoiseMap,
    pub map_width: u32,
    pub map_height: u32,
    pub scale: f64,
    pub octaves: u32,
    pub persistance: f32,
    pub lacunarity: f32,
    pub seed: i32,
    pub offset: Vec2,
    pub texture: Option<TextureId>,
    pub color_map: Option<TextureId>,
}

impl Noise {
    pub fn new(
        map_width: u32,
        map_height: u32,
        seed: i32,
        scale: f64,
        octaves: u32,
        persistance: f32,
        lacunarity: f32,
        offset: Vec2,
    ) -> Self {
        Self {
            map_width,
            map_height,
            seed,
            scale,
            image: None,
            octaves,
            persistance,
            lacunarity,
            offset,
            ..Default::default()
        }
    }

    pub fn generate(&mut self) {
        if self.scale <= 0. {
            self.scale = 0.0001;
        }

        let mut noise_map = NoiseMap::default();
        noise_map.width = self.map_width;
        noise_map.height = self.map_height;

        let mut octaves_offsets: Vec<Vec2> = vec![];
        let mut seed: <ChaCha8Rng as SeedableRng>::Seed = Default::default();
        thread_rng().fill(&mut seed);
        let mut rng = ChaCha8Rng::from_seed(seed);

        let perlin = perlin_noise::PerlinNoise::new();

        for _ in 0..self.octaves as usize {
            let offset_x = rng.gen_range(-10000..100000) + self.offset.x as i32;
            let offset_y = rng.gen_range(-10000..100000) + self.offset.y as i32;
            octaves_offsets.push(vec2(offset_x as f32, offset_y as f32));
        }

        for x in 0..self.map_width as i32 {
            noise_map.data.push(vec![]);
            for z in 0..self.map_height as i32 {
                let mut amplitude: f32 = 1.;
                let mut frequency: f32 = 1.;
                let mut noise_height: f32 = 0.;

                for i in 0..self.octaves as usize {
                    let sample_x =
                        (x) as f64 / self.scale * frequency as f64 + octaves_offsets[i].x as f64;
                    let sample_y =
                        (z) as f64 / self.scale * frequency as f64 + octaves_offsets[i].x as f64;

                    let perlin_value = perlin.get2d([sample_x, sample_y]) as f32 * 2. - 1.;
                    noise_height += perlin_value * amplitude;
                    amplitude *= self.persistance;
                    frequency *= self.lacunarity;
                }

                noise_map.data[x as usize].push(noise_height);

                if noise_height > noise_map.max {
                    noise_map.max = noise_height;
                } else if noise_height < noise_map.min {
                    noise_map.min = noise_height;
                }
            }
        }
        for x in 0..self.map_width {
            for z in 0..self.map_height {
                noise_map.data[x as usize][z as usize] = inverse_lerp(
                    noise_map.min..=noise_map.max,
                    noise_map.data[x as usize][z as usize],
                )
                .unwrap();
            }
        }

        self.noise_map = noise_map;
        self.image = Some(self.noise_map.to_image());
        self.texture = None;
        self.color_map = None;
    }
    pub fn to_texture(&mut self, painter: &mut egui_gl_glfw::Painter) {
        let mut srgba = vec![Color32::BLACK; (self.map_height * self.map_width) as usize];

        let plot_tex_id = painter.new_user_texture(
            (self.map_width as usize, self.map_height as usize),
            &srgba,
            egui::TextureFilter::Linear,
        );

        for x in 0..self.map_width {
            for y in 0..self.map_height {
                let color_val =
                    lerp(0.0..=255.0, self.noise_map.data[x as usize][y as usize]).round() as u8;
                srgba[(x * self.map_width + y) as usize] =
                    Color32::from_rgb(color_val, color_val, color_val);
            }
        }
        painter.update_user_texture_data(&plot_tex_id, &srgba);
        self.texture = Some(plot_tex_id);
    }
    pub fn to_color_map(&mut self, painter: &mut egui_gl_glfw::Painter) {
        let mut srgba = vec![Color32::BLACK; (self.map_height * self.map_width) as usize];

        let plot_tex_id = painter.new_user_texture(
            (self.map_width as usize, self.map_height as usize),
            &srgba,
            egui::TextureFilter::Linear,
        );

        for x in 0..self.map_width {
            for y in 0..self.map_height {
                let height = self.noise_map.data[x as usize][y as usize];
                let mut color_val = Color32::BLACK;
                if height > 0.0 && height < 0.4 {
                    color_val = Color32::BLUE;
                } else if height > 0.4 && height < 0.5 {
                    color_val = Color32::YELLOW;
                } else if height > 0.5 && height < 0.7 {
                    color_val = Color32::GREEN;
                } else if height > 0.7 && height < 0.9 {
                    color_val = Color32::BROWN;
                } else if height > 0.9 {
                    color_val = Color32::WHITE;
                }

                srgba[(x * self.map_width + y) as usize] = color_val;
            }
        }
        painter.update_user_texture_data(&plot_tex_id, &srgba);
        self.color_map = Some(plot_tex_id);
    }

    pub fn draw_ui(&mut self, ctx: &egui::Context, mut painter: &mut egui_gl_glfw::Painter) {
        if self.texture.is_none() {
            self.to_texture(&mut painter);
        }
        if self.color_map.is_none() {
            self.to_color_map(&mut painter);
        }

        egui::SidePanel::new(egui::panel::Side::Left, "Noise Generator").show(ctx, |ui| {
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
                notify.push(ui.add(egui::Slider::new(&mut self.seed, 1..=1000000).text("Seed")));
                ui.horizontal(|ui| {
                    notify.push(ui.add(
                        egui::Slider::new(&mut self.offset.x, 1.0..=100000.).text("Offset X"),
                    ));
                    notify.push(ui.add(
                        egui::Slider::new(&mut self.offset.y, 1.0..=100000.).text("Offset Y"),
                    ));
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
