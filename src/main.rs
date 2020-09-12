extern crate cgmath;
extern crate gl;
extern crate sdl2;

mod engine;
mod game;

const TEST_TRI_SHADER: &str = "test_triangle";

fn main() {
    fn prepare(engine: &mut engine::Engine) {
        engine.set_clear_color(cgmath::Vector3::new(0.2, 0.2, 0.2));

        engine.get_resource_manager().load_shader_program(
            vec![
                ("test\\triangle_vertex", gl::VERTEX_SHADER),
                ("test\\triangle_fragment", gl::FRAGMENT_SHADER),
            ],
            TEST_TRI_SHADER,
        );
    }
    let _engine = engine::Engine::craft(("VXL", 1280, 720))
        .unwrap()
        .prepare(prepare)
        .game_loop();
}
