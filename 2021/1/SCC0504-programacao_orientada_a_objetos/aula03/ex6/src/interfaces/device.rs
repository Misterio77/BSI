/// Traço que representa um dispositivo, e suas funções polimórficas
pub trait Device {
    fn power_on(&mut self);
    fn power_off(&mut self);
    fn check_status(&self) -> bool;
    fn calibrate(&mut self);
}
