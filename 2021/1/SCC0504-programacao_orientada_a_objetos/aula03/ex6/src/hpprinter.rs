use crate::Device;

/// Uma impressora da HP
pub struct HpPrinter {
    powered: bool,
    out_of_ink: bool,
}

impl HpPrinter {
    /// Método construtor
    pub fn new() -> Self {
        HpPrinter {
            powered: false,
            out_of_ink: true,
        }
    }
    /// Encher de tinta
    pub fn fill(&mut self) {
        self.out_of_ink = false;
    }
}

/// Suas funcionalidades de dispositivo
impl Device for HpPrinter {
    fn power_on(&mut self) {
        self.powered = true
    }
    fn power_off(&mut self) {
        self.powered = false
    }
    fn check_status(&self) -> bool {
        self.powered && !self.out_of_ink
    }
    fn calibrate(&mut self) {
        println!("Calibrando...")
    }
}
