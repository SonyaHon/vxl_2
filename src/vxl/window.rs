use sdl2::video::GLContext;

pub struct VXLWindow {
    sdl_window: sdl2::video::Window,
    clear_color: (f32, f32, f32),
    _gl: GLContext,
}

impl VXLWindow {
    pub fn new(window: sdl2::video::Window, gl: GLContext) -> VXLWindow {
        VXLWindow {
            sdl_window: window,
            clear_color: (0.2, 0.2, 0.2),
            _gl: gl,
        }
    }

    pub fn set_clear_color(&mut self, new_clear_color: (f32, f32, f32)) {
        self.clear_color = new_clear_color;
    }

    pub fn clear(&self) {
        unsafe {
            gl::ClearColor(
                self.clear_color.0,
                self.clear_color.1,
                self.clear_color.2,
                1.0,
            );
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    pub fn update(&self) {
        self.sdl_window.gl_swap_window();
    }
}
