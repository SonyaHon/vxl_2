use specs::prelude::*;
pub struct InputState {
    direction: u8
}
impl Component for InputState {
    type Storage = HashMapStorage<Self>;
}
