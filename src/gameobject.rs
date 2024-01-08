use crate::{na::Matrix4, Camera, Transform};

pub trait GameObject {
    fn draw(&mut self, camera: &Camera);
    fn get_transform(&self) -> Transform;
    fn get_transform_matrix(&self) -> Matrix4<f32>;
}
