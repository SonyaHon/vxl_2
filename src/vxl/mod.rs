pub mod input;
pub mod window;

use input::VXLInput;
use window::VXLWindow;
pub struct VXL {
    sdl: sdl2::Sdl,
    video_subsystem: sdl2::VideoSubsystem,
}

impl VXL {
    pub fn new() -> VXL {
        let sdl = sdl2::init().unwrap();
        let video_subsystem = sdl.video().unwrap();

        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(3, 3);

        VXL {
            sdl,
            video_subsystem,
        }
    }

    pub fn create_window(&self, name: &str, width: u32, height: u32) -> VXLWindow {
        let sdl_window = self
            .video_subsystem
            .window(name, width, height)
            .opengl()
            .position_centered()
            .build()
            .unwrap();

        let _gl = sdl_window.gl_create_context().unwrap();
        gl::load_with(|s| {
            self.video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void
        });

        self.set_viewport(0, 0, width as i32, height as i32);

        VXLWindow::new(sdl_window, _gl)
    }

    pub fn create_input(&self) -> VXLInput {
        let event_pump = self.sdl.event_pump().unwrap();
        VXLInput::new(event_pump)
    }

    pub fn set_viewport(&self, x: i32, y: i32, width: i32, height: i32) {
        unsafe {
            gl::Viewport(x, y, width, height);
        }
    }
}
