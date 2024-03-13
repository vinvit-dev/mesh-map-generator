use egui::{emath::inverse_lerp, lerp, vec2, Color32, TextureId, Vec2};
use image::{ImageBuffer, Rgb};
use seeded_random::Seed;

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

#[derive()]
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
    pub updated: bool,
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
            noise_map: NoiseMap::default(),
            texture: None,
            color_map: None,
            updated: false,
        }
    }

    pub fn generate(&mut self) {
        if self.scale <= 0. {
            self.scale = 0.0001;
        }

        let mut noise_map = NoiseMap::default();
        noise_map.width = self.map_width;
        noise_map.height = self.map_height;

        let half_width = (noise_map.width / 2) as i32;
        let half_height = (noise_map.height / 2) as i32;

        let mut octaves_offsets: Vec<Vec2> = vec![];
        let rng = seeded_random::Random::from_seed(Seed::unsafe_new(self.seed as u64));
        // let mut seed: <ChaCha8Rng as SeedableRng>::Seed = Default::default();
        // thread_rng().fill(&mut seed);
        // let mut rng = ChaCha8Rng::from_seed(seed);

        for _ in 0..self.octaves as usize {
            let offset_x = rng.range(0, 100000) as f32 + self.offset.x;
            let offset_y = rng.range(0, 100000) as f32 + self.offset.y;
            // let offset_x = self.offset.x as i32;
            // let offset_y = self.offset.y as i32;
            octaves_offsets.push(vec2(offset_x as f32, offset_y as f32));
        }

        for x in 0..self.map_width as i32 {
            noise_map.data.push(vec![]);
            for z in 0..self.map_height as i32 {
                let mut amplitude: f32 = 1.;
                let mut frequency: f32 = 1.;
                let mut noise_height: f32 = 0.;

                for i in 0..self.octaves as usize {
                    let sample_x = (x - half_width) as f32 / self.scale as f32 * frequency as f32
                        + octaves_offsets[i].x;
                    let sample_y = (z - half_height) as f32 / self.scale as f32 * frequency as f32
                        + octaves_offsets[i].y;

                    cgl_rs::noise::init();
                    let perlin_value = cgl_rs::noise::perlin(sample_x, sample_y, 0.0) * 2. - 1.;
                    cgl_rs::noise::shutdown();
                    // let perlin_value = cgl_rs::noise::perlin(sample_x, sample_y, 0.);
                    // let perlin_value =
                    //     self.perlin_noise.get2d([sample_x, sample_y]) as f32 * 2. - 1.;
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
        self.updated = true;
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
    pub fn get_color_for_height(height: f32) -> Color32 {
        let mut color_val = Color32::BLACK;
        if height > -1.0 && height < 0.5 {
            color_val = Color32::from_rgb(5, 67, 166);
        } else if height > 0.5 && height < 0.55 {
            color_val = Color32::from_rgb(174, 184, 83);
        } else if height > 0.55 && height < 0.8 {
            color_val = Color32::from_rgb(26, 145, 38);
        } else if height > 0.8 && height < 0.9 {
            color_val = Color32::from_rgb(74, 43, 27);
        } else if height > 0.9 {
            color_val = Color32::WHITE;
        }
        color_val
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
                let color_val = Noise::get_color_for_height(height);

                srgba[(x * self.map_width + y) as usize] = Color32::from_rgba_unmultiplied(
                    color_val.r(),
                    color_val.g(),
                    color_val.b(),
                    color_val.a(),
                );
            }
        }

        painter.update_user_texture_data(&plot_tex_id, &srgba);
        self.color_map = Some(plot_tex_id);
    }
}
