use aula02_ex5::{Character, Hero, Power, Villain};

fn info_attack(attack: Result<String, String>) {
    match attack {
        Ok(message) => println!("Ataque bem sucedido: {}", message),
        Err(message) => println!("Ataque falhou: {}", message),
    }
}

fn main() {
    // O superman é um herói forte e ágil
    let mut superman: Character<Hero> = Character::new("Superman", "Clark Kent");
    // Superman possui vários poderes interessantes
    superman.add_power(Power::Flight);
    superman.add_power(Power::Strength);
    superman.add_power(Power::Durability);

    // Que tal o batman como um (breve) rival?
    let mut batman: Character<Hero> = Character::new("Batman", "Bruce Wayne");
    // O batman pode não ter poderes sobrenaturais, mas tem habilidades e grana
    batman.add_power(Power::Technology);
    batman.add_power(Power::Fighting);

    // Vamos colocar o lex luthor, que vai dar uma "ajuda" ao batman
    let mut lexluthor: Character<Villain> = Character::new("Lex Luthor", "Alexander Luthor");
    // Ele tem alguns apetrechos também
    lexluthor.add_power(Power::Technology);

    // O batman está em desvantagem, então vamos deixar o luthor dar um golpe
    // surpresa antes da luta começar?
    info_attack(lexluthor.attack(&mut superman, Some(Power::Technology), Some(0.4)));

    // A nossa luta propriamente dita
    while batman.get_life() > 0 && superman.get_life() > 0 {
        println!();
        // Caso não passemos um poder e a força, serão decididos aleatoriamente
        info_attack(superman.attack(&mut batman, None, None));
        info_attack(batman.attack(&mut superman, None, None));
    }

    println!();

    let batman_life = batman.get_life();
    let superman_life = superman.get_life();

    if batman_life <= 0 && superman_life <= 0 {
        println!("Os dois foram derrotados ao mesmo tempo!!")
    } else if batman_life <= 0 {
        println!("Superman vence!")
    } else {
        println!("Batman vence!")
    }
}
