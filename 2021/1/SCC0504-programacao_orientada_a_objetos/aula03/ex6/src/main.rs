// Alguns exemplos de estruturas que implementam as interfaces da biblioteca

// Letra A (interface de dispositivo)
// Eu resolveria o assunto criando uma interface para os dispositivos.
// Com as informações da interface, desenvolvedores poderiam criar seus próprios drivers, sem
// nescessariamente saber como o sistema funciona (e chama as funções polimórficas)
use aula03_ex6::Device;
use aula03_ex6::HpPrinter;

// Exemplo de função polimórfica da letra A (para usar na main)
/// Essa função funciona com qualquer dispositivo que implemente o traço Device
fn exemplo_polimorfico_a<T: Device>(device: &mut T) {
    device.power_on();
    if device.check_status() {
        println!("Tudo certo!");
    };
}

// Letra B (família de dispositivos)
// Temos um traço VideoDevice (que exige que o Device esteja implementado), que age como uma
// especialização de Device.
use aula03_ex6::VideoDevice;
use aula03_ex6::NvidiaGpu;

// Exemplo de função polimórfica da letra B (para usar na main)
/// Essa função funciona com qualquer dispositivo que implemente o traço VideoDevice
fn exemplo_polimorfico_b<T: VideoDevice>(device: &mut T) {
    device.power_on();
    device.render();
}

fn main() {
    let mut printer = HpPrinter::new();
    let mut gpu = NvidiaGpu::new();

    // Encher a impressora com tinta
    printer.fill();
    // Exemplo polimorfico a (liga e verifica status)
    exemplo_polimorfico_a(&mut printer);

    // Exemplo polimorfico a (liga e verifica status)
    exemplo_polimorfico_a(&mut gpu);
    // Exemplo polimorfico b (liga e renderiza algo)
    exemplo_polimorfico_b(&mut gpu);
}
