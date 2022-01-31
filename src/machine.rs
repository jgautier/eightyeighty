use sdl2::keyboard::Keycode;
use sdl2::Sdl;
use sdl2::pixels;
use sdl2::rect::Point;
use crate::cpu::Cpu;
use std::time::{Instant, Duration};
use std::thread;
use sdl2::event::Event;
use std::convert::TryInto;

pub trait IO {
    fn input(&self, port: u8) -> u8;
    fn output(&mut self, port: u8, val: u8);
}

pub struct SpaceInvaders {
    io: SpaceInvadersIO,
    cpu: Cpu,
    screen: Screen,
    controller: Box<dyn Controller>
}

impl SpaceInvaders {
    pub fn new(bytes: Vec<u8>) -> Self {
        let sdl_context = sdl2::init().unwrap();
        SpaceInvaders {
            io: SpaceInvadersIO::new(),
            cpu: Cpu::new(bytes),
            screen: Screen::new(&sdl_context).unwrap(),
            controller: Box::new(Sdl2KeyboardController::new(sdl_context))
        }
    }

    pub fn play(mut self) {
        // 2mhz(2000000) / 1000 / 1000
        let mut total_cycles = 0;
        let frame_ms: u128 = 16;
        loop {
            let start = Instant::now();
            let mut steps = 0;
            let mut current_cycles = 0;
            while current_cycles < 33_000 / 2 {
                if let Ok(cycles) = self.cpu.execute_next_op(&mut self.io) {
                    current_cycles += cycles as u64;
                    steps += 1;
                } 
            }

            self.cpu.interrupt(1);

            current_cycles = 0;
            while current_cycles < 33_000 {
                if let Ok(cycles) = self.cpu.execute_next_op(&mut self.io) {
                    current_cycles += cycles as u64;
                    steps += 1;
                }
            }

            total_cycles += current_cycles;

            self.cpu.interrupt(2);

            for button in self.controller.get_button_states() {
                match button {
                    (Player::Player1, Button::Coin(ButtonState::Down)) => {
                        self.io.port1 |= 1 << 0x00;
                    }
                    (Player::Player1, Button::OnePlayer(ButtonState::Down)) => {
                        self.io.port1 |= 1 << 0x02;
                    }
                    (Player::Player1, Button::Left(ButtonState::Down)) => {
                        self.io.port1 |= 1 << 0x05;
                    }
                    (Player::Player1, Button::Right(ButtonState::Down)) => {
                        self.io.port1 |= 1 << 0x06;
                    }
                    (Player::Player1, Button::Shoot(ButtonState::Down)) => {
                        self.io.port1 |= 1 << 0x04;
                    }
                    (Player::Player1, Button::Left(ButtonState::Up)) => {
                        self.io.port1 &= !(1 << 0x05);
                    }
                    (Player::Player1, Button::Right(ButtonState::Up)) => {
                        self.io.port1 &= !(1 << 0x06);
                    }
                    (Player::Player1, Button::Shoot(ButtonState::Up)) => {
                        self.io.port1 &= !(1 << 0x04);
                    }
                    (Player::Player1, Button::Coin(ButtonState::Up)) => {
                        self.io.port1 &= !(1 << 0x00);
                    }
                    (Player::Player1, Button::OnePlayer(ButtonState::Up)) => {
                        self.io.port1 &= !(1 << 0x02);
                    }
                    _ => {
                        println!("Unhandled player/button combination")
                    }
                }
            }

            self.screen.clear();
            let framebuffer = &self.cpu.state.memory[0x2400..=0x3FFF];
            for y in 0..224 {
                let line = &framebuffer[(32 * y)..(32 * y + 32)];
                for (x, px) in line.iter().enumerate() {
                    for b in 0..8 {
                        if px & (1 << b) != 0 {
                            self.screen.draw(y as i16, 256 - (8 * x + b) as i16, 0).unwrap()
                        }
                    }
                }
            }
            self.screen.canvas.present();
            thread::sleep(Duration::from_millis((frame_ms - start.elapsed().as_millis()).try_into().unwrap()));
        }
    }
}

