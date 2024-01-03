use crate::MouseMode;
use glfw::{Action, Context, Glfw, Key, PWindow, WindowMode};

pub struct Game {
    glfw: Glfw,
    window: PWindow,
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

    pub fn run_update(&mut self, mut update_fn: impl FnMut(&mut Self, f64)) {
        let mut last_frame_time = self.get_time();

        while !self.window.should_close() {
            let current_time = self.get_time();
            update_fn(self, current_time - last_frame_time);
            last_frame_time = current_time;

            self.window.swap_buffers();
            self.glfw.poll_events();
        }
    }

    pub fn close_window(&mut self) {
        self.window.set_should_close(true);
    }

    pub fn get_time(&self) -> f64 {
        self.glfw.get_time()
    }

    pub fn get_key(&self, key: Key) -> Action {
        self.window.get_key(key)
    }

    pub fn get_framebuffer_size(&self) -> (i32, i32) {
        self.window.get_framebuffer_size()
    }

    pub fn set_mouse_mode(&mut self, mode: MouseMode) {
        self.window.set_cursor_mode(mode);
    }

    pub fn get_mouse_position(&self) -> (f64, f64) {
        self.window.get_cursor_pos()
    }
}
