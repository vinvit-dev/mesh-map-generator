use app::App;
use core::{
    convert::TryInto,
    mem::{size_of, size_of_val},
};
use engine::{
    camera::Camera,
    keyboard_handler::{KeyBoardHandler, KeyBoardHandlerEvent},
    load_texture,
    shader_program::ShaderProgram,
};
use gl::types::GLint;
use glfw::Context;
use nalgebra_glm as glm;

pub mod app;

type Vertex = [f32; 3 + 3 + 2];

const WINDOW_WIDTH: u32 = 640;
const WINDOW_HEIGHT: u32 = 640;

const VERTICES: [Vertex; 24] = [
    // front
    [0.5, 0.5, 0.5, 1.0, 0.0, 0.0, 1.0, 1.0],  // top right 0
    [0.5, -0.5, 0.5, 0.0, 1.0, 0.0, 1.0, 0.0], // bottom right 1
    [-0.5, -0.5, 0.5, 0.0, 0.0, 1.0, 0.0, 0.0], // bottom left 2
    [-0.5, 0.5, 0.5, 0.5, 0.5, 0.0, -0.0, 1.0], // top left 3
    // back
    [0.5, 0.5, -0.5, 1.0, 0.0, 0.0, 1.0, 1.0], // top right 4
    [0.5, -0.5, -0.5, 0.0, 1.0, 0.0, 1.0, 0.0], // bottom right 5
    [-0.5, -0.5, -0.5, 0.0, 0.0, 1.0, 0.0, 0.0], // bottom left 6
    [-0.5, 0.5, -0.5, 0.5, 0.5, 0.0, 0.0, 1.0], // top left 7
    // bottom
    [0.5, -0.5, -0.5, 1.0, 0.0, 0.0, 1.0, 1.0], // top right 4
    [0.5, -0.5, 0.5, 0.0, 1.0, 0.0, 1.0, 0.0],  // bottom right 5
    [-0.5, -0.5, 0.5, 0.0, 0.0, 1.0, 0.0, 0.0], // bottom left 6
    [-0.5, -0.5, -0.5, 0.5, 0.5, 0.0, 0.0, 1.0], // top left 7
    // left
    [-0.5, 0.5, -0.5, 1.0, 0.0, 0.0, 1.0, 1.0], // top right 4
    [-0.5, -0.5, -0.5, 0.0, 1.0, 0.0, 1.0, 0.0], // bottom right 5
    [-0.5, -0.5, 0.5, 0.0, 0.0, 1.0, 0.0, 0.0], // bottom left 6
    [-0.5, 0.5, 0.5, 0.5, 0.5, 0.0, 0.0, 1.0],  // top left 7
    // right
    [0.5, 0.5, -0.5, 1.0, 0.0, 0.0, 1.0, 1.0], // top right 4
    [0.5, -0.5, -0.5, 0.0, 1.0, 0.0, 1.0, 0.0], // bottom right 5
    [0.5, -0.5, 0.5, 0.0, 0.0, 1.0, 0.0, 0.0], // bottom left 6
    [0.5, 0.5, 0.5, 0.5, 0.5, 0.0, 0.0, 1.0],  // top left 7
    // top
    [0.5, 0.5, -0.5, 1.0, 0.0, 0.0, 1.0, 1.0], // top right 4
    [0.5, 0.5, 0.5, 0.0, 1.0, 0.0, 1.0, 0.0],  // bottom right 5
    [-0.5, 0.5, 0.5, 0.0, 0.0, 1.0, 0.0, 0.0], // bottom left 6
    [-0.5, 0.5, -0.5, 0.5, 0.5, 0.0, 0.0, 1.0], // top left 7
];

const INDICES: [i32; 36] = [
    // front
    0, 1, 3, // first
    1, 2, 3, // second
    // back
    4, 5, 7, // first
    5, 6, 7, // second
    // bottom
    8, 9, 11, // first
    9, 10, 11, // second
    // left
    12, 13, 15, // first
    13, 14, 15, // second
    // right
    16, 17, 19, // first
    17, 18, 19, // second
    // top
    20, 21, 23, // first
    21, 22, 23, // second
];

