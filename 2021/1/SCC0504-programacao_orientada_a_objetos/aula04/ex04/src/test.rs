use crate::Stack;

pub fn stack_test() {
    // Criar pilha
    let mut stack: Stack<String> = Stack::new(2);

    // Inserir na pilha, tratando erros com 'match'
    match stack.push("The quick brown fox jumps over the lazy dog".into()) {
        Ok(_) => println!("Inserido com sucesso."),
        Err(e) => println!("Não foi possível inserir: {}", e),
    }
    match stack.push("Testando 321".into()) {
        Ok(_) => println!("Inserido com sucesso."),
        Err(e) => println!("Não foi possível inserir: {}", e),
    }
    match stack.push("Isso aqui deve falhar".into()) {
        Ok(_) => println!("Inserido com sucesso."),
        Err(e) => println!("Não foi possível inserir: {}", e),
    }

    // Retirar da plha, tratando erros com 'match'
    match stack.pop() {
        Ok(x) => println!("Removido com sucesso: {}", x),
        Err(e) => println!("Não foi possível retirar: {}", e),
    }
    match stack.pop() {
        Ok(x) => println!("Removido com sucesso: {}", x),
        Err(e) => println!("Não foi possível retirar: {}", e),
    }
    match stack.pop() {
        Ok(x) => println!("Removido com sucesso: {}", x),
        Err(e) => println!("Não foi possível retirar: {}", e),
    }
}
