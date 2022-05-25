use crate::spi::{Instance, Spi};

pub struct SpiFlash<SPI: Instance, PINS>(Spi<SPI, PINS>);

impl<SPI: Instance, PINS> From<Spi<SPI, PINS>> for SpiFlash<SPI, PINS> {
    fn from(inner: Spi<SPI, PINS>) -> Self {
        Self(inner)
    }
}
