use std::fmt;

/// Estrutura pilha
#[derive(Debug, Clone)]
pub struct Stack<T> {
    // Implementaremos usando um vetor por baixo dos panos
    inner: Vec<T>,
    // Tamanho máximo, definido ao instanciar
    size: usize,
}

impl<T> Stack<T> {
    /// Criar nova pilha, dado tamanho
    pub fn new(size: usize) -> Stack<T> {
        Stack {
            inner: Vec::new(),
            size,
        }
    }
    /// Adicionar elemento no topo da pilha
    /// Retorna StackError::Full, caso esteja cheia
    pub fn push(&mut self, element: T) -> Result<(), StackError> {
        // Caso já esteja cheio
        if self.inner.len() >= self.size {
            Err(StackError::Full(self.size))
        } else {
            self.inner.push(element);
            Ok(())
        }
    }
    /// Retira o primeiro elemento da pilha
    /// Retorna StackError::Empty, caso esteja vazia
    pub fn pop(&mut self) -> Result<T, StackError> {
        // Retira o elemento do vetor
        // Caso esteja vazio (retorna None), retornar o erro "Empty"
        self.inner.pop().ok_or(StackError::Empty)
    }
}

#[derive(Debug, Clone, Copy)]
/// Classe de erros da pilha
pub enum StackError {
    /// Pilha cheia
    /// (Contém seu tamanho máximo, para printar o erro)
    Full(usize),
    /// Pilha vazia
    Empty,
}

/// Implementação de traço Display, para imprimir mensagem de erro
impl fmt::Display for StackError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StackError::Full(size) => write!(f, "a pilha já está cheia ({} elementos)", size),
            StackError::Empty => write!(f, "a pilha está vazia"),
        }
    }
}
