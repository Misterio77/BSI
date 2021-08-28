use aula01_ex2::operatingsystem::OperatingSystem;

fn main() {
    let mut os = OperatingSystem::new();

    println!("OS instanciado! Vamos testar?");

    os.demo();

    println!("Puts, esquecemos de ligar, vamos ver agora vai");

    os.set_power(true);
    os.demo();

    println!("Show!");
}
