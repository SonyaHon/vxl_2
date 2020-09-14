use specs::prelude::*;

pub struct MeshData {
    vertices: Vec<f64>,
    indices: Vec<i32>,
    vao_id: gl::types::GLuint,
    indices_vbo_id: gl::types::GLuint,
    vbo_ids: Vec<gl::types::GLuint>,
}

impl Component for MeshData {
    type Storage = DenseVecStorage<Self>;
}

impl MeshData {
    pub fn from_data(vertices: Vec<f64>, indices: Vec<i32>) -> MeshData {
        let mut vertex_vbo: gl::types::GLuint = 0;
        let mut indices_vbo: gl::types::GLuint = 0;
        let mut vao_id: gl::types::GLuint = 0;
        unsafe {
            // generate and bind vao
            gl::GenVertexArrays(1, &mut vao_id);
            gl::BindVertexArray(vao_id);

            // generate and bind indices vbo
            gl::GenBuffers(1, &mut indices_vbo);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, indices_vbo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * std::mem::size_of::<i32>()) as gl::types::GLsizeiptr,
                indices.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );

            // generate and bind vertex data vbo
            gl::EnableVertexAttribArray(0);
            gl::GenBuffers(1, &mut vertex_vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vertex_vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                vertices.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );

            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                (3 * std::mem::size_of::<f64>()) as gl::types::GLint,
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
            indices_vbo_id: indices_vbo,
        }
    }

    pub fn get_vao_id(&self) -> gl::types::GLuint {
        self.vao_id
    }

    pub fn get_vertex_count(&self) -> i32 {
        self.indices.len() as i32
    }

    pub fn get_indices_vbo(&self) -> gl::types::GLuint {
        self.indices_vbo_id
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
