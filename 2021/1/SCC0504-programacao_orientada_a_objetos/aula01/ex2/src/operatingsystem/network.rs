use super::core::ComponentCore;

/// Driver de rede
pub struct NetworkDriver {
    core: ComponentCore,
}
impl NetworkDriver {
    /// Envia um dado
    // Placeholder, vamos fingir que o dado é um byte
    pub fn send_data_package(&self, data: u8) {
        if self.core.is_powered() {
            println!("Enviei pacote '{}'", data)
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
impl Default for NetworkDriver {
    fn default() -> Self {
        NetworkDriver {
            core: ComponentCore::default(),
        }
    }
}
