use specs::prelude::*;

pub struct MeshData {
    vertices: Vec<f32>,
    indices: Vec<u32>,
    vao_id: gl::types::GLuint,
    vbo_ids: Vec<gl::types::GLuint>,
}

impl Component for MeshData {
    type Storage = DenseVecStorage<Self>;
}

impl MeshData {
    pub fn from_data(vertices: Vec<f32>, indices: Vec<u32>) -> MeshData {
        let mut vertex_vbo: gl::types::GLuint = 0;
        let mut indices_vbo: gl::types::GLuint = 0;
        let mut vao_id: gl::types::GLuint = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao_id);
            gl::BindVertexArray(vao_id);

            gl::GenBuffers(1, &mut indices_vbo);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, indices_vbo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * std::mem::size_of::<u32>()) as gl::types::GLsizeiptr,
                indices.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );

            gl::GenBuffers(1, &mut vertex_vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vertex_vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                vertices.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );
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
            indices,
            vao_id,
            vbo_ids: vec![vertex_vbo, indices_vbo],
        }
    }

    pub fn get_vao_id(&self) -> gl::types::GLuint {
        self.vao_id
    }

    pub fn get_vertex_count(&self) -> i32 {
        self.indices.len() as i32
    }
}

impl Drop for MeshData {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, vec![self.vao_id].as_ptr());
            gl::DeleteBuffers(self.vbo_ids.len() as i32, self.vbo_ids.as_ptr());
        }
    }
}
