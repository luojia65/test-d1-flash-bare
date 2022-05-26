use crate::spi::{Instance, Spi};

pub struct SpiFlash<SPI: Instance, PINS>(Spi<SPI, PINS>);

impl<SPI: Instance, PINS> From<Spi<SPI, PINS>> for SpiFlash<SPI, PINS> {
    fn from(inner: Spi<SPI, PINS>) -> Self {
        Self(inner)
    }
}

impl<SPI: Instance, PINS> SpiFlash<SPI, PINS> {
    pub fn check_status(&self) -> u8 {
        self.0.cs_low();
        let mut buf = [0x0f, 0xc0];
        self.0.transfer(&mut buf[..]);
        buf[0] = 0xff;
        self.0.transfer(&mut buf[..1]);
        self.0.cs_high();
        buf[0]
    }
}
