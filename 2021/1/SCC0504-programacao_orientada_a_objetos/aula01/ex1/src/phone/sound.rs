pub struct Sound {
    volume: f32,
    ringtone: String,
}

impl Sound {
    fn set_ringtone(&mut self, ringtone: &str) {
        self.ringtone = ringtone.into();
        println!("Toque alterado para {}", ringtone);
    }
    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume;
        println!("Toque alterado para {}%", volume * 100.0);
    }
    pub fn play_ringtone(&self) {
        println!(
            "Finge que eu tô tocando o som {} no volume {}",
            self.ringtone, self.volume
        );
    }
}

impl Default for Sound {
    fn default() -> Self {
        Sound {
            volume: 1.0,
            ringtone: "never_gonna_give_you_up.mp3".into(),
        }
    }
}
