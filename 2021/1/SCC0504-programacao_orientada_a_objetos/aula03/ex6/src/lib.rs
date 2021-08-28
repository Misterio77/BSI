pub mod interfaces;
pub mod hpprinter;
pub mod nvidiagpu;

pub use interfaces::{Device, VideoDevice};
pub use hpprinter::HpPrinter;
pub use nvidiagpu::NvidiaGpu;
