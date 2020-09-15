extern crate cgmath;
extern crate gl;
extern crate image;
extern crate sdl2;
extern crate specs;

mod component;
mod esc_res;
mod resource;
mod system;
mod utils;
mod vxl;

use component::{
    camera::Camera, camera::MainCamera, mesh_data::MeshData, renderer::Renderer,
    transform::Transform,
};
use specs::prelude::*;
use vxl::VXL;

fn main() {
    let vxl = VXL::new();
    let mut window = vxl.create_window("VXL", 1280, 720);
    let mut input = vxl.create_input();
    let mut manager = resource::manager::Manager::new();
    let mut clock = vxl.create_clock();

    // SHADER LOADING
    manager.load_shader_program(
        vec![
            ("test/tv", gl::VERTEX_SHADER),
            ("test/tf", gl::FRAGMENT_SHADER),
        ],
        "tri",
    );
    // SHADER LOADING END

    //TEXTURE LOADING
    manager.load_texture("colors", "colors");
    //TEXTURE LOADING END

    window.set_clear_color((0.3, 0.3, 0.3));

    let mut world = World::new();
    world.register::<MeshData>();
    world.register::<Renderer>();
    world.register::<Transform>();
    world.register::<Camera>();
    world.register::<MainCamera>();

    let mut dispatcher = DispatcherBuilder::new()
        .with(system::cam_control::CamControl, "camera_control", &[])
        .with_thread_local(system::triangle::TriangleSys)
        .build();
    dispatcher.setup(&mut world);

    world
        .create_entity()
        .with(MeshData::from_data(
            vec![-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0],
            vec![0, 2, 1],
            vec![0.0, 0.0, 0.5, 1.0, 1.0, 0.0],
        ))
        .with(Renderer::new())
        .with(Transform::from_data(
            cgmath::vec3(-1.0, 0.0, 0.0),
            cgmath::vec3(0.0, 0.0, 0.0),
            1.0,
        ))
        .build();

    world
        .create_entity()
        .with(Transform::from_data(
            cgmath::vec3(0.0, 0.0, 5.0),
            cgmath::vec3(0.0, 0.0, 0.0),
            1.0,
        ))
        .with(MainCamera {})
        .with(Camera::new(1.0, 720.0 / 1280.0, 0.001, 1000.0))
        .build();

    world.insert(manager);

    'main: loop {
        input.update();
        if input.window_should_close() {
            break 'main;
        }
        world.insert(input.get_user_input_res());
        world.insert(clock.get_delta());
        window.clear();

        dispatcher.dispatch(&mut world);

        window.update();
        world.maintain();

        // unsafe {
        //     println!("GL Error: {}", gl::GetError());
        // }
    }
}
