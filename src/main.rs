extern crate gl;
extern crate sdl2;

mod engine;
mod game;

fn a_handler() {
    println!("Hello world");
}

fn prepare(engine: &mut engine::Engine) {
    engine.input_handler.on(sdl2::keyboard::Scancode::A, a_handler);
}

fn game_loop() {}

fn main() {
    let _engine = engine::Engine::craft(("VXL", 1280, 720))
        .unwrap()
        .prepare(prepare)
        .game_loop(game_loop);
}
