pub mod input_handler;

pub struct Engine {
    _sdl: sdl2::Sdl,
    _video_subsystem: sdl2::VideoSubsystem,
    _window: sdl2::video::Window,
    pub input_handler: input_handler::InputHandler
}

impl Engine {
    pub fn craft(window_settings: (&str, u32, u32)) -> Result<Engine, String> {
        let sdl = sdl2::init().unwrap();
        let video_subsystem = sdl.video().unwrap();
        let window = video_subsystem
            .window(window_settings.0, window_settings.1, window_settings.2)
            .opengl()
            .build()
            .unwrap();

        let event_pump = sdl.event_pump().unwrap();
        
        Ok(Engine {
            _sdl: sdl,
            _video_subsystem: video_subsystem,
            _window: window,
            input_handler: input_handler::InputHandler::create(event_pump)
        })
    }

    pub fn prepare(&mut self, prepare_func: fn(engine: &mut Engine)) -> &mut Engine {
        prepare_func(self);
        self
    }

    pub fn game_loop(&mut self, game_loop: fn()) {
        'main: loop {
            self.input_handler.game_loop();

            if self.input_handler.should_window_close() {
                break 'main;
            }

            game_loop();
        }
    }
}


impl Drop for Engine {
    fn drop(&mut self) {}
}
