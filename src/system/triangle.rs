use specs::prelude::*;

use crate::component::{
    camera::{Camera, MainCamera},
    mesh_data::MeshData,
    renderer::Renderer,
    transform::Transform,
};
use crate::resource::manager::Manager;

pub struct TriangleSys;
impl<'a> System<'a> for TriangleSys {
    type SystemData = (
        ReadExpect<'a, Manager>,
        ReadStorage<'a, MeshData>,
        ReadStorage<'a, Renderer>,
        ReadStorage<'a, Transform>,
        ReadStorage<'a, Camera>,
        ReadStorage<'a, MainCamera>,
    );

    fn run(
        &mut self,
        (manager, mesh_data, renderer, transform, camera, main_camera): Self::SystemData,
    ) {
        let (main_cam, main_cam_transform, _) =
            (&camera, &transform, &main_camera).join().next().unwrap();
        let projection_matrix = main_cam.get_projection_matrix();
        let view_matrix = main_cam_transform.get_view_matrix();

        for (mesh_data, _renderer, transform) in (&mesh_data, &renderer, &transform).join() {
            unsafe {
                let shader_prog = manager.get_shader_program("tri");
                shader_prog.bind();
                shader_prog
                    .add_unifrom_matrix4("transform_mat".to_string(), transform.get_matrix());
                shader_prog.add_unifrom_matrix4("projection_mat".to_string(), projection_matrix);
                shader_prog.add_unifrom_matrix4("view_mat".to_string(), view_matrix);

                gl::BindVertexArray(mesh_data.get_vao_id());
                gl::EnableVertexAttribArray(0);
                gl::EnableVertexAttribArray(1);
                manager.get_texture("colors").bind();
                gl::DrawElements(
                    gl::TRIANGLES,
                    mesh_data.get_vertex_count() as gl::types::GLsizei,
                    gl::UNSIGNED_INT,
                    std::ptr::null(),
                );
                gl::DisableVertexAttribArray(0);
                gl::DisableVertexAttribArray(1);
                gl::BindVertexArray(0);

                manager.unbind_texture();
                manager.unbind_shader_program();
            }
        }
    }
}
