use specs::prelude::*;

use crate::component::{mesh_data::MeshData, renderer::Renderer};
use crate::resource::manager::Manager;

pub struct TriangleSys;
impl<'a> System<'a> for TriangleSys {
    type SystemData = (ReadExpect<'a, Manager>, ReadStorage<'a, MeshData>, ReadStorage<'a, Renderer>);

    fn run(&mut self, (manager, mesh_data, renderer): Self::SystemData) {
        for (mesh_data, _renderer) in (&mesh_data, &renderer).join() {
            unsafe {
                manager.get_shader_program("tri").bind();
                gl::BindVertexArray(mesh_data.get_vao_id());
                gl::DrawArrays(gl::TRIANGLES, 0, mesh_data.get_vertex_count());
                gl::BindVertexArray(0);
                manager.unbind_shader_program();
            }
        }
    }
}
