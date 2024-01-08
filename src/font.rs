use std::collections::HashMap;

use crate::{na::Vector2, FilterMode, Texture, WrapMode};
use image::{DynamicImage, GenericImage, Rgba};
use rusttype::{point, Font as RustFont, Scale};

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
    pub fn from_bytes(font_bytes: &[u8]) -> Option<Self> {
        let font = RustFont::try_from_bytes(font_bytes)?;
        let scale = Scale::uniform(Self::default_font_size());
        let v_metrics = font.v_metrics(scale);

        let glyphs = font
            .layout(CHARACTER_SET, scale, point(0.0, 0.0))
            .collect::<Vec<_>>();
        let set_height = (v_metrics.ascent - v_metrics.descent).ceil();
        let set_width = {
            let min_x = glyphs
                .first()
                .map(|g| g.pixel_bounding_box().unwrap_or_default().min.x)
                .unwrap();
            let max_x = glyphs
                .last()
                .map(|g| g.pixel_bounding_box().unwrap_or_default().max.x)
                .unwrap();
            (max_x - min_x) as f32
        };

        let mut texture = DynamicImage::new_rgba8(set_width as u32, set_height as u32);
        let mut character_map = HashMap::<char, Character>::new();

        for (i, glyph) in glyphs.into_iter().enumerate() {
            let bounding_box = glyph.pixel_bounding_box().unwrap_or_default();
            let bottom_left_tex_coord = Vector2::new((bounding_box.min.x as f32) / set_width, 1.0);
            let top_right_tex_coord = Vector2::new((bounding_box.max.x as f32) / set_width, 0.0);
            let h_metrics = glyph.unpositioned().h_metrics();

            character_map.insert(
                CHARACTER_SET.chars().nth(i).unwrap(),
                Character {
                    bottom_left_tex_coord,
                    top_right_tex_coord,
                    width: (bounding_box.max.x - bounding_box.min.x) as f32,
                    bearing_x: h_metrics.left_side_bearing,
                    advance: h_metrics.advance_width,
                },
            );

            glyph.draw(|x, y, v| {
                texture.put_pixel(
                    x + bounding_box.min.x as u32,
                    y + bounding_box.min.y as u32,
                    Rgba([255, 255, 255, (v * 255.0) as u8]),
                )
            });
        }

        Some(Self {
            font_texture: Texture::from_image(
                texture,
                WrapMode::ClampToEdge,
                FilterMode::Linear,
                FilterMode::Linear,
            ),
            character_map,
            font_height: set_height,
            line_distance: set_height + v_metrics.line_gap,
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
