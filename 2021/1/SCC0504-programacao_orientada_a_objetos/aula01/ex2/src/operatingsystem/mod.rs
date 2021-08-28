// Rust não tem herança. Apenas traços (interfaces, que apenas definem métodos em comum, não campos)
// e composição. Também é possível definir campos e comportamento em comum usando generics, mas é
// um pouquinho complicado então vou deixar pra algum outro exercício.
//
// Nesse caso vou implementar o polimorfismo usando composição
// Um pequeno drawback é que eu tenho que dar forward nas funções, mas eu pessoalmente acho que
// isso é mais explicito do que herança.

mod core;

mod network;
mod printing;
mod video;

use network::NetworkDriver;
use printing::PrintingDriver;
use video::VideoDriver;

/// Sistema operacional
pub struct OperatingSystem {
    network: NetworkDriver,
    printing: PrintingDriver,
    video: VideoDriver,
}
impl OperatingSystem {
    pub fn new() -> Self {
        Self::default()
    }
    /// Liga/desliga o sistema completamente
    pub fn set_power(&mut self, powered: bool) {
        self.network.set_power(powered);
        self.printing.set_power(powered);
        self.video.set_power(powered);
    }
    /// Executa algumas funções demonstrativas dos componentes
    pub fn demo(&mut self) {
        if self.network.test() {
            self.network.send_data_package(4);
        } else {
            println!("Erro: rede não está funcional");
        }

        if self.printing.test() {
            self.printing.print_pages(&[1, 2, 3]);
        } else {
            println!("Erro: impressão não está funcional");
        }

        if self.video.test() {
            self.video.set_brightness(0.5);
        } else {
            println!("Erro: vídeo não está funcional");
        }
    }
}
/// Construtor padrão
impl Default for OperatingSystem {
    fn default() -> Self {
        OperatingSystem {
            network: NetworkDriver::default(),
            printing: PrintingDriver::default(),
            video: VideoDriver::default(),
        }
    }
}
