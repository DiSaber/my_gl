use nalgebra::Matrix4;

use crate::{
    camera::Camera,
    mesh::Mesh,
    shader_program::{ProgramValue, ShaderProgram},
    texture::Texture,
    transform::Transform,
};

pub struct MeshObject<'a> {
    pub mesh: &'a Mesh,
    pub textures: Vec<&'a Texture>,
    pub shader_program: &'a ShaderProgram,
    pub transform: Transform,
}

impl<'a> MeshObject<'a> {
    pub fn new(
        mesh: &'a Mesh,
        textures: &[&'a Texture],
        shader_program: &'a ShaderProgram,
    ) -> Self {
        let mesh_object = Self {
            mesh,
            textures: Vec::from(textures),
            shader_program,
            transform: Default::default(),
        };

        for (i, _) in mesh_object.textures.iter().enumerate() {
            mesh_object.shader_program.set_value(
                &("texture".to_owned() + &i.to_string()),
                ProgramValue::Int(i as i32),
            )
        }

        mesh_object
    }

    pub fn draw(&self, camera: &Camera) {
        self.shader_program.set_value(
            "transform",
            ProgramValue::Mat4(
                camera.get_projection_matrix()
                    * camera.get_transform_matrix()
                    * self.get_transform_matrix(),
            ),
        );
        self.mesh.draw(&self.textures);
    }

    pub fn get_transform_matrix(&self) -> Matrix4<f32> {
        self.transform.to_matrix(false)
    }
}
