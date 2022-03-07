#[cfg(not(target_family="wasm"))]
pub mod sdl;
#[cfg(target_family="wasm")]
pub mod web;
use crate::machines::{Button, ButtonState, Player};
use crate::machines::SpaceInvaders;
use crate::machines::Machine;

const GREEN: (u8, u8, u8) = (0, 255, 0);
const WHITE: (u8, u8, u8) = (255, 255, 255);
const RED: (u8, u8, u8) = (255, 0, 0);

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