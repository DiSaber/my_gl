use crate::{
    na::Matrix4,
    shader_program::{ProgramValue, ShaderProgram},
    Camera, GameObject, Mesh, Texture, Transform,
};

#[derive(Clone)]
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

        mesh_object
    }
}

impl<'a> GameObject for MeshObject<'a> {
    fn draw(&mut self, camera: &Camera) {
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

    fn get_transform(&self) -> Transform {
        self.transform
    }

    fn get_transform_matrix(&self) -> Matrix4<f32> {
        self.transform.to_matrix(false)
    }
}
