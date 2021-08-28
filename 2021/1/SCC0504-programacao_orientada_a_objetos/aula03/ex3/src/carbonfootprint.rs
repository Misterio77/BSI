/// Interface da pegada de carbono
pub trait CarbonFootprint {
    /// Obter pegada de carbono em gramas de CO2-equivalente
    fn get_carbon_footprint(&self) -> u32;
}
