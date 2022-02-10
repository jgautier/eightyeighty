mod cpu;
mod machines;
use std::fs;
fn main() {
    let result = fs::read("invaders");
    if let Ok(bytes) = result {
        let space_invaders = machines::spaceinvaders::SpaceInvaders::new(bytes);
        space_invaders.play();
    } else {
        println!("Error reading file {:?}", result);
    }
}
 