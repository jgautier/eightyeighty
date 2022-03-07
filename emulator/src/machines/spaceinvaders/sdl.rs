use sdl2::pixels;
use sdl2::rect::Rect;
use sdl2::mixer::Music;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::collections::HashMap;
use std::cell::RefCell;
use std::time::{Instant, Duration};
use std::thread;
use std::convert::TryInto;
use crate::cpu::Cpu;
use crate::machines::{Screen, Speaker, Controller, ButtonState, Button, Player, SpaceInvaders, SpaceInvadersIO, Machine};
pub struct Sdl2Screen {
    canvas: sdl2::render::WindowCanvas
}

impl Sdl2Screen {
    pub fn new(sdl_context: &sdl2::Sdl) -> Result<Sdl2Screen, String> {
        let video = sdl_context.video()?;
        let window = video.window("Space Invaders", 224 * 4, 256 * 4).position_centered().build().unwrap();

        let mut canvas = window.into_canvas().present_vsync().build().unwrap();

        canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();

        Ok(Sdl2Screen {
            canvas
        })
    }
}

impl Screen for Sdl2Screen {
    fn clear(&mut self) {
        self.canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
        self.canvas.clear();
    }

    fn draw(&mut self, x: i32, y: i32, color: (u8, u8, u8)) {
        let scale = 4;
        self.canvas.set_draw_color(color);
        self.canvas.fill_rect(Rect::new(scale * x, scale * y, 4, 4));
    }

    fn present(&mut self) {
        self.canvas.present();
    }
}

pub enum Sound {
    PlayerShoot
}

pub struct SpaceInvadersSpeaker {
    audio: sdl2::AudioSubsystem,
    sounds: HashMap<String, Music<'static>>
}

impl SpaceInvadersSpeaker {
    pub fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();
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

pub struct KeyboardController {
    event_pump: sdl2::EventPump,
}

impl KeyboardController {
    pub fn new(sdl_context: sdl2::Sdl) -> Self {
        KeyboardController {
            event_pump: sdl_context.event_pump().unwrap()
        }
    }
}

impl Controller for KeyboardController {
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


impl SpaceInvaders {
    pub fn new(bytes: Vec<u8>) -> Self {
        let sdl_context = sdl2::init().unwrap();
        SpaceInvaders {
            io: RefCell::new(SpaceInvadersIO::new()),
            cpu: Cpu::new(bytes),
            screen: Box::new(Sdl2Screen::new(&sdl_context).unwrap()),
            controller: Box::new(KeyboardController::new(sdl_context))
        }
    }

    pub fn play(mut self) {
        let frame_ms: u128 = 16;
        loop {
            let start = Instant::now();
            self.run_next_frame();
            let elapsed = start.elapsed().as_millis();
            if elapsed < frame_ms {
                thread::sleep(Duration::from_millis((frame_ms - elapsed).try_into().unwrap()));
            }
        }
    }
}