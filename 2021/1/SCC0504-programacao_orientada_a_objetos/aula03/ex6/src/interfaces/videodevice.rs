use crate::Device;

/// Representa um dispositivo de vídeo.
///
/// Qualquer estrutura que implementa VideoDevice deve, obrigatoriamente, implementar
/// Device também.
pub trait VideoDevice: Device {
    fn render(&self);
}
