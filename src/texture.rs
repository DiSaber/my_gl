use image::DynamicImage;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WrapMode {
    Repeat = gl::REPEAT as isize,
    MirroredRepeat = gl::MIRRORED_REPEAT as isize,
    ClampToEdge = gl::CLAMP_TO_EDGE as isize,
    ClampToBorder = gl::CLAMP_TO_BORDER as isize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilterMode {
    Nearest = gl::NEAREST as isize,
    Linear = gl::LINEAR as isize,
    NearestMipMapNearest = gl::NEAREST_MIPMAP_NEAREST as isize,
    LinearMipMapNearest = gl::LINEAR_MIPMAP_NEAREST as isize,
    NearestMipMapLinear = gl::NEAREST_MIPMAP_LINEAR as isize,
    LinearMipMapLinear = gl::LINEAR_MIPMAP_LINEAR as isize,
}

impl FilterMode {
    pub fn is_mipmap(&self) -> bool {
        *self == FilterMode::NearestMipMapNearest
            || *self == FilterMode::LinearMipMapNearest
            || *self == FilterMode::NearestMipMapLinear
            || *self == FilterMode::LinearMipMapLinear
    }
}

pub struct Texture {
    id: u32,
}

impl Texture {
    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn from_image(
        image: DynamicImage,
        wrap_mode: WrapMode,
        min_filter_mode: FilterMode,
        mag_filter_mode: FilterMode,
    ) -> Self {
        let mut texture = Self { id: 0 };

        unsafe {
            gl::GenTextures(1, &mut texture.id);
            gl::BindTexture(gl::TEXTURE_2D, texture.id);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, wrap_mode as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, wrap_mode as i32);
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MIN_FILTER,
                min_filter_mode as i32,
            );
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MAG_FILTER,
                mag_filter_mode as i32,
            );
        }

        let image = image.into_rgba8();

        unsafe {
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                image.width() as i32,
                image.height() as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                image.as_raw().as_ptr() as *const gl::types::GLvoid,
            );

            if min_filter_mode.is_mipmap() || mag_filter_mode.is_mipmap() {
                gl::GenerateMipmap(gl::TEXTURE_2D);
            }
        }

        texture
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.id);
        }
    }
}
