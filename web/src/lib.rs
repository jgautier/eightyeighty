mod utils;
use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use emulator::cpu::Cpu;
use web_sys::{Window, Request, RequestInit, RequestMode, Response, KeyboardEvent};
use std::fs;
use emulator::machines::IO;
use emulator::machines::Speaker;
use emulator::machines::spaceinvaders::{SpaceInvadersIO, SpaceInvaders};
use emulator::machines::{Controller, Player, ButtonState, Button};
use wasm_bindgen_futures::JsFuture;
use std::rc::Rc;
use std::cell::RefCell;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, web!");
}

struct WebSpeaker {}
impl Speaker for WebSpeaker {
    fn start_wav_file(&mut self, file_name: &str) {

    }
    fn stop_wav_file(&mut self, file_name: &str) {

    }
    fn play_wav_file(&mut self, file_name: &str) {

    }
}

#[wasm_bindgen]
pub async fn start_spaceinvaders() {
    let window = web_sys::window().unwrap();

    let url = "/resources/spaceinvaders/invaders";
    let mut opts = RequestInit::new();
    opts.method("GET");
    let request = Request::new_with_str_and_init(&url, &opts).unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await.unwrap();
    let resp: Response = resp_value.dyn_into().unwrap();
    let buffer = JsFuture::from(resp.array_buffer().unwrap()).await.unwrap();
    let typed_buf : js_sys::Uint8Array = js_sys::Uint8Array::new(&buffer);

    let mut bytes = vec![0; typed_buf.length() as usize];
    typed_buf.copy_to(&mut bytes);
   
    let space_invdaers = SpaceInvaders::new(bytes);
    space_invdaers.play();
}