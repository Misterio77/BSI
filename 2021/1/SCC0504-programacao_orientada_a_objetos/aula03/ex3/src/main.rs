use aula03_ex3::{Bicycle, Car, CarSize, CarbonFootprint, House, School};

fn main() {
    // Vetor de ponteiros para estruturas que tem o traço da pegada de carbono
    let mut objetos: Vec<Box<dyn CarbonFootprint>> = Vec::new();

    // Criar uninho, com 1000km rodados por mes
    let uninho = Car::new(1000, CarSize::Small);
    objetos.push(Box::new(uninho));

    // Criar golzinho, com 2000km rodados por mes
    let golzinho = Car::new(2000, CarSize::Small);
    objetos.push(Box::new(golzinho));

    // Criar casa, com 350kwh mensais e 3 membros
    let casa = House::new(350, 3);
    objetos.push(Box::new(casa));

    // Criar escola, com 5000kwh mensais e 100 alunos
    let escola = School::new(5000, 100);
    objetos.push(Box::new(escola));

    // Criar bicicleta, com 500km rodados por mes
    let bicicleta1 = Bicycle::new(500);
    objetos.push(Box::new(bicicleta1));

    // Criar bicicleta, com 300km rodados por mes
    let bicicleta2 = Bicycle::new(300);
    objetos.push(Box::new(bicicleta2));

    for objeto in objetos.iter() {
        println!("{}g de CO2-equivalente/mês", objeto.get_carbon_footprint())
    }
}