pub struct SpaceInvadersIO {
    // read ports
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
            port1: 0b0000_1000,
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
            2 => {
                self.shift_amount = val & 0b111;
            },
            4 => {
                let [_, val2] = u16::to_le_bytes(self.shift_register);
                self.shift_register = u16::from_le_bytes([val2, val]);
            },
            3 | 5 | 6 => {},
            _ => panic!("cannot write to port {}", port)
        }
    }
}

pub enum Button {
    Shoot(ButtonState),
    Left(ButtonState),
    Right(ButtonState),
    Coin(ButtonState),
    OnePlayer(ButtonState)
}

#[derive(Debug)]
pub enum ButtonState {
    Up, 
    Down
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
    fn get_button_states(&mut self) -> Vec<(Player, Button)>;
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
    fn get_button_states(&mut self) -> Vec<(Player, Button)> {
        self.event_pump.poll_iter().filter_map(|event| {
            match event {
                Event::Quit{..} => {
                    panic!("bye ");
                    None
                }
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    panic!("bye ");
                    None
                },
                Event::KeyDown { keycode: Some(Keycode::C), .. } => {
                    Some((Player::Player1, Button::Coin(ButtonState::Down)))
                }
                Event::KeyUp { keycode: Some(Keycode::C), .. } => {
                    Some((Player::Player1, Button::Coin(ButtonState::Up)))
                }
                Event::KeyDown { keycode: Some(Keycode::Num1), .. } => {
                    Some((Player::Player1, Button::OnePlayer(ButtonState::Down)))
                }
                Event::KeyUp { keycode: Some(Keycode::Num1), .. } => {
                    Some((Player::Player1, Button::OnePlayer(ButtonState::Up)))
                }
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    Some((Player::Player1, Button::Left(ButtonState::Down)))
                }
                Event::KeyUp { keycode: Some(Keycode::Left), .. } => {
                    Some((Player::Player1, Button::Left(ButtonState::Up)))
                }
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    Some((Player::Player1, Button::Right(ButtonState::Down)))
                }
                Event::KeyUp { keycode: Some(Keycode::Right), .. } => {
                    Some((Player::Player1, Button::Right(ButtonState::Up)))
                }
                Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                    Some((Player::Player1, Button::Shoot(ButtonState::Down)))
                }
                Event::KeyUp { keycode: Some(Keycode::Space), .. } => {
                    Some((Player::Player1, Button::Shoot(ButtonState::Up)))
                }
                _ => { None }
            }
        }).collect()
        /*self.event_pump.keyboard_state().scancodes()
            .filter_map(|(scan_code, pressed)| {
                let key_code = Keycode::from_scancode(scan_code)?;
                let button_state = if pressed {
                    ButtonState::Down
                } else {
                    ButtonState::Up
                };
                match key_code {
                    Keycode::Left => {
                        Some((Player::Player1, Button::Left(button_state)))
                    }
                    Keycode::Right => {
                        Some((Player::Player1, Button::Right(button_state)))
                    }
                    Keycode::Space => {
                        Some((Player::Player1, Button::Shoot(button_state)))
                    },
                    Keycode::C => {
                        
                    }
                    Keycode::Num1 => {
                        Some((Player::Player1, Button::OnePlayer(button_state)))
                    }
                    _ => {
                        None
                    }
                }
            })
            .collect()*/
    }
}

pub struct Screen {
    video: sdl2::VideoSubsystem,
    canvas: sdl2::render::WindowCanvas
}

impl Screen {
    pub fn new(sdl_context: &sdl2::Sdl) -> Result<Screen, String> {
        let video = sdl_context.video()?;
        let window = video.window("Space Invaders", 224, 256).position_centered().build().unwrap();

        let mut canvas = window.into_canvas().present_vsync().build().unwrap();

        canvas.set_draw_color(pixels::Color::RGB(0,0,0));
        canvas.clear();
        canvas.present();

        Ok(Screen {
            video,
            canvas
        })
    }

    pub fn clear(&mut self) {
        self.canvas.set_draw_color(pixels::Color::RGB(255,255,255));
        self.canvas.clear();
    }

    pub fn draw(&mut self, x: i16, y: i16, color: u8) -> Result<(), String> {
        let color = pixels::Color::RGB(color, color, color);
        self.canvas.set_draw_color(color);
        self.canvas.draw_point(Point::new(x.into(), y.into()))
    }
}