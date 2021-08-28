use std::fs;
use std::io;
fn main() {
    println!("Digite o caminho do arquivo");

    // Caminho do arquivo
    let path = {
        // Buffer de leitura
        let mut buffer = String::new();
        // Ler linha
        io::stdin().read_line(&mut buffer).expect("Você deve especificar um caminho");
        // Tirar \n no fim
        buffer.pop();
        // Retornar buffer
        buffer
    };

    // Ler arquivo numa string
    let content = fs::read_to_string(&path).expect("Não foi possível abrir o arquivo");
    // Substituir muito por pouco
    let new_content = content.replace("muito", "pouco");

    // Escrever string alterada
    fs::write(&path, new_content).expect("Não foi possível escrever no arquivo");
}
