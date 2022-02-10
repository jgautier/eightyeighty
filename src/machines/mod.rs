pub mod spaceinvaders;
pub trait IO {
    fn input(&self, port: u8) -> u8;
    fn output(&mut self, port: u8, val: u8);
}