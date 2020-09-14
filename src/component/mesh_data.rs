use specs::prelude::*;

pub struct MeshData {
    vertices: Vec<f32>,
    vao_id: gl::types::GLuint,
    vbo_ids: Vec<gl::types::GLuint>,
}

impl Component for MeshData {
    type Storage = DenseVecStorage<Self>;
}

impl MeshData {
    pub fn from_vertices(vertices: Vec<f32>) -> MeshData {
        let mut vertex_vbo: gl::types::GLuint = 0;
        let mut vao_id: gl::types::GLuint = 0;
        unsafe {
            gl::GenBuffers(1, &mut vertex_vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vertex_vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                vertices.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);

            gl::GenVertexArrays(1, &mut vao_id);
            gl::BindVertexArray(vao_id);
            gl::BindBuffer(gl::ARRAY_BUFFER, vertex_vbo);
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                (3 * std::mem::size_of::<f32>()) as gl::types::GLint,
                std::ptr::null(),
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        };

        MeshData {
            vertices,
            vao_id,
            vbo_ids: vec![vertex_vbo],
        }
    }

    pub fn get_vao_id(&self) -> gl::types::GLuint {
        self.vao_id
    }

    pub fn get_vertex_count(&self) -> i32 {
        (self.vertices.len() / 3) as i32
    }
}

impl Drop for MeshData {
    fn drop(&mut self) {}
}
