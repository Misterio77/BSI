use super::core::ComponentCore;

/// Driver de impressão
pub struct PrintingDriver {
    core: ComponentCore,
}
impl PrintingDriver {
    /// Imprime um conjunto de páginas
    // Placeholder, vamos fingir que cada página é um byte
    pub fn print_pages(&self, pages: &[u8]) {
        if self.core.is_powered() {
            println!("Brrrr... imprimindo");
            for page in pages {
                println!("Imprimi '{}'", page);
            }
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
impl Default for PrintingDriver {
    fn default() -> Self {
        PrintingDriver {
            core: ComponentCore::default(),
        }
    }
}
