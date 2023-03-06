use gameboy::{lcd::GameboyLCD, Gameboy};
use gloo::timers::callback::Interval;

#[derive(Default)]
pub enum RunningState {
    #[default]
    Paused,
    Running,
}

pub struct Debugger {
    running_state: RunningState,
    gameboy: Gameboy,
}

impl Debugger {
    pub fn new() -> Self {
        let mut gameboy = Gameboy::default();
        gameboy.bind_lcd(GameboyLCD::default());
        Debugger {
            running_state: RunningState::Paused,
            gameboy,
        }
    }

    pub fn step(&mut self) {
        self.gameboy.step_single_frame();
    }

    pub fn get_frame_buffer(&self) -> Vec<u8> {
        let lcd = &self.gameboy.ppu.lcd.as_ref().unwrap();
        return lcd.front_buffer().to_vec();
    }

    pub fn load_rom(&mut self, rom: &[u8]) {
        self.gameboy = gameboy::Gameboy::default();
        self.gameboy.bind_lcd(GameboyLCD::default());
        self.gameboy.load_rom(rom, None);
    }
}
