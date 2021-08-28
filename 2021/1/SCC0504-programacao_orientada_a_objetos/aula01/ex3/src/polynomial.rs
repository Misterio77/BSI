// Módulo de formatação e coleções básicas da standard library
use std::{collections, fmt};

/// Representa um polinômio
#[derive(Debug, Clone)]
pub struct Polynomial {
    /// Grau máximo
    max_degree: u32,
    /// Hashmap mapeando grau -> coeficiente
    terms: collections::HashMap<u32, f32>,
}

impl Polynomial {
    /// Cria um novo polinômio, dado grau máximo
    pub fn new(max_degree: u32) -> Polynomial {
        Polynomial {
            max_degree,
            // Inicializar novo hashmap
            terms: collections::HashMap::new(),
        }
    }
    /// Adiciona um termo ao polinômio, somando caso o termo já exista
    ///
    /// Retorna um `Err` caso o grau do termo ultrapasse o grau máximo
    pub fn add(&mut self, coefficient: f32, degree: u32) -> Result<(), String> {
        if degree > self.max_degree {
            return Err(format!(
                "Couldn't add term, as specified degree ({}) is larger than the polynomial max degree ({})",
                degree,
                self.max_degree
            ));
        }
        // Obtêm o termo dado chave grau, e adiciona o coeficiente dado em seu valor
        // Caso não exista, coloca 0 incialmente (que daí vai somar com coeficiente)
        *self.terms.entry(degree).or_insert(0.0) += coefficient;
        Ok(())
    }
    /// Calcula o valor de um polinômio, dado x
    pub fn calculate(&self, x: f32) -> f32 {
        self.terms
            // Iterar pelos termos do hashmap
            .iter()
            // Para cada elemento, multiplicar e elevar de acordo com os valores
            .map(|(degree, coefficient)| *coefficient * x.powi(*degree as i32))
            // Somar cada elemento do iterador, resultando na soma que queríamos
            .sum()
    }
}

/// Traço de display padrão
// Esse traço é usado para fazer "pretty print" de uma estrutura qualquer
// Daí dá pra simplesmente imprimir ele em qualquer macro de formatação (inclusive print!())
impl fmt::Display for Polynomial {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Clonar o hashmap em um Vetor de tuples (u32, f32)
        let mut sorted: Vec<(u32, f32)> = self.terms.clone().into_iter().collect();
        // Ordenar esse vetor, com base no grau do elemento
        sorted.sort_by(|a, b| b.0.cmp(&a.0));

        let mut is_first = true;
        // Iterar pelo vetor ordenado
        for (degree, coefficient) in sorted.iter() {
            // Coeficiente
            if is_first {
                // Caso seja o primeiro elemento, imprimir sinal apenas se negativo (formatador
                // padrão de f32)
                write!(f, "{}", coefficient)?;
                is_first = false;
            } else {
                // Em qualquer outro elemento, sempre imprimir o coeficiente com sinal explicito
                write!(f, "{:+}", coefficient)?;
            }

            // X e grau
            if *degree == 1 {
                write!(f, "x")?;
            } else if *degree != 0 {
                write!(f, "x^{}", degree)?;
            }
        }
        Ok(())
    }
}
