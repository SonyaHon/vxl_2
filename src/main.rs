extern crate cgmath;
extern crate gl;
extern crate sdl2;
extern crate specs;

mod component;
mod resource;
mod system;
mod utils;
mod vxl;

use component::{mesh_data::MeshData, renderer::Renderer};
use specs::prelude::*;
use vxl::VXL;
fn main() {
    let vxl = VXL::new();
    let mut window = vxl.create_window("VXL", 1280, 720);
    let mut input = vxl.create_input();
    let mut manager = resource::manager::Manager::new();

    manager.load_shader_program(
        vec![
            ("test/tv", gl::VERTEX_SHADER),
            ("test/tf", gl::FRAGMENT_SHADER),
        ],
        "tri",
    );

    window.set_clear_color((0.3, 0.3, 0.3));

    let mut world = World::new();
    world.register::<MeshData>();
    world.register::<Renderer>();
    let mut dispatcher = DispatcherBuilder::new()
        .with_thread_local(system::triangle::TriangleSys)
        .build();
    dispatcher.setup(&mut world);

    world
        .create_entity()
        .with(MeshData::from_vertices(vec![
            -0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0,
        ]))
        .with(Renderer::new())
        .build();

    world.insert(manager);

    'main: loop {
        input.update();
        if input.window_should_close() {
            break 'main;
        }
        window.clear();

        dispatcher.dispatch(&mut world);

        window.update();
        world.maintain();
    }
}
