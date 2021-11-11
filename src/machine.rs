use sdl2::keyboard::Keycode;
use sdl2::Sdl;
pub trait IO {
    fn input(&self, port: u8) -> u8;
    fn output(&mut self, port: u8, val: u8);
}

struct SpaceInvaders {
    io: SpaceInvadersIO,
    controller: dyn Controller
}

pub struct SpaceInvadersIO {
    // read ports
    port0: u8,
    port1: u8,
    port2: u8,
    
    shift_register: u16,
    shift_amount: u8
}

impl SpaceInvadersIO {
    pub fn new() -> Self {
        Self {
            shift_register: 0,
            shift_amount: 0,
            port0: 0b0111_0000,
            port1: 0b0001_0000,
            port2: 0b0000_0000
        }
    }
}

impl IO for SpaceInvadersIO {
    fn input(&self, port: u8) -> u8 {
        match port {
            1 => self.port1,
            2 => self.port2,
            3 => (self.shift_register >> (8 - self.shift_amount)) as u8,
            _ => panic!("unhandled input port {}", port)
        }
    }
    fn output(&mut self, port: u8, val: u8) {
        match port {
            2 => self.shift_amount = val & 0b111,
            4 => {
                let [_, val2] = u16::to_le_bytes(self.shift_register);
                self.shift_register = u16::from_le_bytes([val2, val])
            },
            3 | 5 | 6 => {},
            _ => panic!("cannot write to port {}", port)
        }
    }
}

pub enum Button {
    Shoot,
    Left,
    Right
}

pub enum Player {
    Player1,
    Player2
}

pub struct ButtonPress {
    button: Button,
    player: Player
}

/*pub struct Controller {
    button_presses: Vec<(Player, Button)>
}
*/

pub trait Controller {
    fn get_button_presses(&self) -> Vec<(Player, Button)>;
}

pub struct Sdl2KeyboardController {
    event_pump: sdl2::EventPump
}

impl Sdl2KeyboardController {
    pub fn new(sdl_context: Sdl) -> Self {
        Sdl2KeyboardController {
            event_pump: sdl_context.event_pump().unwrap()
        }
    }
}

impl Controller for Sdl2KeyboardController {
    fn get_button_presses(&self) -> Vec<(Player, Button)> {
        self.event_pump.keyboard_state().pressed_scancodes()
            .filter_map(|scan_code| {
                let key_code = Keycode::from_scancode(scan_code)?;
                match key_code {
                    Keycode::Left => {
                        Some((Player::Player1, Button::Left))
                    }
                    Keycode::Right => {
                        Some((Player::Player1, Button::Right))
                    }
                    Keycode::Space => {
                        Some((Player::Player1, Button::Shoot))
                    }
                    _ => {
                        None
                    }
                }
            })
            .collect()
    }
}


#[cfg(test)]
mod test {
    use crate::machine::SpaceInvadersIO;
    #[test]
    fn test() {
        println!("Hello World")
    }
}