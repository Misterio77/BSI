/// Parte compartilhada dos componentes
pub struct ComponentCore {
    /// Se o componente está ligado ou não
    powered: bool,
}

impl ComponentCore {
    /// Liga/desliga componente
    pub fn set_power(&mut self, powered: bool) {
        self.powered = powered;
    }
    /// Retorna status de ligado/desligado do componente
    pub fn is_powered(&self) -> bool {
        self.powered
    }
    /// Testa o componente
    pub fn test(&self) -> bool {
        // Por enquanto isso vai ser um placeholder bobinho
        let pegando_fogo = false;

        // Retornar ok se estiver ligando e não estiver em chamas
        !pegando_fogo && self.is_powered()
    }
}

/// "Construtor" padrão (que não depende de argumento nenhum)
impl Default for ComponentCore {
    fn default() -> Self {
        ComponentCore {
            // Vamos fazer vir desligado como padrão
            powered: false,
        }
    }
}
