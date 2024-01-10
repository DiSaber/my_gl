use crate::{
    na::{Matrix4, Orthographic3, Perspective3, Vector2},
    GameObject, Transform,
};
use palette::LinSrgba;

#[derive(Clone, Copy)]
pub enum CameraType {
    Perspective(Perspective3<f32>),
    Orthographic(Orthographic3<f32>, OrthographicType),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OrthographicType {
    UI { height: f32 },
    World { height: f32 },
}

impl OrthographicType {
    pub fn is_ui(&self) -> bool {
        match self {
            OrthographicType::UI { .. } => true,
            _ => false,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Camera {
    pub transform: Transform,
    pub camera_type: CameraType,
    pub clear_color: LinSrgba,
    screen_size: Vector2<u32>,
}

impl Camera {
    pub fn new_perspective(
        fov: f32,
        screen_size: Vector2<u32>,
        near_clipping_plane: f32,
        far_clipping_plane: f32,
        clear_color: LinSrgba,
    ) -> Self {
        Self {
            transform: Default::default(),
            camera_type: CameraType::Perspective(Perspective3::new(
                (screen_size.x as f32) / (screen_size.y as f32),
                fov.to_radians(),
                near_clipping_plane,
                far_clipping_plane,
            )),
            clear_color,
            screen_size,
        }
    }

    pub fn new_orthographic(
        orthographic_type: OrthographicType,
        screen_size: Vector2<u32>,
        near_clipping_plane: f32,
        far_clipping_plane: f32,
        clear_color: LinSrgba,
    ) -> Self {
        let (left, right, top, bottom) = match orthographic_type {
            OrthographicType::UI { height } => {
                let width = height * ((screen_size.x as f32) / (screen_size.y as f32));
                (0.0, width, 0.0, height)
            }
            OrthographicType::World { height } => {
                let width = height * ((screen_size.x as f32) / (screen_size.y as f32));
                (-width / 2.0, width / 2.0, height / 2.0, -height / 2.0)
            }
        };

        Self {
            transform: Default::default(),
            camera_type: CameraType::Orthographic(
                Orthographic3::new(
                    left,
                    right,
                    bottom,
                    top,
                    near_clipping_plane,
                    far_clipping_plane,
                ),
                orthographic_type,
            ),
            clear_color,
            screen_size: Vector2::new((left - right).abs() as u32, (top - bottom).abs() as u32),
        }
    }

    pub fn draw_objects(&self, objects: &mut [&mut dyn GameObject]) {
        if let CameraType::Orthographic(_, orthographic_type) = self.camera_type {
            if orthographic_type.is_ui() {
                unsafe {
                    gl::Disable(gl::DEPTH_TEST);
                }

                objects.sort_unstable_by(|a, b| {
                    a.get_transform()
                        .position
                        .z
                        .partial_cmp(&b.get_transform().position.z)
                        .unwrap()
                });

                for object in objects {
                    object.draw(self);
                }
                return;
            }
        }

        unsafe {
            gl::Enable(gl::DEPTH_TEST);
        }

        for object in objects {
            object.draw(self);
        }
    }

    pub fn get_projection_matrix(&self) -> Matrix4<f32> {
        match self.camera_type {
            CameraType::Perspective(perspective) => perspective.into(),
            CameraType::Orthographic(orthographic, ..) => orthographic.into(),
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

    pub fn set_screen_size(&mut self, screen_size: Vector2<u32>) {
        unsafe {
            gl::Viewport(0, 0, screen_size.x as i32, screen_size.y as i32);
        }

        if let CameraType::Perspective(perspective) = &mut self.camera_type {
            self.screen_size = screen_size;
            perspective.set_aspect((screen_size.x as f32) / (screen_size.y as f32));
        } else if let CameraType::Orthographic(orthographic, orthographic_type) =
            &mut self.camera_type
        {
            let (left, right, top, bottom) = match *orthographic_type {
                OrthographicType::UI { height } => {
                    let width = height * ((screen_size.x as f32) / (screen_size.y as f32));
                    (0.0, width, 0.0, height)
                }
                OrthographicType::World { height } => {
                    let width = height * ((screen_size.x as f32) / (screen_size.y as f32));
                    (-width / 2.0, width / 2.0, height / 2.0, -height / 2.0)
                }
            };

            self.screen_size =
                Vector2::new((left - right).abs() as u32, (top - bottom).abs() as u32);

            orthographic.set_left_and_right(left, right);
            orthographic.set_bottom_and_top(bottom, top);
        }
    }

    pub fn get_screen_size(&self) -> Vector2<u32> {
        self.screen_size
    }
}
