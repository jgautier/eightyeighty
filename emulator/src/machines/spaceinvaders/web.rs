use std::cell::RefCell;
use crate::machines::{Screen, Speaker, Controller, ButtonState, Button, Player, Machine};
use crate::machines::spaceinvaders::{SpaceInvaders, SpaceInvadersIO};
use crate::cpu::Cpu;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Window, KeyboardEvent, CanvasRenderingContext2d, HtmlAudioElement};
use std::rc::Rc;
use std::time::{Instant, Duration};
use std::thread;
use std::convert::TryInto;
use std::collections::HashMap;

const RESOURCE_PREFIX: &str = "/resources/spaceinvaders/";

pub struct WebSpeaker {
    sounds: HashMap<String, HtmlAudioElement>
}

impl WebSpeaker {
    fn new() -> Self {
        WebSpeaker {
            sounds: HashMap::new()
        }
    }
}

impl Speaker for WebSpeaker {
    fn start_wav_file(&mut self, file_name: &str) {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let file_name = &(RESOURCE_PREFIX.to_string() + file_name);
        if !self.sounds.contains_key(file_name) {
            let audioElement = HtmlAudioElement::new_with_src(file_name).unwrap();        
            document.append_with_node_1(&audioElement);
            self.sounds.insert(file_name.to_string(), audioElement);
            self.sounds.get(file_name).unwrap().set_loop(true);
        }
        self.sounds.get(file_name).unwrap().play();
    }
    fn stop_wav_file(&mut self, file_name: &str) {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let file_name = &(RESOURCE_PREFIX.to_string() + file_name);
        if self.sounds.contains_key(file_name) {
            self.sounds.get(file_name).unwrap().pause();
        }
    }
    fn play_wav_file(&mut self, file_name: &str) {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let file_name = &(RESOURCE_PREFIX.to_string() + file_name);
        if !self.sounds.contains_key(file_name) {
            let audioElement = HtmlAudioElement::new_with_src(file_name).unwrap();        
            document.append_with_node_1(&audioElement);
            self.sounds.insert(file_name.to_string(), audioElement);
        }
        self.sounds.get(file_name).unwrap().play();
    }
}

impl SpaceInvaders {
    pub fn new(bytes: Vec<u8>) -> Self {
        SpaceInvaders {
            io: RefCell::new(SpaceInvadersIO::new(Box::new(WebSpeaker::new()))),
            cpu: Cpu::new(bytes),
            screen: Box::new(WebScreen::new()),
            controller: Box::new(KeyboardController::new())
        }
    }

    pub fn play(mut self) {
        let window = web_sys::window().unwrap();
        let frame_callback = Closure::wrap(Box::new(move || {
            self.run_next_frame();
        }) as Box<dyn FnMut()>);
        window.set_interval_with_callback_and_timeout_and_arguments_0(frame_callback.as_ref().unchecked_ref(), 16).unwrap();
        frame_callback.forget();
    }
}

pub struct KeyboardController {
    button_events: Rc<RefCell<Vec<(Player, Button)>>>
}

impl KeyboardController {
    fn new() -> Self {
        let button_events = Rc::new(RefCell::new(Vec::new()));
        let window = web_sys::window().unwrap();
        let button_events_keydown = button_events.clone();
        let keydown_listener = Closure::wrap(Box::new(move |event: KeyboardEvent| {
            match event.code().as_str() {
                "KeyC" => {
                    button_events_keydown.borrow_mut().push((Player::Player1, Button::Coin(ButtonState::Down)));
                }
                "Digit1" => {
                    button_events_keydown.borrow_mut().push((Player::Player1, Button::OnePlayer(ButtonState::Down)));
                }
                "ArrowLeft" => {
                    button_events_keydown.borrow_mut().push((Player::Player1, Button::Left(ButtonState::Down)));
                }
                "ArrowRight" => {
                    button_events_keydown.borrow_mut().push((Player::Player1, Button::Right(ButtonState::Down)));
                }
                "Space" => {
                    button_events_keydown.borrow_mut().push((Player::Player1, Button::Shoot(ButtonState::Down)));
                }
                _ => {
                    // unhandled
                }
            }
        }) as Box<dyn FnMut(_)>);
        let button_events_keyup = button_events.clone();
        let keyup_listener = Closure::wrap(Box::new(move |event: KeyboardEvent| {
            match event.code().as_str() {
                "KeyC" => {
                    button_events_keyup.borrow_mut().push((Player::Player1, Button::Coin(ButtonState::Up)));
                },
                "Digit1" => {
                    button_events_keyup.borrow_mut().push((Player::Player1, Button::OnePlayer(ButtonState::Up)));
                },
                "ArrowLeft" => {
                    button_events_keyup.borrow_mut().push((Player::Player1, Button::Left(ButtonState::Up)));
                }
                "ArrowRight" => {
                    button_events_keyup.borrow_mut().push((Player::Player1, Button::Right(ButtonState::Up)));
                },
                "Space" => {
                    button_events_keyup.borrow_mut().push((Player::Player1, Button::Shoot(ButtonState::Up)));
                }
                _ => {
                    // unhandled
                }

            }
        }) as Box<dyn FnMut(_)>);
        window.add_event_listener_with_callback("keydown", keydown_listener.as_ref().unchecked_ref()).unwrap();
        window.add_event_listener_with_callback("keyup", keyup_listener.as_ref().unchecked_ref()).unwrap();
        keydown_listener.forget();
        keyup_listener.forget();
        KeyboardController {
            button_events: button_events
        }
    }
}

impl Controller for KeyboardController {
    fn get_button_states(&mut self) -> Vec<(Player, Button)> {
        let ret_vec = self.button_events.borrow().to_vec();
        self.button_events.borrow_mut().clear();
        return ret_vec;
    }
}

pub struct WebScreen {
    context: CanvasRenderingContext2d
}

impl WebScreen {
    fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let canvas = document.get_element_by_id("canvas").unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();
    
        canvas.set_width(224 * 4);
        canvas.set_height(256 * 4);
    
        
        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        WebScreen {
            context
        }
    }
}

impl Screen for WebScreen {
    fn clear(&mut self) {
        self.context.set_fill_style(&"black".into());
        self.context.fill_rect(0f64, 0f64, (224 * 4) as f64, (256 * 4) as f64);
    }

    fn draw(&mut self, x: i32, y: i32, color: (u8, u8, u8)) {
        self.context.set_fill_style(&format!("rgb({},{},{}", color.0, color.1, color.2).into());
        self.context.fill_rect((x * 4) as f64, (y * 4) as f64, 4.0, 4.0);
    }

    fn present(&mut self) {
        // no op for now
    }
}