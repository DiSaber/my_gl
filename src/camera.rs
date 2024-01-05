use crate::na::{Matrix4, Orthographic3, Perspective3};
use crate::{mesh_object::MeshObject, transform::Transform};
use palette::LinSrgba;

pub enum CameraType {
    Perspective(Perspective3<f32>),
    Orthographic(Orthographic3<f32>),
}

pub enum OrthographicType {
    UI,
    World { width: f32, height: f32 },
}

pub struct Camera {
    pub transform: Transform,
    pub camera_type: CameraType,
    pub clear_color: LinSrgba,
}

impl Camera {
    pub fn new_perspective(
        fov: f32,
        screen_width: i32,
        screen_height: i32,
        near_clipping_plane: f32,
        far_clipping_plane: f32,
        clear_color: LinSrgba,
    ) -> Self {
        Self {
            transform: Default::default(),
            camera_type: CameraType::Perspective(Perspective3::new(
                (screen_width as f32) / (screen_height as f32),
                fov.to_radians(),
                near_clipping_plane,
                far_clipping_plane,
            )),
            clear_color,
        }
    }

    pub fn new_orthographic(
        orthographic_type: OrthographicType,
        near_clipping_plane: f32,
        far_clipping_plane: f32,
        clear_color: LinSrgba,
    ) -> Self {
        let (left, right, top, bottom) = match orthographic_type {
            OrthographicType::UI => (0.0, 1.0, 0.0, 1.0),
            OrthographicType::World { width, height } => {
                (-width / 2.0, width / 2.0, height / 2.0, -height / 2.0)
            }
        };

        Self {
            transform: Default::default(),
            camera_type: CameraType::Orthographic(Orthographic3::new(
                left,
                right,
                bottom,
                top,
                near_clipping_plane,
                far_clipping_plane,
            )),
            clear_color,
        }
    }

    pub fn draw_objects(&self, objects: &[&MeshObject]) {
        for object in objects {
            object.draw(self);
        }
    }

    pub fn get_projection_matrix(&self) -> Matrix4<f32> {
        match self.camera_type {
            CameraType::Perspective(perspective) => perspective.into(),
            CameraType::Orthographic(orthographic) => orthographic.into(),
        }
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

    /// Only applies to perspective cameras
    pub fn set_screen_size(&mut self, screen_width: i32, screen_height: i32) {
        if let CameraType::Perspective(perspective) = &mut self.camera_type {
            unsafe {
                gl::Viewport(0, 0, screen_width, screen_height);
            }
            perspective.set_aspect((screen_width as f32) / (screen_height as f32));
        }
    }

    /// Only applies to orthographic cameras
    pub fn set_orthographic_type(&mut self, orthographic_type: OrthographicType) {
        if let CameraType::Orthographic(orthographic) = &mut self.camera_type {
            let (left, right, top, bottom) = match orthographic_type {
                OrthographicType::UI => (0.0, 1.0, 0.0, 1.0),
                OrthographicType::World { width, height } => {
                    (-width / 2.0, width / 2.0, height / 2.0, -height / 2.0)
                }
            };

            orthographic.set_left_and_right(left, right);
            orthographic.set_bottom_and_top(bottom, top);
        }
    }
}
