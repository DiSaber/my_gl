extern crate nalgebra as na;

pub mod camera;
pub mod mesh;
pub mod mesh_object;
pub mod shader;
pub mod shader_program;
pub mod texture;
pub mod transform;
pub mod utils;
pub mod vertex;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
