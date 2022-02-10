use sdl2::keyboard::Keycode;
use sdl2::Sdl;
use sdl2::pixels;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use crate::cpu::Cpu;
use crate::machines::IO;
use std::time::{Instant, Duration};
use std::thread;
use sdl2::event::Event;
use std::convert::TryInto;
use sdl2::mixer::{Music};
use std::collections::HashMap;

const GREEN: Color = Color::RGB(0, 255, 0);
const WHITE: Color = Color::RGB(255, 255, 255);
const RED: Color = Color::RGB(255, 0, 0);



pub struct SpaceInvaders {
    io: SpaceInvadersIO,
    cpu: Cpu,
    screen: Screen,
    controller: Box<dyn Controller>
}

impl SpaceInvaders {
    pub fn new(bytes: Vec<u8>) -> Self {
        let sdl_context = sdl2::init().unwrap();
        let speaker = SpaceInvadersSpeaker::new(&sdl_context);
        SpaceInvaders {
            io: SpaceInvadersIO::new(Box::new(speaker)),
            cpu: Cpu::new(bytes),
            screen: Screen::new(&sdl_context).unwrap(),
            controller: Box::new(Sdl2KeyboardController::new(sdl_context))
        }
    }

    pub fn play(mut self) {
        let frame_ms: u128 = 16;
        loop {
            let start = Instant::now();
            let mut current_cycles = 0;
            while current_cycles < 33_000 / 2 {
                if let Ok(cycles) = self.cpu.execute_next_op(&mut self.io) {
                    current_cycles += cycles as u64;
                } 
            }

            self.cpu.interrupt(1);

            current_cycles = 0;
            while current_cycles < 33_000 {
                if let Ok(cycles) = self.cpu.execute_next_op(&mut self.io) {
                    current_cycles += cycles as u64;
                }
            }

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
                            self.screen.draw(x as i32, y, color).unwrap();
                        }
                    }
                }
            }
            self.screen.canvas.present();
            let elapsed = start.elapsed().as_millis();
            if elapsed < frame_ms {
                thread::sleep(Duration::from_millis((frame_ms - elapsed).try_into().unwrap()));
            }
        }
    }
}

pub enum Sound {
    PlayerShoot
}

pub struct SpaceInvadersIO {
    // read ports
    port1: u8,
    port2: u8,
    
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
                        self.speaker.start_wav_file("resources/spaceinvaders/ufo.wav");
                    } else if val & 0x1 == 0 && self.prev_port3_val & 0x1 == 1 {
                        self.speaker.stop_wav_file("resources/spaceinvaders/ufo.wav");
                    }
                    if val & 0x2 == 2 && self.prev_port3_val & 0x2 == 0 {
                        self.speaker.play_wav_file("resources/spaceinvaders/shoot.wav");
                    }
                    if val & 0x4 == 4 && self.prev_port3_val & 0x4 == 0 {
                        self.speaker.play_wav_file("resources/spaceinvaders/player_dies.wav");
                    }
                    if val & 0x8 == 8 && self.prev_port3_val & 0x8 == 0 {
                        self.speaker.play_wav_file("resources/spaceinvaders/invader_dies.wav");
                    }
                    self.prev_port3_val = val;
                }
            },
            5 => {
                if val & 0x1 == 1 && self.prev_port5_val & 0x1 == 0 {
                    self.speaker.play_wav_file("resources/spaceinvaders/bomp.wav")
                }
                self.prev_port5_val = val;
            },
            6 => {},
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
                    panic!();
                    None
                }
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
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
    }
}

pub struct Screen {
    canvas: sdl2::render::WindowCanvas
}

impl Screen {
    pub fn new(sdl_context: &sdl2::Sdl) -> Result<Screen, String> {
        let video = sdl_context.video()?;
        let window = video.window("Space Invaders", 224 * 4, 256 * 4).position_centered().build().unwrap();

        let mut canvas = window.into_canvas().present_vsync().build().unwrap();

        canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();

        Ok(Screen {
            canvas
        })
    }

    pub fn clear(&mut self) {
        self.canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
        self.canvas.clear();
    }

    pub fn draw(&mut self, x: i32, y: i32, color: Color) -> Result<(), String> {
        let scale = 4;
        self.canvas.set_draw_color(color);
        self.canvas.fill_rect(Rect::new(scale * x, scale * y, 4, 4))
    }
}

pub trait Speaker {
    fn start_wav_file(&mut self, file_name: &str);
    fn stop_wav_file(&mut self, file_name: &str);
    fn play_wav_file(&mut self, file_name: &str);

}

pub struct SpaceInvadersSpeaker {
    audio: sdl2::AudioSubsystem,
    sounds: HashMap<String, Music<'static>>
}

impl SpaceInvadersSpeaker {
    pub fn new(sdl_context: &sdl2::Sdl) -> Self {
        sdl2::mixer::open_audio(11025, sdl2::mixer::AUDIO_U8, sdl2::mixer::DEFAULT_CHANNELS, 1_024).unwrap();
        sdl2::mixer::init(sdl2::mixer::InitFlag::all()).unwrap();
        sdl2::mixer::allocate_channels(6);
        SpaceInvadersSpeaker {
            audio: sdl_context.audio().unwrap(),
            sounds: HashMap::new()
        }
    }
}

impl Speaker for SpaceInvadersSpeaker {
    fn start_wav_file(&mut self, file_name: &str) {
        if !self.sounds.contains_key(file_name) {
            self.sounds.insert(file_name.to_string(), Music::from_file(file_name).unwrap());
        }
        self.sounds.get(file_name).unwrap().play(-1).unwrap()
    }
    fn stop_wav_file(&mut self, file_name: &str) {
        if let Some(sound) = self.sounds.get(file_name) {
            sound.play(0).unwrap();
        }
    }
    fn play_wav_file(&mut self, file_name: &str) {
        if !self.sounds.contains_key(file_name) {
            self.sounds.insert(file_name.to_string(), Music::from_file(file_name).unwrap());
        }
        self.sounds.get(file_name).unwrap().play(1).unwrap()
    }
}