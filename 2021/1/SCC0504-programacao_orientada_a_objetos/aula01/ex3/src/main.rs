// O módulo polynomial está em src/polynomial.rs, e é exportado em src/lib.rs
//
// Esse (e outros programas que eu farei ao longo da disciplina), são estruturados como uma
// biblioteca (cuja raíz é lib.rs), que é consumida aqui na main.rs
//
// Aqui vamos importar a estrutura Polynomial que definimos lá
use aula01_ex3::polynomial::Polynomial;

fn main() -> Result<(), String> {
    // Construir uma nova instância
    let mut poli = Polynomial::new(3);

    // 4.5x^3
    poli.add(4.5, 3)?;
    // -2x^3
    poli.add(-2.0, 2)?;
    // 2x
    poli.add(2.0, 1)?;
    // -1x
    poli.add(-1.0, 1)?;
    // -4
    poli.add(-4.0, 0)?;

    // Esse irá causar um erro, pois ultrapassa o grau máximo
    // poli.add(1.0, 10)?;

    println!("Polinômio:");
    println!("P(x) = {}", poli);
    println!("P(2) = {}", poli.calculate(2.0));

    Ok(())
}
