pub struct InputHandler {
    event_pump: sdl2::EventPump,
    should_window_close: bool,
    callbacks: std::collections::HashMap<sdl2::keyboard::Keycode, Vec<fn()>>,
}

impl InputHandler {
    pub fn create(event_pump: sdl2::EventPump) -> InputHandler {
        InputHandler {
            event_pump,
            should_window_close: false,
            callbacks: std::collections::HashMap::new(),
        }
    }

    pub fn game_loop(&mut self) {
        for event in self.event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => self.should_window_close = true,
                sdl2::event::Event::KeyDown { keycode, .. } => {
                    let handlers = self.callbacks.get(&keycode.unwrap());
                    if handlers.is_none() {
                        return;
                    }
                    for handler in handlers.unwrap() {
                        handler();
                    }
                }
                _ => {}
            }
        }
    }

    pub fn should_window_close(&self) -> bool {
        self.should_window_close
    }

    pub fn on(&mut self, key: sdl2::keyboard::Keycode, handler: fn()) {
        let entry = self.callbacks.entry(key).or_insert(vec![handler]);
        entry.push(handler);
    }
}
