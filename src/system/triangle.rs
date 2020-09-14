use specs::prelude::*;

use crate::component::{mesh_data::MeshData, renderer::Renderer};
use crate::esc_res::delta_time::DeltaTime;
use crate::esc_res::user_input::UserInput;
use crate::resource::manager::Manager;

pub struct TriangleSys;
impl<'a> System<'a> for TriangleSys {
    type SystemData = (
        ReadExpect<'a, Manager>,
        ReadExpect<'a, UserInput>,
        ReadExpect<'a, DeltaTime>,
        ReadStorage<'a, MeshData>,
        ReadStorage<'a, Renderer>,
    );

    fn run(&mut self, (manager, user_input, delta_time, mesh_data, renderer): Self::SystemData) {
        for (mesh_data, _renderer) in (&mesh_data, &renderer).join() {
            unsafe {
                manager.get_shader_program("tri").bind();
                gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, mesh_data.get_indices_vbo());
                gl::BindVertexArray(mesh_data.get_vao_id());
                // gl::DrawArrays(gl::TRIANGLES, 0, mesh_data.get_vertex_count());
                gl::DrawElements(
                    gl::TRIANGLES,
                    mesh_data.get_vertex_count(),
                    gl::UNSIGNED_INT,
                    std::ptr::null() as *const gl::types::GLvoid,
                );
                println!("GL ERROR: {}", gl::GetError());
                gl::BindVertexArray(0);
                manager.unbind_shader_program();
            }
        }
    }
}
