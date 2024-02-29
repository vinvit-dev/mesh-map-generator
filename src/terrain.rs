use ds_heightmap::Output;
use nalgebra_glm as glm;
use std::mem::size_of;

use engine::{
    buffer::{buffer_data, Buffer, BufferType, VertexArray},
    camera::Camera,
    shader_program::{ShaderProgram, Uniform},
};

pub struct Terrain {
    vao: VertexArray,
    shader_program: ShaderProgram,
    width: u32,
    depth: u32,
}

type Vertex = [f32; 3];

impl Terrain {
    pub fn init(width: u32, depth: u32) -> Self {
        let mut array: Vec<Vertex> = vec![];

        let highmap = gen_hightmap(depth as f32, width as usize);

        let pdistance = 0.04;

        for x in 0..width - 1 {
            for z in 0..depth - 1 {
                array.push([
                    pdistance * x as f32 - width as f32 / 2. * pdistance,
                    get_highmap_point(&highmap, x, z),
                    pdistance * z as f32 - depth as f32 / 2. * pdistance,
                ]);
                array.push([
                    pdistance * (x + 1) as f32 - width as f32 / 2. * pdistance,
                    get_highmap_point(&highmap, x + 1, z),
                    pdistance * z as f32 - depth as f32 / 2. * pdistance,
                ]);
                array.push([
                    pdistance * x as f32 - width as f32 / 2. * pdistance,
                    get_highmap_point(&highmap, x, z + 1),
                    pdistance * (z + 1) as f32 - depth as f32 / 2. * pdistance,
                ]);
                array.push([
                    pdistance * (x + 1) as f32 - width as f32 / 2. * pdistance,
                    get_highmap_point(&highmap, x + 1, z + 1),
                    pdistance * (z + 1) as f32 - depth as f32 / 2. * pdistance,
                ]);
                array.push([
                    pdistance * (x + 1) as f32 - width as f32 / 2. * pdistance,
                    get_highmap_point(&highmap, x + 1, z),
                    pdistance * z as f32 - depth as f32 / 2. * pdistance,
                ]);
                array.push([
                    pdistance * x as f32 - width as f32 / 2. * pdistance,
                    get_highmap_point(&highmap, x, z + 1),
                    pdistance * (z + 1) as f32 - depth as f32 / 2. * pdistance,
                ]);
            }
        }

        let vao = VertexArray::new().unwrap();
        Buffer::new().unwrap().bind(BufferType::Array);
        vao.bind();
        buffer_data(
            BufferType::Array,
            bytemuck::cast_slice(array.as_slice()),
            gl::STATIC_DRAW,
        );

        unsafe {
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                size_of::<Vertex>().try_into().unwrap(),
                0 as *const _,
            );
            gl::EnableVertexAttribArray(0);
        };

        // let mut indices: Vec<f32> = vec![];
        //
        // Buffer::new().unwrap().bind(BufferType::Array);
        // vao.bind();
        // buffer_data(
        //     BufferType::Array,
        //     bytemuck::cast_slice(array.as_slice()),
        //     gl::STATIC_DRAW,
        // );
        //
        // for x in 0..width {
        //     for z in 0..depth {}
        // }

        let shader_program =
            ShaderProgram::from_vert_frag_files("./shaders/terrain.vs", "./shaders/terrain.fs")
                .unwrap();

        Self {
            vao,
            shader_program,
            width,
            depth,
        }
    }

    pub fn render(&self, camera: &Camera) {
        self.vao.bind();
        self.shader_program.use_program();
        self.shader_program
            .set_uniform("view", Uniform::Mat4(camera.look_at()));
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
        unsafe { gl::DrawArrays(gl::TRIANGLES, 0, (self.width * self.depth * 6) as i32) }
    }
}

pub fn gen_hightmap(depth: f32, width: usize) -> Output {
    let mut heightmap_runner = ds_heightmap::Runner::new();
    heightmap_runner.set_depth(depth / 2.);
    heightmap_runner.set_width(width);
    heightmap_runner.set_height(depth as usize);
    heightmap_runner.set_rough(1.0);
    heightmap_runner.ds()
}

pub fn get_highmap_point(hightmap: &Output, x: u32, z: u32) -> f32 {
    hightmap
        .data
        .to_vec()
        .get(x as usize)
        .unwrap()
        .get(z as usize)
        .unwrap()
        .clone()
        / ((hightmap.max + hightmap.min) / 3.)
}
