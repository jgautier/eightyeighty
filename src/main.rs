mod emulator;
mod machine;
use std::fs;
fn main() {
    let result = fs::read("invaders");
    if let Ok(bytes) = result {
        let space_invaders_io = &mut machine::SpaceInvadersIO::new();
        let mut emu = emulator::Emulator::new(bytes);
        emu.run(space_invaders_io);
    } else {
        println!("Error reading file {:?}", result);
    }
}
