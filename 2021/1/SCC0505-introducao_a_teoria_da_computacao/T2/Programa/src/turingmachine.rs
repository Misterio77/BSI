use std::collections::HashMap as Map;
use std::collections::HashSet as Set;

type Symbol = char;
type State = u16;

/// Direções em que podemos nos mover
#[derive(Debug, Copy, Clone)]
pub enum Direction {
    /// Mover p/ esquerda
    Left,
    /// Mover p/ direita
    Right,
    /// Não mover
    None,
}

/// Transformar caractere em direção
impl From<char> for Direction {
    fn from(f: char) -> Direction {
        match f {
            'R'|'r' => Direction::Right,
            'L'|'l' => Direction::Left,
            _ => Direction::None,
        }
    }
}

/// Representa uma máquina de turing
#[derive(Debug)]
pub struct TuringMachine {
    /// Conjunto de símbolos de entrada
    pub input_symbols: Set<Symbol>,
    /// Conjunto de símbolos possíveis, além dos de entrada
    pub tape_symbols: Set<Symbol>,
    /// Símbolo que representa o vazio
    pub blank_symbol: Symbol,
    /// Conjunto de estados possíveis
    pub states: Set<State>,
    /// Estado inicial
    pub initial_state: State,
    /// Conjunto de estados finais aceitáveis
    pub accepting_states: Set<State>,
    /// Representa a função de transição
    /// Implementado como um mapa
    pub transitions: Map<(State, Symbol), (State, Symbol, Direction)>,
}

impl TuringMachine {
    /// Executa a máquina de turing na fita inicial dada
    pub fn run_tape(&self, tape: Vec<Symbol>) -> bool {
        // Tomar posse da fita, como mutável
        let mut tape = tape;

        // Caso seja apenas "-", trocar pelo vetor vazio
        if tape.len() == 1 && tape[0] == '-' {
            tape = Vec::new();
        }

        // Verificar se algum símbolo na fita inicial não está nos permitidos
        if tape.iter().any(|s| !self.input_symbols.contains(s)) {
            return false;
        }

        // Iniciar na ponta esquerda da fita
        let mut index = 0;
        // Iniciar no estado inicial
        let mut current_state = self.initial_state;

        // Caso a fita esteja vazia, criar uma posição com vazio nela
        if tape.is_empty() {
            tape.push(self.blank_symbol);
        }

        loop {
            // Chamar a transição para ver para onde vamos
            let transition = self.transition(current_state, tape[index]);
            // Caso a transição leve a algum lugar
            if let Some(transition) = transition {
                let (next_state, write_symbol, direction) = transition;
                // Escrever símbolo na posição atual da fita
                tape[index] = write_symbol;
                // Guardar o novo estado
                current_state = next_state;
                // Andar com a fita
                match direction {
                    Direction::Left => {
                        // Caso nosso movimento passe do limite esquerdo
                        if index == 0 {
                            tape.insert(0, self.blank_symbol);
                        } else {
                            index -= 1;
                        }
                    }
                    Direction::Right => {
                        // Caso nosso movimento passe do limite direito
                        index += 1;
                        if index == tape.len() {
                            tape.push(self.blank_symbol);
                        }
                    }
                    Direction::None => {}
                }
            }
            // Caso não tenha para onde ir
            else {
                // Halt
                // Verificar se o estado atual é aceito, e retornar
                break self.accepting_states.contains(&current_state);
            }
        }
    }
    /// Dado estado e símbolo atual da fita, retorna o novo estado, símbolo a ser escrito, e
    /// direção que se deve seguir
    fn transition(
        &self,
        state: State,
        symbol: Symbol,
    ) -> Option<(State, Symbol, Direction)> {
        self.transitions.get(&(state, symbol)).copied()
    }
}
