use crate::{Device, VideoDevice};

/// Uma placa de vídeo da nvidia
pub struct NvidiaGpu {
    powered: bool,
}

impl NvidiaGpu {
    /// Método construtor
    pub fn new() -> Self {
        NvidiaGpu { powered: false }
    }
}

/// Suas funcionalidades de dispositivo
impl Device for NvidiaGpu {
    fn power_on(&mut self) {
        self.powered = true
    }
    fn power_off(&mut self) {
        self.powered = false
    }
    fn check_status(&self) -> bool {
        self.powered
    }
    fn calibrate(&mut self) {
        println!("Calibrando...")
    }
}

/// Suas funcionalidades de dispositivo de vídeo
impl VideoDevice for NvidiaGpu {
    fn render(&self) {
        println!("Renderizando...")
    }
}
