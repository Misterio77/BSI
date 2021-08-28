use aula01_ex1::phone::Phone;

fn main() {
    let mut samsung_s10 = Phone::new();
    // Ativar método público demo
    // Esse método chama vários métodos públicos
    // dos componentes, que por sua vez não são públicos a
    // quem meramente usa o celular, por exemplo.
    //
    // Alguns desses métodos públicos dos componentes chamam
    // outros métodos privados, exemplificando como pode
    // ser implementado um API que só expõe o nescessário, em
    // diferentes camadas (componente -> dispositivo -> usuário).
    samsung_s10.demo();
}
