use crate::spi::{Instance, Spi};

mod consts {
    #![allow(unused_variables)]

    pub(super) const DUMMY: u8 = 0xff;
    pub(super) const CMD_GET_FEATURE: u8 = 0x0f;
    pub(super) const CMD_READ_ID: u8 = 0x9f;
    pub(super) const CMD_READ_GAGE: u8 = 0x13;
    pub(super) const FEAT_STATUS: u8 = 0xc0;
    pub(super) const LEN_PAGE_BITS: u32 = 11;
    pub(super) const LEN_PAGE_MASK: u32 = (1 << LEN_PAGE_BITS) - 1;
}

use consts::*;

pub struct SpiFlash<SPI: Instance, PINS>(Spi<SPI, PINS>);

pub struct SpiFlashReader<SPI: Instance, PINS> {
    inner: SpiFlash<SPI, PINS>,
    offset: usize,
}

impl<SPI: Instance, PINS> From<Spi<SPI, PINS>> for SpiFlash<SPI, PINS> {
    fn from(inner: Spi<SPI, PINS>) -> Self {
        Self(inner)
    }
}

impl<SPI: Instance, PINS> SpiFlash<SPI, PINS> {
    /// 读硬件 ID。
    pub fn read_id(&self) -> u16 {
        let mut buf = [CMD_READ_ID, DUMMY];

        self.wait();
        self.0.cs_low();
        self.0.transfer(&mut buf[..1]);
        buf[0] = DUMMY;
        self.0.transfer(&mut buf[..2]);
        self.0.cs_high();

        u16::from_be_bytes([buf[0], buf[1]])
    }

    /// 准备从 `base` 地址开始顺序读取。
    pub fn read_from(self, base: u32) -> SpiFlashReader<SPI, PINS> {
        let mut buf = u32::to_be_bytes(base >> LEN_PAGE_BITS);
        buf[0] = CMD_READ_GAGE;

        self.wait();
        self.0.cs_low();
        self.0.transfer(&mut buf[..4]);
        self.0.cs_high();

        SpiFlashReader {
            inner: self,
            offset: (base & LEN_PAGE_MASK) as _,
        }
    }
}

impl<SPI: Instance, PINS> SpiFlash<SPI, PINS> {
    fn get_feature(&self, key: u8) -> u8 {
        self.0.cs_low();
        let mut buf = [CMD_GET_FEATURE, key];
        self.0.transfer(&mut buf[..2]);
        buf[0] = DUMMY;
        self.0.transfer(&mut buf[..1]);
        self.0.cs_high();
        buf[0]
    }

    fn wait(&self) {
        while self.get_feature(FEAT_STATUS) & 1 == 1 {
            core::hint::spin_loop();
        }
    }
}

impl<SPI: Instance, PINS> SpiFlashReader<SPI, PINS> {
    pub fn read(self, buf: &mut [u8]) -> SpiFlash<SPI, PINS> {
        self.inner.wait();
        println!("read from flash by spi flash reader");
        loop {
            core::hint::spin_loop();
        }
    }
}
