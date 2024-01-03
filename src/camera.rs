use crate::{mesh_object::MeshObject, transform::Transform};
use na::{Matrix4, Perspective3};
use palette::LinSrgba;

pub struct Camera {
    pub transform: Transform,
    perspective: Perspective3<f32>,
    pub clear_color: LinSrgba,
}

impl Camera {
    pub fn new(
        fov: f32,
        screen_width: i32,
        screen_height: i32,
        near_clipping_plane: f32,
        far_clipping_plane: f32,
        clear_color: LinSrgba,
    ) -> Self {
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
        }

        Self {
            transform: Default::default(),
            perspective: Perspective3::new(
                (screen_width as f32) / (screen_height as f32),
                fov.to_radians(),
                near_clipping_plane,
                far_clipping_plane,
            ),
            clear_color,
        }
    }

    pub fn draw_objects(&self, objects: &[&MeshObject]) {
        for object in objects {
            object.draw(self);
        }
    }

    pub fn get_projection_matrix(&self) -> Matrix4<f32> {
        self.perspective.into()
    }

    pub fn get_transform_matrix(&self) -> Matrix4<f32> {
        self.transform.to_matrix(true)
    }

    pub fn clear(&self) {
        unsafe {
            gl::ClearColor(
                self.clear_color.red,
                self.clear_color.green,
                self.clear_color.blue,
                self.clear_color.alpha,
            );
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }

    pub fn set_screen_size(&mut self, screen_width: i32, screen_height: i32) {
        unsafe {
            gl::Viewport(0, 0, screen_width, screen_height);
        }
        self.perspective
            .set_aspect((screen_width as f32) / (screen_height as f32));
    }
}
