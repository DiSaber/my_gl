use crate::{transform::Transform, Mesh, ShaderProgram};

pub struct TextObject<'a> {
    pub shader_program: &'a ShaderProgram,
    pub transform: Transform,
    pub text: String,
    internal_text: String,
    internal_mesh: Mesh,
    // More fields to be added later
}
