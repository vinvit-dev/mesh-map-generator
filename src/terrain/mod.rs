use std::mem::size_of;

use egui::emath::inverse_lerp;
use engine::{
    buffer::{buffer_data, Buffer, BufferType, VertexArray},
    camera::Camera,
    shader_program::{ShaderProgram, Uniform},
};
use glfw::PWindow;
use nalgebra_glm as glm;

use self::noise::Noise;

pub mod noise;
pub mod noise_ui;

type VERTICE = [f32; 3 + 3];

pub struct Terrain {
    pub vao: VertexArray,
    pub vbo: Buffer,
    pub ebo: Buffer,
    pub mesh_data: Option<MeshData>,
    pub shader_program: ShaderProgram,
}

impl Terrain {
    pub fn init() -> Self {
        let vao = VertexArray::new().unwrap();
        let vbo = Buffer::new().unwrap();
        let ebo = Buffer::new().unwrap();
        let shader_program =
            ShaderProgram::from_vert_frag_files("./shaders/terrain.vs", "./shaders/terrain.fs")
                .unwrap();
        Self {
            vao,
            vbo,
            ebo,
            shader_program,
            mesh_data: None,
        }
    }

    pub fn generate(noise: &mut Noise) -> MeshData {
        let width = noise.map_width;
        let height = noise.map_height;

        let top_left_x = (width - 1) as f32 / -2.;
        let top_left_z = (height - 1) as f32 / 2.;

        let mut mesh_data = MeshData {
            vertices: vec![],
            triangles: vec![],
        };
        let mut vertex_index = 0 as usize;

        for x in 0..width as usize {
            for y in 0..height as usize {
                let mut point_height = noise.noise_map.data[x][y];
                let color = Noise::get_color_for_height(point_height);
                let r = inverse_lerp(0.0..=255.0, color.r() as f32).unwrap();
                let g = inverse_lerp(0.0..=255.0, color.g() as f32).unwrap();
                let b = inverse_lerp(0.0..=255.0, color.b() as f32).unwrap();
                point_height = if point_height <= 0.50 {
                    0.50
                } else {
                    point_height
                };
                mesh_data.vertices.push([
                    x as f32 + top_left_x,
                    point_height,
                    top_left_z - y as f32,
                    r,
                    g,
                    b,
                ]);
                if x < width as usize - 1 && y < height as usize - 1 {
                    mesh_data.add_triangle(
                        vertex_index,
                        vertex_index + height as usize + 1,
                        vertex_index + height as usize,
                    );
                    mesh_data.add_triangle(
                        vertex_index + height as usize + 1,
                        vertex_index,
                        vertex_index + 1,
                    );
                }
                vertex_index += 1;
            }
        }
        mesh_data
    }

    pub fn update(&mut self, mut noise: &mut Noise) {
        self.vao.bind();
        self.vbo.bind(BufferType::Array);
        let mesh_data = Terrain::generate(&mut noise);
        buffer_data(
            BufferType::Array,
            bytemuck::cast_slice(mesh_data.vertices.as_slice()),
            gl::STATIC_DRAW,
        );
        self.mesh_data = Some(mesh_data.clone());
        unsafe {
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                size_of::<VERTICE>().try_into().unwrap(),
                0 as *const _,
            );
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                1,
                3,
                gl::FLOAT,
                gl::FALSE,
                size_of::<VERTICE>().try_into().unwrap(),
                size_of::<[f32; 3]>() as *const _,
            );
            gl::EnableVertexAttribArray(1);
        };
        self.ebo.bind(BufferType::ElementArray);
        buffer_data(
            BufferType::ElementArray,
            bytemuck::cast_slice(mesh_data.triangles.as_slice()),
            gl::STATIC_DRAW,
        );
    }

    pub fn render(&mut self, camera: &Camera, window: &PWindow) {
        let cursor_mode = window.get_cursor_mode();
        self.vao.bind();
        self.shader_program.use_program();
        match cursor_mode {
            glfw::CursorMode::Disabled => {
                self.shader_program
                    .set_uniform("view", Uniform::Mat4(camera.look_at()));
            }
            _ => {}
        }
        let model = glm::Mat4::identity();
        self.shader_program
            .set_uniform("model", Uniform::Mat4(model));
        let projection = glm::perspective(
            camera.fov.to_radians(),
            1280 as f32 / 768 as f32,
            0.01,
            100.,
        );
        self.shader_program
            .set_uniform("projection", Uniform::Mat4(projection));
        unsafe { gl::Enable(gl::DEPTH_TEST) };
        unsafe {
            gl::DrawElements(
                gl::TRIANGLES,
                self.mesh_data.clone().unwrap().triangles.len() as i32,
                gl::UNSIGNED_INT,
                0 as *const _,
            )
        };
        unsafe { gl::Disable(gl::DEPTH_TEST) };
    }
}

#[derive(Default, Clone)]
pub struct MeshData {
    pub vertices: Vec<VERTICE>,
    pub triangles: Vec<i32>,
}

impl MeshData {
    pub fn add_triangle(&mut self, a: usize, b: usize, c: usize) {
        self.triangles.push(a as i32);
        self.triangles.push(b as i32);
        self.triangles.push(c as i32);
    }
}
