use crate::na::{Vector2, Vector3};
use crate::{texture::Texture, vertex::Vertex};
use std::ptr::null;

#[derive(Clone, Copy)]
pub enum UsageType {
    Static = gl::STATIC_DRAW as isize,
    Dynamic = gl::DYNAMIC_DRAW as isize,
}

pub struct Mesh {
    vao: u32,
    vbo: u32,
    ebo: u32,
    vertex_buffer_size: usize,
    face_buffer_size: usize,
    total_faces: usize,
    usage_type: UsageType,
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
                (self.total_faces * 3) as i32,
                gl::UNSIGNED_INT,
                0 as *const std::ffi::c_void,
            )
        }
    }

    pub fn from_tobj(obj: &tobj::Mesh, usage_type: UsageType) -> Self {
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

        Self::from_vertices(&vertices, &faces, usage_type)
    }

    pub fn from_vertices(
        vertices: &[Vertex],
        faces: &[Vector3<u32>],
        usage_type: UsageType,
    ) -> Self {
        let mut mesh = Self {
            vao: 0,
            vbo: 0,
            ebo: 0,
            vertex_buffer_size: vertices.len(),
            face_buffer_size: faces.len(),
            total_faces: faces.len(),
            usage_type,
        };
        unsafe {
            gl::GenVertexArrays(1, &mut mesh.vao);
            gl::GenBuffers(1, &mut mesh.vbo);
            gl::GenBuffers(1, &mut mesh.ebo);

            gl::BindVertexArray(mesh.vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, mesh.vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * std::mem::size_of::<Vertex>()) as isize,
                vertices.as_ptr() as *const gl::types::GLvoid,
                usage_type as u32,
            );

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, mesh.ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (faces.len() * std::mem::size_of::<Vector3<u32>>()) as isize,
                faces.as_ptr() as *const gl::types::GLvoid,
                usage_type as u32,
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

    pub fn update_vertices(&mut self, vertices: &[Vertex], faces: &[Vector3<u32>]) {
        unsafe {
            gl::BindVertexArray(self.vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
        }
        if vertices.len() > self.vertex_buffer_size {
            unsafe {
                gl::BufferData(
                    gl::ARRAY_BUFFER,
                    (vertices.len() * std::mem::size_of::<Vertex>()) as isize,
                    vertices.as_ptr() as *const gl::types::GLvoid,
                    self.usage_type as u32,
                );
            }

            self.vertex_buffer_size = vertices.len();
        } else {
            unsafe {
                gl::BufferSubData(
                    gl::ARRAY_BUFFER,
                    0,
                    (vertices.len() * std::mem::size_of::<Vertex>()) as isize,
                    vertices.as_ptr() as *const gl::types::GLvoid,
                )
            }
        }

        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
        }
        if faces.len() > self.face_buffer_size {
            unsafe {
                gl::BufferData(
                    gl::ELEMENT_ARRAY_BUFFER,
                    (faces.len() * std::mem::size_of::<Vector3<u32>>()) as isize,
                    faces.as_ptr() as *const gl::types::GLvoid,
                    self.usage_type as u32,
                );
            }

            self.face_buffer_size = faces.len();
        } else {
            unsafe {
                gl::BufferSubData(
                    gl::ELEMENT_ARRAY_BUFFER,
                    0,
                    (faces.len() * std::mem::size_of::<Vector3<u32>>()) as isize,
                    faces.as_ptr() as *const gl::types::GLvoid,
                );
            }
        }

        self.total_faces = faces.len();
    }
}

impl Clone for Mesh {
    fn clone(&self) -> Self {
        let mut mesh = Self {
            vao: 0,
            vbo: 0,
            ebo: 0,
            vertex_buffer_size: self.vertex_buffer_size,
            face_buffer_size: self.face_buffer_size,
            total_faces: self.total_faces,
            usage_type: self.usage_type,
        };

        unsafe {
            gl::GenVertexArrays(1, &mut mesh.vao);
            gl::GenBuffers(1, &mut mesh.vbo);
            gl::GenBuffers(1, &mut mesh.ebo);

            gl::BindBuffer(gl::COPY_READ_BUFFER, self.vbo);
            gl::BindBuffer(gl::COPY_WRITE_BUFFER, mesh.vbo);
            gl::BufferData(
                gl::COPY_WRITE_BUFFER,
                (mesh.vertex_buffer_size * std::mem::size_of::<Vertex>()) as isize,
                null(),
                mesh.usage_type as u32,
            );
            gl::CopyBufferSubData(
                gl::COPY_READ_BUFFER,
                gl::COPY_WRITE_BUFFER,
                0,
                0,
                (mesh.vertex_buffer_size * std::mem::size_of::<Vertex>()) as isize,
            );

            gl::BindBuffer(gl::COPY_READ_BUFFER, self.ebo);
            gl::BindBuffer(gl::COPY_WRITE_BUFFER, mesh.ebo);
            gl::BufferData(
                gl::COPY_WRITE_BUFFER,
                (mesh.face_buffer_size * std::mem::size_of::<Vector3<u32>>()) as isize,
                null(),
                mesh.usage_type as u32,
            );
            gl::CopyBufferSubData(
                gl::COPY_READ_BUFFER,
                gl::COPY_WRITE_BUFFER,
                0,
                0,
                (mesh.face_buffer_size * std::mem::size_of::<Vector3<u32>>()) as isize,
            );

            gl::BindVertexArray(mesh.vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, mesh.vbo);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, mesh.ebo);
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
