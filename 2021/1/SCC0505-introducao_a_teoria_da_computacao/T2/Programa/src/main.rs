use scc050_t2::{Direction, TuringMachine};
use std::io::{stdin, BufRead};

fn main() {
    // Lockar stdin
    let stdin = stdin();
    // Ler entrada com info da maquina
    let input = Input::from_reader(&mut stdin.lock());
    // Ler fitas
    let tapes = read_tapes(&mut stdin.lock());

    // Criar máquina de turing a partir do input
    let turing_machine = TuringMachine::from(input);

    // Para cada fita
    for tape in tapes {
        // Testar a fita e ver se aceita ou rejeita
        if turing_machine.run_tape(tape) {
            println!("aceita")
        } else {
            println!("rejeita")
        }
    }
}

/// Representa a entrada do programa, baseado nas especificações
/// As especificações são um pouco menos genericas do que a nossa maquina,
/// logo essa estrutura cuida de ler e facilitar na conversão
struct Input {
    number_of_states: u16,
    input_symbols: Vec<char>,
    tape_symbols: Vec<char>,
    accepting_state: u16,
    transitions: Vec<(u16, char, u16, char, Direction)>,
}

impl Input {
    /// Partindo de um BufRead (por exemplo, entrada padrão), lê e retorna a estrutura Input
    fn from_reader(reader: &mut (dyn BufRead)) -> Input {
        let mut lines = reader.lines();

        // Primeira linha
        let number_of_states = lines
            .next()
            .expect("Não foi possível ler entrada")
            .expect("Não foi possível ler entrada")
            .parse()
            .expect("O número de estados deve ser numérico");

        // Segunda linha
        let input_symbols = lines
            .next()
            .expect("Não foi possível ler entrada")
            .expect("Não foi possível ler entrada")
            .split_whitespace()
            .skip(1)
            .map(|word| word.chars().next().expect("Digite um símbolo válido"))
            .collect();

        // Terceira linha
        let tape_symbols = lines
            .next()
            .expect("Não foi possível ler entrada")
            .expect("Não foi possível ler entrada")
            .split_whitespace()
            .skip(1)
            .map(|word| word.chars().next().expect("Digite um símbolo válido"))
            .collect();

        // Quarta linha
        let accepting_state = lines
            .next()
            .expect("Não foi possível ler entrada")
            .expect("Não foi possível ler entrada")
            .parse()
            .expect("O estado de aceitação deve ser numérico");

        // Quinta linha
        let transitions_qty = lines
            .next()
            .expect("Não foi possível ler entrada")
            .expect("Não foi possível ler entrada")
            .parse()
            .expect("O número de transições deve ser numérico");

        // Transições
        let transitions = lines
            .by_ref()
            .take(transitions_qty)
            .map(|line| {
                let line = line.expect("Não foi possível ler entrada");
                let mut words = line.split_whitespace();
                let source_state = words
                    .next()
                    .expect("Não foi possível ler entrada")
                    .parse()
                    .expect("O estado origem da transição deve ser numérico");
                let source_symbol = words
                    .next()
                    .expect("Não foi possível ler entrada")
                    .chars()
                    .next()
                    .expect("O símbolo da transição ser um caractere válido");
                let target_state = words
                    .next()
                    .expect("Não foi possível ler entrada")
                    .parse()
                    .expect("O estado destino da transição deve ser numérico");
                let target_symbol = words
                    .next()
                    .expect("Não foi possível ler entrada")
                    .chars()
                    .next()
                    .expect("O símbolo destino da transição deve ser um caractere válido");
                let direction = words
                    .next()
                    .expect("Não foi possível ler entrada")
                    .chars()
                    .next()
                    .expect("Digite a direção da transição")
                    .into();
                (source_state, source_symbol, target_state, target_symbol, direction)
            }).collect();

        Input {
            number_of_states,
            input_symbols,
            tape_symbols,
            accepting_state,
            transitions,
        }
    }
}

impl From<Input> for TuringMachine {
    /// Converter o input para nossa turing machine
    fn from(input: Input) -> TuringMachine {
        TuringMachine {
            input_symbols: input.input_symbols.into_iter().collect(),
            tape_symbols: input.tape_symbols.into_iter().collect(),
            blank_symbol: 'B',
            states: (0..input.number_of_states).collect(),
            initial_state: 0,
            accepting_states: vec![input.accepting_state].into_iter().collect(),
            transitions: input.transitions.into_iter().map(|transition| {
                let (src_st, src_sy, dst_st, dst_sy, dir) = transition;
                ((src_st, src_sy), (dst_st, dst_sy, dir))
            }).into_iter().collect(),
        }
    }
}

/// Ler fitas do input
fn read_tapes(reader: &mut (dyn BufRead)) -> Vec<Vec<char>> {
    let mut lines = reader.lines();
    // Qtde de fitas
    let tapes_qty = lines
        .next()
        .expect("Não foi possível ler entrada")
        .expect("Não foi possível ler entrada")
        .parse()
        .expect("O número de fitas deve ser numérico");

    // Fitas
    lines.by_ref()
        // Pegar as próximas linhas igual ao numero de fitas
        .take(tapes_qty)
        // Para cada linha, transformar em chars e coletar num vetor
        .map(|line| line.unwrap().chars().collect::<Vec<char>>())
        .collect()
}
