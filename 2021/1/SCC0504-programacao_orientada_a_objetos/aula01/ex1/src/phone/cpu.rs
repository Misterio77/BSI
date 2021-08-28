pub struct Cpu {
    clock_mhz: f32,
}

impl Cpu {
    fn set_clock(&mut self, clock: f32) {
        self.clock_mhz = clock;
        println!("Clock do cpu alterado para {}", clock);
    }
    pub fn set_power_saver(&mut self, power_saver: bool) {
        if power_saver {
            println!("Entrando no modo de economia de energia");
            self.set_clock(0.6);
        } else {
            println!("Saindo do modo de economia de energia");
            self.set_clock(1.6);
        }
    }
}

impl Default for Cpu {
    fn default() -> Cpu {
        Cpu { clock_mhz: 1.6 }
    }
}
