use glfw::{Context, Glfw, PWindow, WindowMode};

pub struct Game {
    glfw: Glfw,
    pub window: PWindow,
}

impl Game {
    pub fn new(width: u32, height: u32, title: &str, mode: WindowMode<'_>) -> Self {
        let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();
        glfw.window_hint(glfw::WindowHint::ContextVersion(4, 6));
        glfw.window_hint(glfw::WindowHint::ContextCreationApi(
            glfw::ContextCreationApi::Native,
        ));

        let (mut window, _events) = glfw.create_window(width, height, title, mode).unwrap();

        window.make_current();
        gl::load_with(|s| window.get_proc_address(s));
        Self { glfw, window }
    }

    pub fn run_update(&mut self, mut update_fn: impl FnMut(f64)) {
        let mut last_frame_time = self.glfw.get_time();

        while !self.window.should_close() {
            let current_time = self.glfw.get_time();
            update_fn(current_time - last_frame_time);
            last_frame_time = current_time;

            self.window.swap_buffers();
            self.glfw.poll_events();
        }
    }

    pub fn get_time(&self) -> f64 {
        self.glfw.get_time()
    }
}
