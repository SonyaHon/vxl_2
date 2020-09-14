use crate::esc_res::user_input::UserInput;

pub struct VXLInput {
    sdl_event_pump: sdl2::EventPump,
    window_should_close: bool,
    mouse_last_position: cgmath::Vector2<i32>,
    mouse_current_position: cgmath::Vector2<i32>,
}

impl VXLInput {
    pub fn new(sdl_event_pump: sdl2::EventPump) -> VXLInput {
        VXLInput {
            sdl_event_pump,
            window_should_close: false,
            mouse_last_position: cgmath::vec2(0, 0),
            mouse_current_position: cgmath::vec2(0, 0),
        }
    }

    pub fn update(&mut self) {
        for event in self.sdl_event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => self.window_should_close = true,
                _ => {}
            }
        }

        let mouse_state = self.sdl_event_pump.mouse_state();
        self.mouse_last_position = self.mouse_current_position.clone();
        self.mouse_current_position = cgmath::vec2(mouse_state.x(), mouse_state.y());
    }

    pub fn window_should_close(&self) -> bool {
        self.window_should_close
    }

    pub fn get_user_input_res(&self) -> UserInput {
        let keyboard_state = self.sdl_event_pump.keyboard_state();
        let mouse_state = self.sdl_event_pump.mouse_state();
        UserInput::new(
            keyboard_state.pressed_scancodes().collect(),
            mouse_state.pressed_mouse_buttons().collect(),
            self.mouse_current_position.clone(),
            self.mouse_current_position - self.mouse_last_position,
        )
    }
}
