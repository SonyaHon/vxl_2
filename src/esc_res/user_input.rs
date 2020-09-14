pub struct UserInput {
    pressed_keys: Vec<sdl2::keyboard::Scancode>,
    mouse_pressed_keys: Vec<sdl2::mouse::MouseButton>,
    mouse_position: cgmath::Vector2<i32>,
    mouse_diff: cgmath::Vector2<i32>,
}

impl UserInput {
    pub fn new(
        pressed_keys: Vec<sdl2::keyboard::Scancode>,
        mouse_pressed_keys: Vec<sdl2::mouse::MouseButton>,
        mouse_position: cgmath::Vector2<i32>,
        mouse_diff: cgmath::Vector2<i32>,
    ) -> UserInput {
        UserInput {
            pressed_keys,
            mouse_pressed_keys,
            mouse_position,
            mouse_diff,
        }
    }

    pub fn is_key_pressed(&self, key: sdl2::keyboard::Scancode) -> bool {
        for pressed_key in self.pressed_keys.as_slice() {
            if pressed_key == &key {
                return true;
            }
        }
        false
    }

    pub fn is_mouse_button_pressed(&self, button: sdl2::mouse::MouseButton) -> bool {
        for pressed_button in self.mouse_pressed_keys.as_slice() {
            if pressed_button == &button {
                return true;
            }
        }
        false
    }

    pub fn get_mouse_pos(&self) -> cgmath::Vector2<i32> {
        self.mouse_position
    }

    pub fn get_mouse_diff(&self) -> cgmath::Vector2<i32> {
        self.mouse_diff
    }
}
