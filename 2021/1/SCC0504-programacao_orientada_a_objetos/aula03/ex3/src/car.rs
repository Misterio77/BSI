use crate::CarbonFootprint;

/// Representa um carro (movido à gasolina)
pub struct Car {
    /// Kilometragem/mes do carro
    monthly_km: u32,
    /// Tamanho do carro
    size: CarSize,
}

/// Tamanhos possíveis de um carro
pub enum CarSize {
    Large,
    Medium,
    Small,
}

impl Car {
    pub fn new(monthly_km: u32, size: CarSize) -> Car {
        Car { monthly_km, size }
    }
}

impl CarbonFootprint for Car {
    fn get_carbon_footprint(&self) -> u32 {
        // Dados obtidos em carbonfootprint.com
        let efficiency = match self.size {
            CarSize::Large => 278,
            CarSize::Medium => 186,
            CarSize::Small => 148,
        };

        self.monthly_km * efficiency
    }
}
