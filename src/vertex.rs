use na::{Vector2, Vector3};

#[repr(C)]
pub struct Vertex {
    pub position: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub tex_coord: Vector2<f32>,
}

impl Vertex {
    pub fn new(position: Vector3<f32>, normal: Vector3<f32>, tex_coord: Vector2<f32>) -> Self {
        Self {
            position,
            normal,
            tex_coord,
        }
    }

    pub fn tex(position: Vector3<f32>, tex_coord: Vector2<f32>) -> Self {
        Self {
            position,
            normal: Vector3::zeros(),
            tex_coord,
        }
    }

    pub fn lengths() -> [usize; 3] {
        [3, 3, 2]
    }
}
