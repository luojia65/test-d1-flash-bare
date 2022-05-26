use crate::spi::{Instance, Spi};

pub struct SpiFlash<SPI: Instance, PINS>(Spi<SPI, PINS>);

impl<SPI: Instance, PINS> From<Spi<SPI, PINS>> for SpiFlash<SPI, PINS> {
    fn from(inner: Spi<SPI, PINS>) -> Self {
        Self(inner)
    }
}

impl<SPI: Instance, PINS> SpiFlash<SPI, PINS> {
    pub fn wait(&self) {
        while self.get_feature(0xc0) & 1 == 1 {
            core::hint::spin_loop();
        }
    }

    pub fn read_id(&self) -> u16 {
        self.wait();
        self.0.cs_low();
        let mut buf = [0x9f, 0xff];
        self.0.transfer(&mut buf[..1]);
        buf[0] = 0xff;
        self.0.transfer(&mut buf[..2]);
        self.0.cs_high();
        u16::from_be_bytes([buf[0], buf[1]])
    }
}

impl<SPI: Instance, PINS> SpiFlash<SPI, PINS> {
    fn get_feature(&self, key: u8) -> u8 {
        self.0.cs_low();
        let mut buf = [0x0f, key];
        self.0.transfer(&mut buf[..]);
        buf[0] = 0xff;
        self.0.transfer(&mut buf[..1]);
        self.0.cs_high();
        buf[0]
    }
}
