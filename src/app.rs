use glfw::Context;

pub struct App;

impl App {
    pub fn init(
        s_width: u32,
        s_height: u32,
        title: &str,
    ) -> (
        glfw::PWindow,
        glfw::GlfwReceiver<(f64, glfw::WindowEvent)>,
        glfw::Glfw,
    ) {
        // Init glfw
        use glfw::fail_on_errors;
        let mut glfw = glfw::init(fail_on_errors!()).unwrap();

        // Set OpenGlLproperties
        glfw.window_hint(glfw::WindowHint::ContextVersionMajor(3));
        glfw.window_hint(glfw::WindowHint::ContextVersionMinor(3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));

        // Init window
        let (mut window, events) = glfw
            .create_window(s_width, s_height, title, glfw::WindowMode::Windowed)
            .expect("Fail to create widnow");
        window.make_current();

        // Init OpenGL
        gl::load_with(|f_name| glfw.get_proc_address_raw(f_name));

        (window, events, glfw)
    }
}
