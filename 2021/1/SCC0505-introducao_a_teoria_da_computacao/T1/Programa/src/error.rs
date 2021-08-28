use std::fmt;

/// Possíveis erros que podem ocorrer
#[derive(Debug)]
pub enum AutomatonError {
    /// Ocorre quando uma transição não é válida
    InvalidTransition(u16),
    /// Ocorre com erros relacionados a leitura de input
    Io(std::io::Error),
    /// Ocorre quando o usuário digita algo que não é número
    NotANumber(std::num::ParseIntError),
}

impl fmt::Display for AutomatonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AutomatonError::InvalidTransition(q) => {
                write!(f, "A transição é inválida, pois o estado {} não existe", q)
            }
            AutomatonError::Io(e) => {
                write!(f, "Erro de leitura: {:?}", e)
            }
            AutomatonError::NotANumber(e) => {
                write!(f, "Entrada inválida, digite um número ({:?})", e)
            }
        }
    }
}

impl From<std::io::Error> for AutomatonError {
    fn from(e: std::io::Error) -> Self {
        AutomatonError::Io(e)
    }
}

impl From<std::num::ParseIntError> for AutomatonError {
    fn from(e: std::num::ParseIntError) -> Self {
        AutomatonError::NotANumber(e)
    }
}

impl std::error::Error for AutomatonError {}

pub type Result<T> = std::result::Result<T, AutomatonError>;
