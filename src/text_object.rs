use palette::LinSrgba;

use crate::{
    na::{Matrix4, Vector2, Vector3, Vector4},
    Font, GameObject, Mesh, ProgramValue, ShaderProgram, Transform, UsageType, Vertex,
};

pub struct TextObject<'a> {
    pub shader_program: &'a ShaderProgram,
    pub transform: Transform,
    pub text: String,
    pub font_size: u32,
    pub text_color: LinSrgba,
    font: &'a Font,
    internal_text: String,
    internal_font_size: u32,
    internal_mesh: Mesh,
}

impl<'a> TextObject<'a> {
    pub fn new(
        initial_text: String,
        font_size: u32,
        text_color: LinSrgba,
        shader_program: &'a ShaderProgram,
        font: &'a Font,
        usage_type: UsageType,
    ) -> Self {
        let (vertices, faces) = Self::generate_mesh(font, &initial_text, font_size);

        Self {
            shader_program,
            transform: Default::default(),
            text: initial_text.clone(),
            font_size,
            font,
            text_color,
            internal_text: initial_text,
            internal_font_size: font_size,
            internal_mesh: Mesh::from_vertices(&vertices, &faces, usage_type),
        }
    }

    fn generate_mesh(
        font: &'a Font,
        text: &str,
        font_size: u32,
    ) -> (Vec<Vertex>, Vec<Vector3<u32>>) {
        let character_map = font.get_character_map();
        let mut vertices = Vec::<Vertex>::with_capacity(text.chars().count() * 4);
        let mut faces = Vec::<Vector3<u32>>::with_capacity(text.chars().count() * 2);

        let font_scale = Font::font_scale(font_size as f32);

        let mut x = 0.0_f32;
        let y = 0.0_f32;
        for char in text.chars() {
            let char_info = character_map[&char];

            if !char.is_whitespace() {
                let char_x = x + (char_info.bearing_x * font_scale);
                let char_y = y;

                let char_width = char_info.width * font_scale;
                let char_height = font.get_char_height() * font_scale;

                let face_offset = (vertices.len() / 4) as u32;
                // TODO: Add multiline support
                vertices.append(&mut vec![
                    // Top right
                    Vertex::tex(
                        Vector3::new(char_x + char_width, char_y, 0.0),
                        char_info.top_right_tex_coord,
                    ),
                    // Bottom right
                    Vertex::tex(
                        Vector3::new(char_x + char_width, char_y + char_height, 0.0),
                        Vector2::new(
                            char_info.top_right_tex_coord.x,
                            char_info.bottom_left_tex_coord.y,
                        ),
                    ),
                    // Bottom left
                    Vertex::tex(
                        Vector3::new(char_x, char_y + char_height, 0.0),
                        char_info.bottom_left_tex_coord,
                    ),
                    // Top left
                    Vertex::tex(
                        Vector3::new(char_x, char_y, 0.0),
                        Vector2::new(
                            char_info.bottom_left_tex_coord.x,
                            char_info.top_right_tex_coord.y,
                        ),
                    ),
                ]);

                faces.append(&mut vec![
                    Vector3::new(
                        0 + (face_offset * 4),
                        1 + (face_offset * 4),
                        3 + (face_offset * 4),
                    ),
                    Vector3::new(
                        1 + (face_offset * 4),
                        2 + (face_offset * 4),
                        3 + (face_offset * 4),
                    ),
                ]);
            }

            x += char_info.advance * font_scale;
        }

        (vertices, faces)
    }
}

impl<'a> GameObject for TextObject<'a> {
    fn draw(&mut self, camera: &crate::Camera) {
        self.shader_program.set_value(
            "transform",
            ProgramValue::Mat4(
                camera.get_projection_matrix()
                    * camera.get_transform_matrix()
                    * self.get_transform_matrix(),
            ),
        );
        self.shader_program.set_value(
            "text_color",
            ProgramValue::Vec4(Vector4::new(
                self.text_color.red,
                self.text_color.green,
                self.text_color.blue,
                self.text_color.alpha,
            )),
        );

        if self.internal_text != self.text || self.internal_font_size != self.font_size {
            let (vertices, faces) = Self::generate_mesh(self.font, &self.text, self.font_size);
            self.internal_mesh.update_vertices(&vertices, &faces);

            self.internal_text = self.text.clone();
            self.internal_font_size = self.font_size;
        }

        self.internal_mesh.draw(&[self.font.get_texture()]);
    }

    fn get_transform(&self) -> Transform {
        self.transform
    }

    fn get_transform_matrix(&self) -> Matrix4<f32> {
        self.transform.to_matrix(false)
    }
}
