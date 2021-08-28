use std::collections::HashMap;

use petgraph::graph::NodeIndex;
use petgraph::visit::EdgeRef;
use petgraph::{Direction, Graph};

use crate::{AutomatonError, Result};

/// Um autômato
pub struct Automaton {
    /// Estados e transições
    ///
    /// Cada nó é uma tupla com um estado (inteiro positivo, nulo no caso do estado inicial), e uma
    /// boolean que indica se aquele estado é um estado final aceitável
    /// Cada aresta é um caractere (ou um nulo, no caso de epsilon), que é um símbolo do alfabeto da
    /// linguagem
    graph: Graph<(Option<u16>, bool), Option<char>>,
    /// Estado(s) iniciais (são armazenados índices p/ nós do grafo)
    initial_state: NodeIndex,
}

impl Automaton {
    /// Cria um novo autômato, dado vetor de estados (u16), símbolos (char),
    /// estados iniciais (u16), estados aceitos (u16) e transições (tripla u16, char, u16)
    pub fn new(
        states: &[u16],
        initial_states: &[u16],
        accepted_states: &[u16],
        transitions: &[(u16, char, u16)],
    ) -> Result<Automaton> {
        // Criar um grafo, com tamanho do número de estados (+ o inicial), e quantidade de arestas igual ao
        // número de transições
        let mut transitions_graph = Graph::with_capacity(states.len() + 1, transitions.len());

        // Estado inicial real do autômato. Os estados iniciais passados pelo usuário na verdade
        // serão filhos desse, com uma transição epsilon
        let initial_state_index = transitions_graph.add_node((None, false));

        // Vamos guardar os índices dos estados no  grafo num mapa hash temporário,
        // pra facilitar na hora de marcar as transições
        let mut index = HashMap::new();

        // Adicionar todos os estados em nós do grafo
        for state in states {
            // Ver se o estado é um dos aceitos
            let is_accepted = accepted_states.contains(state);
            // Adicionar o estado ao grafo (e se é aceito), e guardar seu índice
            let state_index = transitions_graph.add_node((Some(*state), is_accepted));

            // Adicionar o estado e seu índice no nosso hashmap
            index.insert(state, state_index);

            // Caso o estado seja um dos iniciais
            if initial_states.contains(&state) {
                // Adicionar uma conexão do estado inicial pra esse que acabamos de criar
                // Cujo símbolo é None (epsilon)
                transitions_graph.add_edge(initial_state_index, state_index, None);
            }
        }

        // Adicionar todos as transições em arestas do grafo
        for transition in transitions.iter() {
            // Pegar estado pré, símbolo, e estado pós
            let (q0, x, q1) = transition;

            // Verificar que o estado pré existe
            let q0 = *index
                .get(q0)
                .ok_or(AutomatonError::InvalidTransition(*q0))?;
            // Verificar que o estado pós existe
            let q1 = *index
                .get(q1)
                .ok_or(AutomatonError::InvalidTransition(*q1))?;
            // Verificar se o símbolo é epsilon
            let x = match x {
                // Caso seja epsilon, colocar None
                '-' => None,
                // Caso contrário, desreferenciar o símbolo
                x => Some(*x),
            };
            // Adicionar aresta ao grafo
            transitions_graph.add_edge(q0, q1, x);
        }

        // Retornar autômato criado
        Ok(Automaton {
            graph: transitions_graph,
            initial_state: initial_state_index,
        })
    }
    /// Dado uma cadeia, retorna se ela é válida ou não dentro da linguagem
    pub fn verify_chain(&self, chain: &[char]) -> bool {
        // Caso a cadeia contenha '-' (indica epsilon), transformar num slice vazio
        let chain = if chain.contains(&'-') { &[] } else { chain };
        // Chamar função recursiva de verificação
        self.verify_chain_inner(&self.initial_state, Vec::new(), chain)
    }
    /// Função recursiva de verificação (função interna)
    fn verify_chain_inner(
        &self,
        current: &NodeIndex,
        visited: Vec<&NodeIndex>,
        chain: &[char],
    ) -> bool {
        // Caso não tenha mais símbolos na cadeia, e a posição atual é uma das aceitas
        if chain.is_empty() && self.graph[*current].1 {
            // Aceitar cadeia
            return true;
        }

        // Pegar todas as transições saindo do estado atual
        let edges = self
            .graph
            .edges_directed(*current, Direction::Outgoing)
            .filter(|edge| !visited.contains(&&edge.target()));

        // Caso ainda tenha símbolos para seguir
        if !chain.is_empty() {
            // Pegar as arestas que contém o elemento atual da cadeia
            let matching_edges = edges
                .clone()
                .filter(|edge| *edge.weight() == Some(chain[0]));
            // Para cada aresta que é possível seguir
            for edge in matching_edges {
                // Seguir no vértice, tirando o elemento atual da cadeia
                if self.verify_chain_inner(&edge.target(), visited.clone(), &chain[1..]) {
                    // Caso em algum desses esteja verificado, propagar isso
                    return true;
                }
            }
        }

        // Tentar agora com as transições epsilon
        let epsilon_edges = edges.clone().filter(|edge| *edge.weight() == None);
        // Para cada aresta epsilon
        for edge in epsilon_edges {
            // Chamar ele, com a cadeia intacta (o elemento não é consumido ao entrar num epsilon)
            let mut visited = visited.clone();
            visited.push(current);
            if self.verify_chain_inner(&edge.target(), visited, chain) {
                // Caso em algum desses esteja verificado, propagar isso
                return true;
            }
        }

        // Se chegou aqui, é pq não tem nenhum aresta (nem mesmo epsilon) para seguir
        // E nenhum dos que tentamos seguir caíram num nó aceito
        // Nesse caso, retornar false
        false
    }
}
