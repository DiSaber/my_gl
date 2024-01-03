use na::{Matrix4, UnitQuaternion, Vector3};

#[derive(Debug, Clone, Copy)]
pub struct Transform {
    pub position: Vector3<f32>,
    pub rotation: UnitQuaternion<f32>,
    pub scale: Vector3<f32>,
}

impl Transform {
    pub fn to_matrix(&self, is_camera: bool) -> Matrix4<f32> {
        let position = Matrix4::identity().prepend_translation(&self.position);
        let rotation = self.rotation.to_homogeneous();
        let scale = Matrix4::identity().prepend_nonuniform_scaling(&self.scale);

        if is_camera {
            (position * rotation).try_inverse().unwrap()
        } else {
            position * rotation * scale
        }
    }

    pub fn set_euler_angles(&mut self, euler_angles: &Vector3<f32>) {
        self.rotation =
            UnitQuaternion::from_euler_angles(euler_angles.x, euler_angles.y, euler_angles.z);
    }

    pub fn set_euler_angles_deg(&mut self, euler_angles: &Vector3<f32>) {
        self.set_euler_angles(&Vector3::new(
            euler_angles.x.to_radians(),
            euler_angles.y.to_radians(),
            euler_angles.z.to_radians(),
        ));
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            position: Vector3::zeros(),
            rotation: UnitQuaternion::identity(),
            scale: Vector3::new(1.0, 1.0, 1.0),
        }
    }
}
