//! Module with default functions, feel free to `use *;`

#[cfg(feature = "gpio")]
mod gpio;
#[cfg(feature = "gpio")]
pub use gpio::*;
