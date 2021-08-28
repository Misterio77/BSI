#[derive(Copy, Clone, Debug, PartialEq)]
/// Poder
///
/// Um enum de poderes. Cada possibilidade representa um poder distinto.
pub enum Power {
    Flight,
    LaserVision,
    Pyrokinesis,
    ThermalResistance,
    Telekinesis,
    Strength,
    Durability,
    Technology,
    Fighting,
}

impl Power {
    /// Nome do super poder
    pub fn get_name(&self) -> String {
        match self {
            Power::Flight => "Voo",
            Power::LaserVision => "Visão laser",
            Power::Pyrokinesis => "Pirocinese",
            Power::ThermalResistance => "Resistência térmica",
            Power::Telekinesis => "Telecinese",
            Power::Strength => "Super força",
            Power::Durability => "Super resistência",
            Power::Technology => "Tecnologia",
            Power::Fighting => "Luta",
        }
        .into()
    }
    /// Efetividade dos poderes
    ///
    /// Retorna um escalar representando o quão efetivo aquele poder é contra um personagem que
    /// possui um outro poder. Caso seja "neutro" (maioria), é 1.0
    pub fn effect_against(&self, opponent: &Power) -> f32 {
        match (self, opponent) {
            // Quem tem resistência térmica aguenta uma visão laser
            (Power::LaserVision, Power::ThermalResistance) => 0.5,
            // Quem tem resistência térmica aguenta alguns foguinhos
            (Power::Pyrokinesis, Power::ThermalResistance) => 0.1,
            // O poder de voar pode ser usado para fugir de coisas atiradas com telecinese
            (Power::Telekinesis, Power::Flight) => 0.9,
            // Força é... Bom, forte contra tudo?
            (Power::Strength, _) => 1.7,
            // Habilidades de luta também ajudam, porém bem menos
            (Power::Fighting, _) => 1.2,
            // Tecnologia é interessante, pois quem usa normalmente consegue ir contra outros poderes
            (Power::Technology, _) => 1.3,
            // Durabilidade ajuda a proteger contra qualquer coisa
            (_, Power::Durability) => 0.6,
            // Todo o resto
            (_, _) => 1.0,
        }
    }
}

/// Formatador padrão
impl std::fmt::Display for Power {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_name())
    }
}
