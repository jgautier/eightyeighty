mod emulator;
use std::fs;
fn main() {
    let result = fs::read("invaders");
    if let Ok(bytes) = result {
        let mut emu = emulator::Emulator::new(bytes);
        emu.run();
    } else {
        println!("Error reading file {:?}", result);
    }
}
