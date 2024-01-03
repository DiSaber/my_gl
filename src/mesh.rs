use crate::{texture::Texture, vertex::Vertex};
use na::{Vector2, Vector3};

pub struct Mesh {
    vao: u32,
    vbo: u32,
    ebo: u32,
    total_indices: i32,
}

impl Mesh {
    pub fn draw(&self, textures: &[&Texture]) {
        for (i, texture) in textures.iter().enumerate() {
            unsafe {
                gl::ActiveTexture(gl::TEXTURE0 + (i as u32));
                gl::BindTexture(gl::TEXTURE_2D, texture.get_id());
            }
        }

        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawElements(
                gl::TRIANGLES,
                self.total_indices,
                gl::UNSIGNED_INT,
                0 as *const std::ffi::c_void,
            )
        }
    }

    pub fn from_tobj(obj: &tobj::Mesh) -> Self {
        let vertices = (0..(obj.positions.len() / 3))
            .map(|i| {
                Vertex::new(
                    Vector3::new(
                        obj.positions[i * 3],
                        obj.positions[i * 3 + 1],
                        obj.positions[i * 3 + 2],
                    ),
                    if obj.normals.is_empty() {
                        Vector3::zeros()
                    } else {
                        Vector3::new(
                            obj.normals[i * 3],
                            obj.normals[i * 3 + 1],
                            obj.normals[i * 3 + 2],
                        )
                    },
                    if obj.texcoords.is_empty() {
                        Vector2::zeros()
                    } else {
                        Vector2::new(obj.texcoords[i * 2], obj.texcoords[i * 2 + 1])
                    },
                )
            })
            .collect::<Vec<Vertex>>();
        let faces = (0..(obj.indices.len() / 3))
            .map(|i| {
                Vector3::new(
                    obj.indices[i * 3],
                    obj.indices[i * 3 + 1],
                    obj.indices[i * 3 + 2],
                )
            })
            .collect::<Vec<Vector3<u32>>>();

        Self::from_vertices(vertices, faces)
    }

    pub fn from_vertices(vertices: Vec<Vertex>, faces: Vec<Vector3<u32>>) -> Self {
        let mut mesh = Self {
            vao: 0,
            vbo: 0,
            ebo: 0,
            total_indices: (faces.len() * 3) as i32,
        };
        unsafe {
            gl::GenBuffers(1, &mut mesh.vbo);
            gl::GenBuffers(1, &mut mesh.ebo);
            gl::GenVertexArrays(1, &mut mesh.vao);

            gl::BindVertexArray(mesh.vao);
        }

        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, mesh.vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * std::mem::size_of::<Vertex>()) as isize,
                vertices.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, mesh.ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (faces.len() * std::mem::size_of::<Vector3<u32>>()) as isize,
                faces.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );
        }

        let mut offset = 0;

        for (i, length) in Vertex::lengths().iter().enumerate() {
            unsafe {
                gl::VertexAttribPointer(
                    i as u32,
                    *length as i32,
                    gl::FLOAT,
                    gl::FALSE,
                    std::mem::size_of::<Vertex>() as i32,
                    offset as *const gl::types::GLvoid,
                );
                gl::EnableVertexAttribArray(i as u32);
            }

            offset += length * std::mem::size_of::<f32>();
        }

        mesh
    }
}

impl Drop for Mesh {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteBuffers(1, &self.ebo);
            gl::DeleteVertexArrays(1, &self.vao);
        }
    }
}
