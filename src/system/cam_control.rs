use crate::{
    component::camera::Camera, component::camera::MainCamera, component::transform::Transform,
    esc_res::user_input::UserInput,
};
use sdl2::keyboard::Scancode;
use specs::prelude::*;

pub struct CamControl;
impl<'a> System<'a> for CamControl {
    type SystemData = (
        ReadExpect<'a, UserInput>,
        ReadStorage<'a, MainCamera>,
        WriteStorage<'a, Camera>,
        WriteStorage<'a, Transform>,
    );

    fn run(&mut self, (user_input, main_camera, mut camera, mut transform): Self::SystemData) {
        let (_main_camera, _camera, transform) = (&main_camera, &mut camera, &mut transform)
            .join()
            .next()
            .unwrap();

        if user_input.is_key_pressed(Scancode::A) {
            transform.translate(cgmath::vec3(-0.01, 0.0, 0.0));
        } else if user_input.is_key_pressed(Scancode::D) {
            transform.translate(cgmath::vec3(0.01, 0.0, 0.0));
        }

        if user_input.is_key_pressed(Scancode::W) {
            transform.translate(cgmath::vec3(0.0, 0.0, -0.01));
        } else if user_input.is_key_pressed(Scancode::S) {
            transform.translate(cgmath::vec3(0.0, 0.0, 0.01));
        }

        println!("Camera pos {:?}", transform.get_translate());
    }
}
