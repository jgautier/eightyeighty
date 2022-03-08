pub mod spaceinvaders;

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

pub trait Controller {
    fn get_button_states(&mut self) -> Vec<(Player, Button)>;
}