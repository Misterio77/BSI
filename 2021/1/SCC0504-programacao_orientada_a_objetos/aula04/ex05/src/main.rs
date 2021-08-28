use lista04_ex5::CheckingAccount;

fn main() {
    let mut account = CheckingAccount::new();
    // Depositar 1000 reais
    let amount = 1000.0;
    match account.deposit(amount) {
        Ok(_) => println!("Depósito de {} feito com suceso", amount),
        Err(e) => println!("Não foi possível depositar {}: {}", amount, e),
    }
    // Colocar um pouco de limite de crédito
    let amount = 100.0;
    match account.set_limit(amount) {
        Ok(_) => println!("Limite alterado para {}", amount),
        Err(e) => println!("Não foi possível alterar o limite para {}: {}", amount, e),
    }
    // Mostrar a conta corrente na tela
    println!("Sua conta corrente: {}", account);
    println!();

    // Tentar sacar 1200 reais
    // (Dará erro)
    let amount = 1200.0;
    match account.withdraw(amount) {
        Ok(_) => println!("Saque de {} feito com suceso", amount),
        Err(e) => println!("Não foi possível sacar {}: {}", amount, e),
    }
    // Colocar mais limite de crédito
    let amount = 300.0;
    match account.set_limit(amount) {
        Ok(_) => println!("Limite alterado para {}", amount),
        Err(e) => println!("Não foi possível alterar o limite para {}: {}", amount, e),
    }
    // Tentar sacar 1200 reais, novamente
    let amount = 1200.0;
    match account.withdraw(amount) {
        Ok(_) => println!("Saque de {} feito com suceso", amount),
        Err(e) => println!("Não foi possível sacar {}: {}", amount, e),
    }
    // Mostrar a conta corrente na tela
    println!("Sua conta corrente: {}", account);
    println!();

    // Tentar abaixar o crédito de forma incorreta
    // (Dará erro)
    let amount = 100.0;
    match account.set_limit(amount) {
        Ok(_) => println!("Limite alterado para {}", amount),
        Err(e) => println!("Não foi possível alterar o limite para {}: {}", amount, e),
    }
    // Depositar uma quantia, que irá pagar o crédito usado
    let amount = 300.0;
    match account.deposit(amount) {
        Ok(_) => println!("Depósito de {} feito com suceso", amount),
        Err(e) => println!("Não foi possível depositar {}: {}", amount, e),
    }
    // Tentar abaixar o crédito, agora dá
    let amount = 100.0;
    match account.set_limit(amount) {
        Ok(_) => println!("Limite alterado para {}", amount),
        Err(e) => println!("Não foi possível alterar o limite para {}: {}", amount, e),
    }

    // Mostrar a conta corrente na tela
    println!("Sua conta corrente: {}", account);
    println!();


    // Tentar depositar 0 reais
    // (Dará erro)
    let amount = 0.0;
    match account.deposit(amount) {
        Ok(_) => println!("Depósito de {} feito com suceso", amount),
        Err(e) => println!("Não foi possível depositar {}: {}", amount, e),
    }
    // Tentar sacar -1 reais
    // (Dará erro)
    let amount = -1.0;
    match account.withdraw(amount) {
        Ok(_) => println!("Depósito de {} feito com suceso", amount),
        Err(e) => println!("Não foi possível sacar {}: {}", amount, e),
    }
}
