pub struct VXLInput {
    sdl_event_pump: sdl2::EventPump,
    window_should_close: bool,
}

impl VXLInput {
    pub fn new(sdl_event_pump: sdl2::EventPump) -> VXLInput {
        VXLInput {
            sdl_event_pump,
            window_should_close: false,
        }
    }

    pub fn update(&mut self) {
        for event in self.sdl_event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => self.window_should_close = true,
                _ => {}
            }
        }
    }

    pub fn window_should_close(&self) -> bool {
        self.window_should_close
    }
}
