use super::Engine;

pub trait Entity {
    fn get_id(&self) -> u64;

    fn update(&mut self, engine: &mut Engine);

    fn prepare(&mut self, engine: Engine) {}
}
