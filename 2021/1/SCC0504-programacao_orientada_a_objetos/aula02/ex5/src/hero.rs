use crate::Occupation;

/// Herói
pub struct Hero {
    villains_arrested: u16,
}

// Marca que herói é uma ocupação, e pode ser usado como tal em um personagem
impl Occupation for Hero {
    fn new() -> Self {
        Hero {
            villains_arrested: 0,
        }
    }
}

impl Hero {
    pub fn get_villains_arrested(&self) -> u16 {
        self.villains_arrested
    }
    pub fn add_villain_arrested(&mut self) {
        self.villains_arrested += 1;
    }
}
