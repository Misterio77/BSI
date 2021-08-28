pub struct TouchScreen {
    brightness: f32,
}

impl TouchScreen {
    fn set_brightness(&mut self, brightness: f32) {
        self.brightness = brightness;
        println!("Brilho da tela alterado para {}%", brightness * 100.0);
    }
    pub fn off(&mut self) {
        self.set_brightness(0.0);
    }
}

impl Default for TouchScreen {
    fn default() -> Self {
        TouchScreen { brightness: 1.0 }
    }
}
