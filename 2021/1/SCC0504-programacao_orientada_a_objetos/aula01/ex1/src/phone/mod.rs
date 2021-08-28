pub mod communication;
pub mod cpu;
pub mod sound;
pub mod touchscreen;

use communication::Communication;
use cpu::Cpu;
use sound::Sound;
use touchscreen::TouchScreen;

pub struct Phone {
    cpu: Cpu,
    screen: TouchScreen,
    sound: Sound,
    comms: Communication,
}

impl Phone {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn demo(&mut self) {
        println!("Modo de demonstração:");
        println!();
        self.comms.connect_wifi("Casa");
        self.comms.send_hello_message();
        self.cpu.set_power_saver(true);
        self.sound.set_volume(0.5);
        self.sound.play_ringtone();
        self.screen.off();
    }
}

impl Default for Phone {
    fn default() -> Self {
        Phone {
            cpu: Cpu::default(),
            screen: TouchScreen::default(),
            sound: Sound::default(),
            comms: Communication::default(),
        }
    }
}
