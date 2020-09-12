pub mod graphics;
pub mod input_handler;
pub mod resource;
pub mod utils;

pub struct Engine {
    _sdl: sdl2::Sdl,
    _video_subsystem: sdl2::VideoSubsystem,
    window: sdl2::video::Window,
    _gl_context: sdl2::video::GLContext,
    clear_color: cgmath::Vector3<f32>,
    resouce_manager: resource::manager::Manager,
    input_handler: input_handler::InputHandler,
}

impl Engine {
    pub fn craft(window_settings: (&str, u32, u32)) -> Result<Engine, String> {
        let sdl = sdl2::init().unwrap();
        let video_subsystem = sdl.video().unwrap();

        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(3, 3);

        let window = video_subsystem
            .window(window_settings.0, window_settings.1, window_settings.2)
            .opengl()
            .position_centered()
            .build()
            .unwrap();

        let event_pump = sdl.event_pump().unwrap();
        let _gl_context = window.gl_create_context().unwrap();
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

        unsafe { gl::Viewport(0, 0, window_settings.1 as i32, window_settings.2 as i32) }

        Ok(Engine {
            _sdl: sdl,
            _video_subsystem: video_subsystem,
            _gl_context,
            window,
            clear_color: cgmath::Vector3::new(0.0, 0.0, 0.0),
            resouce_manager: resource::manager::Manager::new(),
            input_handler: input_handler::InputHandler::create(event_pump),
        })
    }

    pub fn prepare(&mut self, prepare_func: fn(engine: &mut Engine)) -> &mut Engine {
        prepare_func(self);
        self
    }

    pub fn game_loop(&mut self) {
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

            // todo add esc system here

            self.window.gl_swap_window();
        }
    }

    pub fn set_clear_color(&mut self, new_color: cgmath::Vector3<f32>) {
        self.clear_color = new_color;
    }
}

impl Engine {
    pub fn get_input_handler(&mut self) -> &mut input_handler::InputHandler {
        &mut self.input_handler
    }

    pub fn get_resource_manager(&mut self) -> &mut resource::manager::Manager {
        &mut self.resouce_manager
    }
}

impl Drop for Engine {
    fn drop(&mut self) {}
}
