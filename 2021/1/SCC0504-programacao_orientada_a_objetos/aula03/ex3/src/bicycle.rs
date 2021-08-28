use crate::CarbonFootprint;

/// Representa uma bicicleta
pub struct Bicycle {
    /// Kilometragem/mes da bicicleta
    monthly_km: u32,
}

impl Bicycle {
    pub fn new(monthly_km: u32) -> Bicycle {
        Bicycle { monthly_km }
    }
}

impl CarbonFootprint for Bicycle {
    fn get_carbon_footprint(&self) -> u32 {
        // Pedalar 1km (incluindo pegada de carbono da comida do ciclista) tem a média de uns 20g
        // de CO2-equivalente
        self.monthly_km * 20
    }
}
