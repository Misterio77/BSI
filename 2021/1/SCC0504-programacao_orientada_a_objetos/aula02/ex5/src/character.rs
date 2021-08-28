use crate::{Hero, Power, Villain};

#[derive(Default, Clone)]
/// Personagem
///
/// Um personagem fictício com nome fantasia, nome real, vida atual, e poderes (se tiver)
/// Também temos o campo ocupação, que guardará qual a ocupação do personagem (e dados associados)
/// Rust não tem herança, então usaremos generics para ter algo similar
pub struct Character<T: Occupation> {
    name: String,
    real_name: String,
    life: i32,
    powers: Vec<Power>,
    occupation: T,
}

/// Ocupação
///
/// É um traço que agrupa as diferentes ocupações de um personagem (herói, vilão...)
/// Eles tem em comum um construtor, que é usado no construtor do character
pub trait Occupation {
    fn new() -> Self;
}

// Métodos que se aplicam a qualquer ocupação
impl<T: Occupation> Character<T> {
    pub fn new(name: &str, real_name: &str) -> Self {
        Character {
            name: name.into(),
            real_name: real_name.into(),
            powers: Vec::new(),
            life: 1000,
            occupation: T::new(),
        }
    }
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    pub fn get_real_name(&self) -> String {
        self.real_name.clone()
    }
    pub fn get_life(&self) -> i32 {
        self.life
    }
    pub fn set_name(&mut self, name: &str) {
        self.name = name.into()
    }
    pub fn set_real_name(&mut self, real_name: &str) {
        self.real_name = real_name.into()
    }
    pub fn set_life(&mut self, life: i32) {
        self.life = life
    }

    /// Adiciona um novo poder ao personagem
    pub fn add_power(&mut self, power: Power) {
        self.powers.push(power)
    }
    /// Faz com que o personagem ataque outro personagem, com dada intensidade e poder
    /// Intensidade e/ou poder podem ser omitidos, sendo assim escolhidos aleatóriamente
    pub fn attack<U: Occupation>(
        &mut self,
        target: &mut Character<U>,
        power: Option<Power>,
        intensity: Option<f32>,
    ) -> Result<String, String> {
        // Verificar se foi passado um poder
        let power = match power {
            // Caso sim
            Some(power) => {
                // Caso o personagem tenha esse poder, usar esse
                if self.powers.contains(&power) {
                    Ok(power)
                // Caso não tenha, interromper ataque
                } else {
                    Err(format!("{} não tem o poder {}.", self.name, power))
                }
            }
            // Caso não
            None => {
                // Caso o personagem não tenha poder algum
                if self.powers.is_empty() {
                    Err(format!("{} não tem nenhum poder.", self.name))
                // Caso tenha
                } else {
                    // Gerar uma unsigned int aleatória, e tirar módulo com o número de poderes
                    let chosen_index = rand::random::<usize>() % self.powers.len();
                    // Usar poder escolhido
                    Ok(*self.powers.get(chosen_index).unwrap())
                }
            }
        }?;

        // Pegar intensidade passada, ou gerar uma (0.0-0.5)
        let intensity = intensity.unwrap_or_else(rand::random::<f32>) / 2.0;

        // Guardaremos o modificador de dano aqui
        let mut modifier = 1.0;

        // Iterar pelos poderes do alvo, alterando a intensidade caso haja uma resistência de
        // poderes
        for opponent_power in target.powers.iter() {
            modifier *= power.effect_against(opponent_power);
        }

        // Gerar uma boolean aleatória (50/50)
        if rand::random() {
            // Dano base (apenas baseado na intensidade passada/sorteada)
            let base_damage = intensity * 1000.0;
            // Dano com modificadores (baseado também na interação entre poderes)
            let damage = base_damage * modifier;

            target.set_life(target.get_life() - damage as i32);

            Ok(format!(
                "{} atingiu {} com '{}', inflingindo {} ({}*{:.2}) de dano!",
                self.name, target.name, power, damage as u32, base_damage as u32, modifier
            ))
        } else {
            Err(format!("{} errou seu ataque.", self.name))
        }
    }
}

// Métodos para personagens do tipo herói
impl Character<Hero> {
    pub fn get_villains_arrested(&self) -> u16 {
        self.occupation.get_villains_arrested()
    }
    pub fn add_villain_arrested(&mut self) {
        self.occupation.add_villain_arrested()
    }
}

// Métodos para personagens do tipo vilão
impl Character<Villain> {
    pub fn get_prison_years(&self) -> u16 {
        self.occupation.get_prison_years()
    }
    pub fn set_prison_years(&mut self, prison_years: u16) {
        self.occupation.set_prison_years(prison_years)
    }
}