fn main() {
    let bitmap = load_texture("./assets/wall.jpg");
    let (mut window, events, mut glfw) = App::init(WINDOW_WIDTH, WINDOW_HEIGHT, "Heightmap");

    window.set_key_polling(true);
    window.set_cursor_pos_polling(true);
    window.set_scroll_polling(true);
    window.set_framebuffer_size_callback(move |_, width, height| unsafe {
        gl::Viewport(0, 0, width, height);
    });
    window.set_cursor_mode(glfw::CursorMode::Disabled);

    unsafe {
        gl::ClearColor(0.2, 0.3, 0.3, 1.0);
    };

    let mut texture = 0;
    unsafe {
        gl::GenTextures(1, &mut texture);
        gl::BindTexture(gl::TEXTURE_2D, texture);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGBA as GLint,
            bitmap.width().try_into().unwrap(),
            bitmap.height().try_into().unwrap(),
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            bitmap.as_ptr().cast(),
        );
        gl::GenerateMipmap(gl::TEXTURE_2D);
    }

    unsafe {
        let mut vbo = 0;
        let mut ebo = 0;
        let mut vao = 0;
        gl::GenVertexArrays(1, &mut vao);
        assert_ne!(vao, 0);
        gl::BindVertexArray(vao);
        gl::GenBuffers(1, &mut vbo);
        assert_ne!(vbo, 0);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            size_of_val(&VERTICES) as isize,
            VERTICES.as_ptr().cast(),
            gl::STATIC_DRAW,
        );
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            size_of::<Vertex>().try_into().unwrap(),
            0 as *const _,
        );
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            size_of::<Vertex>().try_into().unwrap(),
            size_of::<[f32; 3]>() as *const _,
        );
        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(
            2,
            2,
            gl::FLOAT,
            gl::FALSE,
            size_of::<Vertex>().try_into().unwrap(),
            size_of::<[f32; 6]>() as *const _,
        );
        gl::EnableVertexAttribArray(2);
        gl::BindVertexArray(vao);
        gl::GenBuffers(1, &mut ebo);
        assert_ne!(ebo, 0);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            size_of_val(&INDICES) as isize,
            INDICES.as_ptr().cast(),
            gl::STATIC_DRAW,
        );

        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            size_of::<Vertex>().try_into().unwrap(),
            0 as *const _,
        );
    }
    let shader_program =
        ShaderProgram::from_vert_frag_files("./shaders/camera.vs", "./shaders/camera.fs").unwrap();
    shader_program.use_program();

    let mut camera = Camera::create_camera(WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32, 0.05, 5.);

    let mut delta_time: f32;
    let mut last_frame: f32 = 0.;

    let cubes_position: [glm::Vec3; 4] = [
        glm::vec3(0.0, 0.0, 0.0),
        glm::vec3(2.0, 5.0, -3.0),
        glm::vec3(-1.5, -2.2, -2.5),
        glm::vec3(-3.8, -2.0, -1.3),
    ];

    unsafe { gl::Enable(gl::DEPTH_TEST) };

    let mut keyboard_handler = KeyBoardHandler::new();
    while !window.should_close() {
        let current_frame = glfw.get_time() as f32;
        delta_time = current_frame - last_frame;
        last_frame = current_frame;
        // handle events this frame
        for (_, event) in glfw::flush_messages(&events) {
            let keyboard_event = keyboard_handler.handle_event(event.clone());
            match keyboard_event {
                Some(KeyBoardHandlerEvent::Pressed(key)) => {
                    if key == glfw::Key::Escape {
                        window.set_should_close(true);
                    }
                }
                Some(KeyBoardHandlerEvent::Released(_)) => {}
                None => {}
            }
            camera.handle_input(keyboard_handler.clone(), delta_time);
            camera.handle_cursor_pos(event.clone());
            // camera.handle_scroll(event.clone());
        }

        let projection = glm::perspective(
            camera.fov.to_radians(),
            WINDOW_WIDTH as f32 / WINDOW_HEIGHT as f32,
            0.1,
            100.,
        );
        shader_program.set_uniform(
            "projection",
            engine::shader_program::Uniform::Mat4(projection),
        );

        let mut model = glm::Mat4::identity();
        let time = glfw.get_time() as f32;
        model = glm::rotate(&model, time, &glm::vec3(0.5, 0.5, 0.0));
        shader_program.set_uniform("model", engine::shader_program::Uniform::Mat4(model));

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
        let view = camera.look_at();
        shader_program.set_uniform("view", engine::shader_program::Uniform::Mat4(view));
        for pos in cubes_position {
            let mut model = glm::Mat4::identity();
            model = glm::translate(&model, &pos);
            let time = glfw.get_time() as f32;
            model = glm::rotate(&model, time, &glm::vec3(0.5, 0.5, 0.0));
            shader_program.set_uniform("model", engine::shader_program::Uniform::Mat4(model));

            unsafe {
                gl::DrawElements(gl::TRIANGLES, 36, gl::UNSIGNED_INT, 0 as *const _);
            }
        }
        window.swap_buffers();
        glfw.poll_events();
    }
}
