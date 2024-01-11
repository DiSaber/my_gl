use palette::LinSrgba;

use crate::{
    na::{Matrix4, Vector2, Vector3, Vector4},
    Alignment, Font, GUIObject, GameObject, Mesh, ProgramValue, ShaderProgram, Size, Transform,
    UsageType, Vertex,
};

pub struct TextObject<'a> {
    pub shader_program: &'a ShaderProgram,
    pub transform: Transform,
    text: String,
    font_size: u32,
    pub text_color: LinSrgba,
    pub size: Size,
    pub alignment: Alignment,
    font: &'a Font,
    must_update: bool,
    internal_size: Vector2<f32>,
    internal_mesh: Mesh,
}

impl<'a> TextObject<'a> {
    pub fn new(
        initial_text: String,
        font_size: u32,
        shader_program: &'a ShaderProgram,
        font: &'a Font,
        usage_type: UsageType,
    ) -> Self {
        let (vertices, faces, mesh_size) = Self::generate_mesh(font, &initial_text, font_size);

        Self {
            shader_program,
            transform: Default::default(),
            text: initial_text.clone(),
            font_size,
            font,
            text_color: LinSrgba::new(1.0, 1.0, 1.0, 1.0),
            size: Size::Auto,
            alignment: Alignment::TopLeft,
            must_update: false,
            internal_size: mesh_size,
            internal_mesh: Mesh::from_vertices(&vertices, &faces, usage_type),
        }
    }

    pub fn with_text_color(mut self, text_color: LinSrgba) -> Self {
        self.text_color = text_color;
        self
    }

    pub fn with_size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    pub fn with_alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
        self
    }

    pub fn get_text(&self) -> &str {
        &self.text
    }

    pub fn set_text(&mut self, text: String) {
        self.text = text;
        self.must_update = true;
    }

    pub fn get_font_size(&self) -> u32 {
        self.font_size
    }

    pub fn set_font_size(&mut self, font_size: u32) {
        self.font_size = font_size;
        self.must_update = true;
    }

    pub fn get_font(&self) -> &Font {
        self.font
    }

    pub fn set_font(&mut self, font: &'a Font) {
        self.font = font;
        self.must_update = true;
    }

    fn generate_mesh(
        font: &'a Font,
        text: &str,
        font_size: u32,
    ) -> (Vec<Vertex>, Vec<Vector3<u32>>, Vector2<f32>) {
        let character_map = font.get_character_map();
        let mut vertices = Vec::<Vertex>::with_capacity(text.chars().count() * 4);
        let mut faces = Vec::<Vector3<u32>>::with_capacity(text.chars().count() * 2);
        let mut size = Vector2::zeros();

        let font_scale = Font::font_scale(font_size as f32);

        let mut x = 0.0_f32;
        let mut y = 0.0_f32;
        for char in text.chars() {
            if let Some(char_info) = character_map.get(&char) {
                if !char.is_whitespace() {
                    let char_x = x + (char_info.bearing_x * font_scale);
                    let char_y = y;
                    let char_width = char_info.width * font_scale;
                    let char_height = font.get_char_height() * font_scale;

                    let face_offset = (vertices.len() / 4) as u32;

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
                if x > size.x {
                    size.x = x;
                }
            } else if char == '\n' {
                x = 0.0;
                y += font.get_line_distance() * font_scale;
            }
        }

        size.y = y + (font.get_line_distance() * font_scale);

        (vertices, faces, size)
    }
}

impl<'a> GameObject for TextObject<'a> {
    fn draw(&mut self, camera: &crate::Camera) {
        self.force_update();

        self.shader_program.set_value(
            "transform",
            ProgramValue::Mat4(
                camera.get_projection_matrix()
                    * camera.get_transform_matrix()
                    * self.get_aligned_transform_matrix(camera.get_screen_size()),
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

        self.internal_mesh.draw(&[self.font.get_texture()]);
    }

    fn get_transform(&self) -> Transform {
        self.transform
    }

    fn get_transform_matrix(&self) -> Matrix4<f32> {
        self.transform.to_matrix(false)
    }
}

impl<'a> GUIObject for TextObject<'a> {
    fn get_aligned_transform_matrix(&self, screen_size: Vector2<u32>) -> Matrix4<f32> {
        let mut transform = self.transform;
        transform.position = self.alignment.align_position(
            self.transform.position,
            Vector2::new(
                self.get_size().x * self.transform.scale.x,
                self.get_size().y * self.transform.scale.y,
            ),
            screen_size,
        );
        transform.to_matrix(false)
    }

    fn force_update(&mut self) {
        if self.must_update {
            let (vertices, faces, mesh_size) =
                Self::generate_mesh(self.font, &self.text, self.font_size);
            self.internal_mesh.update_vertices(&vertices, &faces);

            self.must_update = false;
            self.internal_size = mesh_size;
        }
    }

    fn get_size(&self) -> Vector2<f32> {
        // Get the most up-to-date size
        match self.size {
            Size::Auto => self.internal_size,
            Size::Manual(size) => size,
        }
    }
}
