use crate::CarbonFootprint;

/// Representa um edifício
pub struct Building {
    /// Consumo elétrico (kWh) do edificio
    monthly_kwh: u32,
}

/// Representa uma escola
pub struct School {
    building: Building,
    students: u32,
}

impl School {
    pub fn new(monthly_kwh: u32, students: u32) -> Self {
        School {
            students,
            building: Building { monthly_kwh },
        }
    }
}

/// Representa uma casa
pub struct House {
    building: Building,
    family_members: u32,
}

impl House {
    pub fn new(monthly_kwh: u32, family_members: u32) -> Self {
        House {
            family_members,
            building: Building { monthly_kwh },
        }
    }
}

/// Implementação do traço pegada de carbono para edifício
impl CarbonFootprint for Building {
    fn get_carbon_footprint(&self) -> u32 {
        // para produzir 1kWh de energia, se emite em torno de 453g de CO2-equivalente
        self.monthly_kwh * 453
    }
}

/// Implementação do traço pegada de carbono para escola
impl CarbonFootprint for School {
    fn get_carbon_footprint(&self) -> u32 {
        // Somar uma estimativa de alimentos (e etc) com o gasto comum do edifício (energia)
        (self.students * 100) + self.building.get_carbon_footprint()
    }
}

/// Implementação do traço pegada de carbono para casa
impl CarbonFootprint for House {
    fn get_carbon_footprint(&self) -> u32 {
        // Somar uma estimativa de alimentos (e etc) com o gasto comum do edifício (energia)
        (self.family_members * 200) + self.building.get_carbon_footprint()
    }
}
