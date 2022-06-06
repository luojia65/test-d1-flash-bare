use crate::spi::{Instance, Spi};

mod consts {
    #![allow(unused_variables)]

    pub(super) const CMD_WRSR: u8 = 0x01;
    pub(super) const CMD_PROG: u8 = 0x02;
    pub(super) const CMD_READ: u8 = 0x03;
    pub(super) const CMD_READ_SR: u8 = 0x05;
    pub(super) const CMD_WRITE_ENABLE: u8 = 0x06;
    pub(super) const CMD_E4K: u8 = 0x20;
    pub(super) const CMD_E32K: u8 = 0x52;
    pub(super) const CMD_SFDP: u8 = 0x5a;
    pub(super) const CMD_READ_ID: u8 = 0x9f;
    pub(super) const CMD_ENTER_4B: u8 = 0xb7;
    pub(super) const CMD_E64K: u8 = 0xd8;
    pub(super) const CMD_EXIT_4B: u8 = 0xe9;
}

use consts::*;

/// NOR Flash with SPI.
pub struct SpiNor<SPI: Instance, PINS>(Spi<SPI, PINS>);

impl<SPI: Instance, PINS> SpiNor<SPI, PINS> {
    #[inline]
    pub fn new(inner: Spi<SPI, PINS>) -> Self {
        Self(inner)
    }
    #[inline]
    pub fn free(self) -> Spi<SPI, PINS> {
        self.0
    }
}

impl<SPI: Instance, PINS> SpiNor<SPI, PINS> {
    /// Reads hardware ID.
    #[inline]
    pub fn read_id(&self) -> [u8; 3] {
        let mut buf = [0u8; 3];

        self.0.transfer([CMD_READ_ID], 1, &mut buf);

        buf
    }
}
