use nalgebra_glm as glm;
use std::{mem::size_of, slice::SliceIndex};

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
        let mut array: Vec<Vertex> = vec![[0.0, 0.0, 0.0]; width as usize];
        let mut heightmap_runner = ds_heightmap::Runner::new();
        heightmap_runner.set_depth(depth as f32);
        heightmap_runner.set_width(width as usize);
        heightmap_runner.set_height(depth as usize);
        heightmap_runner.set_rough(2.0);
        let output = heightmap_runner.ds();

        let pdistance = 0.2;

        array.clear();
        for x in 0..width - 1 {
            for z in 0..depth - 1 {
                array.push([
                    pdistance * x as f32 - width as f32 / 2. * pdistance,
                    output
                        .data
                        .to_vec()
                        .get(x as usize)
                        .unwrap()
                        .get(z as usize)
                        .unwrap()
                        .clone()
                        / output.max
                        - 1.0,
                    pdistance * z as f32 - depth as f32 / 2. * pdistance,
                ]);
                array.push([
                    pdistance * (x + 1) as f32 - width as f32 / 2. * pdistance,
                    output
                        .data
                        .to_vec()
                        .get((x + 1) as usize)
                        .unwrap()
                        .get(z as usize)
                        .unwrap()
                        .clone()
                        / output.max
                        - 1.0,
                    pdistance * z as f32 - depth as f32 / 2. * pdistance,
                ]);
                array.push([
                    pdistance * x as f32 - width as f32 / 2. * pdistance,
                    output
                        .data
                        .to_vec()
                        .get(x as usize)
                        .unwrap()
                        .get((z + 1) as usize)
                        .unwrap()
                        .clone()
                        / output.max
                        - 1.0,
                    pdistance * (z + 1) as f32 - depth as f32 / 2. * pdistance,
                ]);
                array.push([
                    pdistance * (x + 1) as f32 - width as f32 / 2. * pdistance,
                    output
                        .data
                        .to_vec()
                        .get((x + 1) as usize)
                        .unwrap()
                        .get((z + 1) as usize)
                        .unwrap()
                        .clone()
                        / output.max
                        - 1.0,
                    pdistance * (z + 1) as f32 - depth as f32 / 2. * pdistance,
                ]);
                array.push([
                    pdistance * (x + 1) as f32 - width as f32 / 2. * pdistance,
                    output
                        .data
                        .to_vec()
                        .get((x + 1) as usize)
                        .unwrap()
                        .get(z as usize)
                        .unwrap()
                        .clone()
                        / output.max
                        - 1.0,
                    pdistance * z as f32 - depth as f32 / 2. * pdistance,
                ]);
                array.push([
                    pdistance * x as f32 - width as f32 / 2. * pdistance,
                    output
                        .data
                        .to_vec()
                        .get(x as usize)
                        .unwrap()
                        .get((z + 1) as usize)
                        .unwrap()
                        .clone()
                        / output.max
                        - 1.0,
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

        let mut indices: Vec<Vertex> = vec![[0.0, 0.0, 0.0]; width as usize];

        for x in 0..width {
            for z in 0..depth {}
        }

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
        self.shader_program
            .set_uniform("model", Uniform::Mat4(glm::Mat4::identity()));
        let projection =
            glm::perspective(camera.fov.to_radians(), 640 as f32 / 480 as f32, 0.1, 100.);
        self.shader_program
            .set_uniform("projection", Uniform::Mat4(projection));
        unsafe { gl::DrawArrays(gl::TRIANGLES, 0, (self.width * self.depth * 6) as i32) }
        // unsafe { gl::DrawArrays(gl::LINES, 0, (self.width * self.depth * 3) as i32) }
        // unsafe { gl::DrawArrays(gl::POINTS, 0, (self.width * self.depth * 3) as i32) }
    }
}
