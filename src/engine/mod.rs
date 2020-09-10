pub mod input_handler;

pub struct Engine {
    _sdl: sdl2::Sdl,
    _video_subsystem: sdl2::VideoSubsystem,
    window: sdl2::video::Window,
    _gl_context: sdl2::video::GLContext,
    clear_color: cgmath::Vector3<f32>,
    pub input_handler: input_handler::InputHandler,
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
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);
        let _gl_context = window.gl_create_context().unwrap();

        Ok(Engine {
            _sdl: sdl,
            _video_subsystem: video_subsystem,
            _gl_context,
            window,
            clear_color: cgmath::Vector3::new(0.0, 0.0, 0.0),
            input_handler: input_handler::InputHandler::create(event_pump),
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
            unsafe {
                gl::ClearColor(
                    self.clear_color.x,
                    self.clear_color.y,
                    self.clear_color.z,
                    1.0,
                );
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }

            game_loop();

            self.window.gl_swap_window();
        }
    }

    pub fn set_clear_color(&mut self, new_color: cgmath::Vector3<f32>) {
        self.clear_color = new_color;
    }
}

impl Drop for Engine {
    fn drop(&mut self) {}
}