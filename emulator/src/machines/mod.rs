pub mod spaceinvaders;
use std::cell::RefCell;
use crate::cpu::Cpu;

pub trait IO {
    fn input(&self, port: u8) -> u8;
    fn output(&mut self, port: u8, val: u8);
}

pub trait Speaker {
    fn start_wav_file(&mut self, file_name: &str);
    fn stop_wav_file(&mut self, file_name: &str);
    fn play_wav_file(&mut self, file_name: &str);
}

pub trait Screen {
    fn clear(&mut self);
    fn draw(&mut self, x: i32, y: i32, color: (u8, u8, u8));
    fn present(&mut self);
}

pub trait Machine {
    fn run_next_frame(&mut self);
}

pub struct SpaceInvaders {
    io: RefCell<SpaceInvadersIO>,
    cpu: Cpu,
    screen: Box<dyn Screen>,
    controller: Box<dyn Controller>
}

pub struct SpaceInvadersIO {
    // read ports
    pub port1: u8,
    pub port2: u8,
    
    shift_register: u16,
    shift_amount: u8,
    prev_port3_val: u8,
    prev_port5_val: u8
    //speaker: Box<dyn Speaker>
}

impl SpaceInvadersIO {
    pub fn new() -> Self {
        Self {
            shift_register: 0,
            shift_amount: 0,
            port1: 0b0000_1000,
            port2: 0b0000_0000,
            prev_port3_val: 0,
            prev_port5_val: 0,
            //speaker: speaker
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
            2 => {
                self.shift_amount = val & 0b111;
            },
            4 => {
                let [_, val2] = u16::to_le_bytes(self.shift_register);
                self.shift_register = u16::from_le_bytes([val2, val]);
            },
            3 => {
                if val != self.prev_port3_val {
                    if val & 0x1 == 1 && self.prev_port3_val & 0x1 == 0 {
                        //self.speaker.start_wav_file("../resources/spaceinvaders/ufo.wav");
                    } else if val & 0x1 == 0 && self.prev_port3_val & 0x1 == 1 {
                        //self.speaker.stop_wav_file("../resources/spaceinvaders/ufo.wav");
                    }
                    if val & 0x2 == 2 && self.prev_port3_val & 0x2 == 0 {
                        //self.speaker.play_wav_file("../resources/spaceinvaders/shoot.wav");
                    }
                    if val & 0x4 == 4 && self.prev_port3_val & 0x4 == 0 {
                        //self.speaker.play_wav_file("../resources/spaceinvaders/player_dies.wav");
                    }
                    if val & 0x8 == 8 && self.prev_port3_val & 0x8 == 0 {
                        //self.speaker.play_wav_file("../resources/spaceinvaders/invader_dies.wav");
                    }
                    self.prev_port3_val = val;
                }
            },
            5 => {
                if val & 0x1 == 1 && self.prev_port5_val & 0x1 == 0 {
                    //self.speaker.play_wav_file("../resources/spaceinvaders/bomp.wav")
                }
                self.prev_port5_val = val;
            },
            6 => {},
            _ => panic!("cannot write to port {}", port)
        }
    }
}

#[derive(Clone, Debug)]
pub enum Button {
    Shoot(ButtonState),
    Left(ButtonState),
    Right(ButtonState),
    Coin(ButtonState),
    OnePlayer(ButtonState)
}

#[derive(Clone, Debug)]
pub enum ButtonState {
    Up, 
    Down
}

#[derive(Clone, Debug)]
pub enum Player {
    Player1,
    Player2
}

#[derive(Clone, Debug)]
pub struct ButtonPress {
    button: Button,
    player: Player
}

pub trait Controller {
    fn get_button_states(&mut self) -> Vec<(Player, Button)>;
}