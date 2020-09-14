extern crate cgmath;
extern crate gl;
extern crate sdl2;
extern crate specs;

mod component;
mod resource;
mod system;
mod utils;
mod vxl;

use specs::prelude::*;
use vxl::VXL;

fn main() {
    let vxl = VXL::new();

    let mut window = vxl.create_window("VXL", 1280, 720);
    let mut input = vxl.create_input();

    window.set_clear_color((0.3, 0.3, 0.3));

    let mut world = World::new();
    let mut dispatcher = DispatcherBuilder::new().build();

    dispatcher.setup(&mut world);

    'main: loop {
        input.update();
        if input.window_should_close() {
            break 'main;
        }

        window.clear();

        dispatcher.dispatch(&mut world);
        world.maintain();

        window.update();
    }
}
