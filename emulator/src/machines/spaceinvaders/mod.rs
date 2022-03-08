#[cfg(not(target_family="wasm"))]
pub mod sdl;
#[cfg(target_family="wasm")]
pub mod web;
use crate::machines::{Button, ButtonState, Player};
use crate::machines::{Machine, IO, Screen, Controller, Speaker};
use std::cell::RefCell;
use crate::cpu::Cpu;


const GREEN: (u8, u8, u8) = (0, 255, 0);
const WHITE: (u8, u8, u8) = (255, 255, 255);
const RED: (u8, u8, u8) = (255, 0, 0);

pub struct SpaceInvadersIO {
    // read ports
    pub port1: u8,
    pub port2: u8,
    
    shift_register: u16,
    shift_amount: u8,
    prev_port3_val: u8,
    prev_port5_val: u8,
    speaker: Box<dyn Speaker>
}

impl SpaceInvadersIO {
    pub fn new(speaker: Box<dyn Speaker>) -> Self {
        Self {
            shift_register: 0,
            shift_amount: 0,
            port1: 0b0000_1000,
            port2: 0b0000_0000,
            prev_port3_val: 0,
            prev_port5_val: 0,
            speaker: speaker
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
                        self.speaker.start_wav_file("ufo.wav");
                    } else if val & 0x1 == 0 && self.prev_port3_val & 0x1 == 1 {
                        self.speaker.stop_wav_file("ufo.wav");
                    }
                    if val & 0x2 == 2 && self.prev_port3_val & 0x2 == 0 {
                        self.speaker.play_wav_file("shoot.wav");
                    }
                    if val & 0x4 == 4 && self.prev_port3_val & 0x4 == 0 {
                        self.speaker.play_wav_file("player_dies.wav");
                    }
                    if val & 0x8 == 8 && self.prev_port3_val & 0x8 == 0 {
                        self.speaker.play_wav_file("invader_dies.wav");
                    }
                    self.prev_port3_val = val;
                }
            },
            5 => {
                if val & 0x1 == 1 && self.prev_port5_val & 0x1 == 0 {
                    self.speaker.play_wav_file("bomp.wav")
                }
                self.prev_port5_val = val;
            },
            6 => {},
            _ => panic!("cannot write to port {}", port)
        }
    }
}


pub struct SpaceInvaders {
    io: RefCell<SpaceInvadersIO>,
    cpu: Cpu,
    screen: Box<dyn Screen>,
    controller: Box<dyn Controller>
}

impl Machine for SpaceInvaders {
    fn run_next_frame(&mut self) {
        let mut current_cycles = 0;
        while current_cycles < 33_000 / 2 {
            if let Ok(cycles) = self.cpu.execute_next_op(&self.io) {
                current_cycles += cycles as u64;
            } 
        }

        self.cpu.interrupt(1);

        current_cycles = 0;
        while current_cycles < 33_000 {
            if let Ok(cycles) = self.cpu.execute_next_op(&self.io) {
                current_cycles += cycles as u64;
            }
        }

        self.cpu.interrupt(2);

        for button in self.controller.get_button_states() {
            match button {
                (Player::Player1, Button::Coin(ButtonState::Down)) => {
                    self.io.borrow_mut().port1 |= 1 << 0x00;
                }
                (Player::Player1, Button::OnePlayer(ButtonState::Down)) => {
                    self.io.borrow_mut().port1 |= 1 << 0x02;
                }
                (Player::Player1, Button::Left(ButtonState::Down)) => {
                    self.io.borrow_mut().port1 |= 1 << 0x05;
                }
                (Player::Player1, Button::Right(ButtonState::Down)) => {
                    self.io.borrow_mut().port1 |= 1 << 0x06;
                }
                (Player::Player1, Button::Shoot(ButtonState::Down)) => {
                    self.io.borrow_mut().port1 |= 1 << 0x04;
                }
                (Player::Player1, Button::Left(ButtonState::Up)) => {
                    self.io.borrow_mut().port1 &= !(1 << 0x05);
                }
                (Player::Player1, Button::Right(ButtonState::Up)) => {
                    self.io.borrow_mut().port1 &= !(1 << 0x06);
                }
                (Player::Player1, Button::Shoot(ButtonState::Up)) => {
                    self.io.borrow_mut().port1 &= !(1 << 0x04);
                }
                (Player::Player1, Button::Coin(ButtonState::Up)) => {
                    self.io.borrow_mut().port1 &= !(1 << 0x00);
                }
                (Player::Player1, Button::OnePlayer(ButtonState::Up)) => {
                    self.io.borrow_mut().port1 &= !(1 << 0x02);
                }
                _ => {
                    println!("Unhandled player/button combination")
                }
            }
        }

        self.screen.clear();
        let framebuffer = &self.cpu.state.memory[0x2400..=0x3FFF];
        for x in 0..224 {
            let line = &framebuffer[(32 * x)..(32 * x + 32)];
            for (i, px) in line.iter().enumerate() {
                for b in 0..8 {
                    if px & (1 << b) != 0 {
                        let y = 256 - (8 * i + b) as i32;
                        let color = if y > 180 {
                            GREEN
                        } else if y > 33 && y < 50 {
                            RED
                        } else {
                            WHITE
                        };
                        self.screen.draw(x as i32, y, color);
                    }
                }
            }
        }
        self.screen.present();
    }
}