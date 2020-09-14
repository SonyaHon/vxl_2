use specs::prelude::*;

pub struct Renderer {}

impl Component for Renderer {
    type Storage = DenseVecStorage<Self>;
}

impl Renderer {
    pub fn new() -> Renderer {
        Renderer {}
    }
}
