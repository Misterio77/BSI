use super::core::ComponentCore;

/// Representa driver de vídeo
pub struct VideoDriver {
    /// Parte comum de componentes
    core: ComponentCore,
    /// Brilho atual
    brightness: f32,
}
impl VideoDriver {
    /// Altera brilho do driver
    pub fn set_brightness(&mut self, brightness: f32) {
        if self.core.is_powered() {
            self.brightness = brightness;
            println!("Brilho alterado para {}%", brightness * 100.0)
        } else {
            println!("Erro: componente desligado")
        }
    }
    pub fn set_power(&mut self, powered: bool) {
        self.core.set_power(powered);
    }
    pub fn test(&self) -> bool {
        self.core.test()
    }
}
/// Construtor padrão
impl Default for VideoDriver {
    fn default() -> Self {
        VideoDriver {
            core: ComponentCore::default(),
            brightness: 0.0,
        }
    }
}
