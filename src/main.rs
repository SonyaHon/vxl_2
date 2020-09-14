extern crate cgmath;
extern crate gl;
extern crate sdl2;
extern crate specs;

mod component;
mod esc_res;
mod resource;
mod system;
mod utils;
mod vxl;

use component::{mesh_data::MeshData, renderer::Renderer};
use specs::prelude::*;
use vxl::VXL;

// fn gl_debug_callback(
//     source: gl::types::GLenum,
//     e_type: gl::types::GLenum,
//     id: gl::types::GLuint,
//     severity: gl::types::GLenum,
//     len: gl::types::GLsizei,
//     message: *const i8,
//     user_param: *mut std::ffi::c_void
// ) {
//     println!("GL ERROR {}", message.as_ref().uwrap() as &str);
// }


fn main() {
    let vxl = VXL::new();
    let mut window = vxl.create_window("VXL", 1280, 720);
    let mut input = vxl.create_input();
    let mut manager = resource::manager::Manager::new();
    let mut clock = vxl.create_clock();

    // unsafe {
    //     gl::Enable(gl::DEBUG_OUTPUT);
    //     gl::DebugMessageCallback(gl_debug_callback as gl::types::GLDEBUGPROC, std::ptr::null());
    // }

    // SHADER LOADING
    manager.load_shader_program(
        vec![
            ("test/tv", gl::VERTEX_SHADER),
            ("test/tf", gl::FRAGMENT_SHADER),
        ],
        "tri",
    );
    // SHADER LOADING END

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
        .with(MeshData::from_data(
            vec![-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0],
            vec![0, 2, 1],
        ))
        .with(Renderer::new())
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
    }
}
