use std::collections::HashMap;

use crate::{na::Vector2, FilterMode, Texture, WrapMode};
use ab_glyph::{point, Font as ab_Font, FontRef, InvalidFont, ScaleFont};
use image::{DynamicImage, GenericImage, Rgba};
// use rusttype::{point, Font as RustFont, Scale};

const CHARACTER_SET: &str = " !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~";

#[derive(Debug, Clone, Copy)]
pub struct Character {
    pub bottom_left_tex_coord: Vector2<f32>,
    pub top_right_tex_coord: Vector2<f32>,
    pub width: f32,
    pub bearing_x: f32,
    pub advance: f32,
}

pub struct Font {
    font_texture: Texture,
    character_map: HashMap<char, Character>,
    font_height: f32,
    line_distance: f32,
}

impl Font {
    pub fn from_bytes(font_bytes: &[u8]) -> Result<Self, InvalidFont> {
        let font = FontRef::try_from_slice(font_bytes)?;
        let font = font.as_scaled(Self::default_font_size());

        let texture_width = CHARACTER_SET
            .chars()
            .map(|c| font.h_advance(font.glyph_id(c)))
            .sum::<f32>();
        let mut texture = DynamicImage::new_rgba8(texture_width as u32, font.height() as u32);
        let mut character_map = HashMap::<char, Character>::new();
        let mut total_advance = 0.0_f32;

        for char in CHARACTER_SET.chars() {
            let glyph_id = font.glyph_id(char);
            let advance = font.h_advance(glyph_id);
            let bearing_x = font.h_side_bearing(glyph_id);

            if let Some(glyph) = font.outline_glyph(
                glyph_id.with_scale_and_position(font.scale, point(total_advance, 0.0)),
            ) {
                let bottom_left_tex_coord = Vector2::new(
                    (glyph.px_bounds().min.x + total_advance) / texture_width,
                    1.0,
                );
                let top_right_tex_coord = Vector2::new(
                    (glyph.px_bounds().max.x + total_advance) / texture_width,
                    0.0,
                );
                character_map.insert(
                    char,
                    Character {
                        bottom_left_tex_coord,
                        top_right_tex_coord,
                        width: glyph.px_bounds().max.x - glyph.px_bounds().min.x,
                        bearing_x,
                        advance,
                    },
                );
                glyph.draw(|x, y, v| {
                    texture.put_pixel(
                        x + total_advance as u32,
                        y,
                        Rgba([255, 255, 255, (v * 255.0) as u8]),
                    )
                });
            } else if char == ' ' {
                character_map.insert(
                    char,
                    Character {
                        bottom_left_tex_coord: Vector2::zeros(),
                        top_right_tex_coord: Vector2::zeros(),
                        width: 0.0,
                        bearing_x,
                        advance,
                    },
                );
            }

            total_advance += advance;
        }

        texture.save("./font_output.png").unwrap();

        Ok(Self {
            font_texture: Texture::from_image(
                texture,
                WrapMode::ClampToEdge,
                FilterMode::Linear,
                FilterMode::Linear,
            ),
            character_map,
            font_height: font.height(),
            line_distance: font.height() + font.line_gap(),
        })
    }

    pub fn get_character_map(&self) -> &HashMap<char, Character> {
        &self.character_map
    }

    pub fn get_texture(&self) -> &Texture {
        &self.font_texture
    }

    pub fn get_line_distance(&self) -> f32 {
        self.line_distance
    }

    pub fn get_font_height(&self) -> f32 {
        self.font_height
    }

    pub fn default_font_size() -> f32 {
        32.0
    }

    pub fn font_scale(font_size: f32) -> f32 {
        font_size / Self::default_font_size()
    }
}
