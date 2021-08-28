use std::fmt;

/// Conta corrente
pub struct CheckingAccount {
    /// Saldo disponível
    balance: f32,
    /// Limite total de crédito
    credit_limit: f32,
    /// Crédito utilizado
    credit_used: f32,
}

impl CheckingAccount {
    /// Criar nova conta corrente
    pub fn new() -> CheckingAccount {
        CheckingAccount {
            balance: 0.0,
            credit_limit: 0.0,
            credit_used: 0.0,
        }
    }
    /// Retirar uma quantia da conta corrente (usando limite de crédito quando nescessário)
    pub fn withdraw(&mut self, amount: f32) -> Result<(), CheckingAccountError> {
        // Caso a quantia seja inválida
        if amount <= 0.0 {
            Err(CheckingAccountError::NotPositive)
        // Caso o saldo comporte
        } else if amount <= self.balance {
            // Retirar do saldo
            self.balance -= amount;
            Ok(())
        // Caso nescessite do limite, e este limite disponível seja suficiente
        } else if amount <= (self.balance + (self.credit_limit - self.credit_used)) {
            // Adicionar ao crédito usado a diferença do saque e do saldo
            self.credit_used += amount - self.balance;
            // Zerar saldo
            self.balance = 0.0;
            Ok(())
        // Caso não seja suficiente
        } else {
            // (Colocamos o número para imprimir quanto é o máximo)
            Err(CheckingAccountError::TooBig(self.balance + (self.credit_limit - self.credit_used)))
        }
    }
    /// Depositar uma quantia na conta corrente (automaticamente paga e libera o limite de crédito)
    pub fn deposit(&mut self, amount: f32) -> Result<(), CheckingAccountError> {
        // Caso a quantia seja inválida
        if amount <= 0.0 {
            Err(CheckingAccountError::NotPositive)
        // Caso o depósito pague o limite usado totalmente
        // (vale para casos em que o limite usado é 0)
        } else if amount >= self.credit_used {
            // Adicionar ao saldo a diferença do deposito e do credito em uso
            self.balance += amount - self.credit_used;
            // Zerar o crédito em uso
            self.credit_used = 0.0;
            Ok(())
        // Caso não pague totalmente
        } else {
            // Pagar o quanto for possível do limite
            self.credit_used -= amount;
            Ok(())
        }
    }
    /// Alterar limite de crédito
    pub fn set_limit(&mut self, limit: f32) -> Result<(), CheckingAccountError> {
        // Verificar que o novo limite comporta a dívida do correntista
        if limit >= self.credit_used {
            self.credit_limit = limit;
            Ok(())
        } else {
            Err(CheckingAccountError::TooSmall(self.credit_used))
        }
    }
}

/// Implementação do traço display, para imprimir a conta corrente
impl fmt::Display for CheckingAccount {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Saldo: {}, Limite: {}/{}", self.balance, self.credit_used, self.credit_limit)
    }
}

pub enum CheckingAccountError {
    TooSmall(f32),
    TooBig(f32),
    NotPositive,
}

/// Implementação de traço Display, para imprimir mensagem de erro
/// Nota: implementei esse traço ao invés de fazer um "getMessage()", pq é o traço próprio para a
/// impressão de tipos no Rust.
impl fmt::Display for CheckingAccountError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CheckingAccountError::TooSmall(n) => write!(f, "o valor deve ser no mínimo {}", n),
            CheckingAccountError::TooBig(n) => write!(f, "o valor deve ser no máximo {}", n),
            CheckingAccountError::NotPositive => write!(f, "o valor deve ser um número estritamento positivo"),
        }
    }
}
