use crate::{
    na::{Matrix4, Vector2, Vector3},
    GameObject,
};

pub trait GUIObject: GameObject {
    fn get_aligned_transform_matrix(&self, screen_size: Vector2<u32>) -> Matrix4<f32>;
    /// This operation is potentially expensive! Updates any internal gui state such as auto sizing.
    fn force_update(&mut self);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Alignment {
    TopLeft,
    TopCenter,
    TopRight,
    CenterLeft,
    Center,
    CenterRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
}

impl Alignment {
    pub fn align_position(
        &self,
        position: Vector3<f32>,
        size: Vector2<f32>,
        screen_size: Vector2<u32>,
    ) -> Vector3<f32> {
        match self {
            Alignment::TopLeft => position,
            Alignment::TopCenter => Vector3::new(
                ((screen_size.x as f32) / 2.0) - (size.x / 2.0) + position.x,
                position.y,
                position.z,
            ),
            Alignment::TopRight => Vector3::new(
                (screen_size.x as f32) - size.x + position.x,
                position.y,
                position.z,
            ),
            Alignment::CenterLeft => Vector3::new(
                position.x,
                ((screen_size.y as f32) / 2.0) - (size.y / 2.0) + position.y,
                position.z,
            ),
            Alignment::Center => Vector3::new(
                ((screen_size.x as f32) / 2.0) - (size.x / 2.0) + position.x,
                ((screen_size.y as f32) / 2.0) - (size.y / 2.0) + position.y,
                position.z,
            ),
            Alignment::CenterRight => Vector3::new(
                (screen_size.x as f32) - size.x + position.x,
                ((screen_size.y as f32) / 2.0) - (size.y / 2.0) + position.y,
                position.z,
            ),
            Alignment::BottomLeft => Vector3::new(
                position.x,
                (screen_size.y as f32) - size.y + position.y,
                position.z,
            ),
            Alignment::BottomCenter => Vector3::new(
                ((screen_size.x as f32) / 2.0) - (size.x / 2.0) + position.x,
                (screen_size.y as f32) - size.y + position.y,
                position.z,
            ),
            Alignment::BottomRight => Vector3::new(
                (screen_size.x as f32) - size.x + position.x,
                (screen_size.y as f32) - size.y + position.y,
                position.z,
            ),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Size {
    Auto,
    Manual(Vector2<f32>),
}
