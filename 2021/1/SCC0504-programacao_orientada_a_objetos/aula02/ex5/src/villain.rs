use crate::Occupation;
/// Vilão
pub struct Villain {
    prison_years: u16,
}

// Marca que vilão é uma ocupação, e pode ser usado como tal em um personagem
impl Occupation for Villain {
    fn new() -> Self {
        Villain { prison_years: 0 }
    }
}

impl Villain {
    pub fn get_prison_years(&self) -> u16 {
        self.prison_years
    }
    pub fn set_prison_years(&mut self, prison_years: u16) {
        self.prison_years = prison_years;
    }
}
