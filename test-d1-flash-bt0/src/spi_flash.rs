use crate::spi::{Instance, Spi};

mod consts {
    #![allow(unused_variables)]

    pub(super) const DUMMY: u8 = 0xff;
    pub(super) const CMD_GET_FEATURE: u8 = 0x0f;
    pub(super) const CMD_READ_ID: u8 = 0x9f;
    pub(super) const CMD_READ_GAGE: u8 = 0x13;
    pub(super) const CMD_READ_CACHE: u8 = 0x03;
    pub(super) const FEAT_STATUS: u8 = 0xc0;
    pub(super) const LEN_PAGE_BITS: u32 = 11;
    pub(super) const LEN_PAGE: u32 = 1 << LEN_PAGE_BITS;
    pub(super) const LEN_PAGE_MASK: u32 = LEN_PAGE - 1;
}

use consts::*;

/// NAND Flash with SPI.
pub struct SpiNand<SPI: Instance, PINS>(Spi<SPI, PINS>);

impl<SPI: Instance, PINS> From<Spi<SPI, PINS>> for SpiNand<SPI, PINS> {
    fn from(inner: Spi<SPI, PINS>) -> Self {
        Self(inner)
    }
}

impl<SPI: Instance, PINS> SpiNand<SPI, PINS> {
    /// Reads hardware ID.
    pub fn read_id(&self) -> u16 {
        let mut buf = [CMD_READ_ID, DUMMY];

        self.wait();
        self.dialog(|spi| {
            spi.transfer(&mut buf[..1]);
            spi.transfer(&mut buf[..2]);
        });

        u16::from_be_bytes([buf[0], buf[1]])
    }

    /// Copies bytes from `base` address to `buf`.
    pub fn copy_into(self, mut base: u32, mut buf: &mut [u8]) {
        println!("copy {} bytes from {base:#x}", buf.len());
        while !buf.is_empty() {
            let mut cmd = u32::to_be_bytes(base >> LEN_PAGE_BITS);
            cmd[0] = CMD_READ_GAGE;
            self.wait();
            self.dialog(|spi| spi.transfer(&mut cmd));

            let ca = base & LEN_PAGE_MASK;
            let (head, tail) = buf.split_at_mut(buf.len().min((LEN_PAGE - ca) as _));
            base += head.len() as u32;
            buf = tail;

            cmd = [CMD_READ_CACHE, (ca >> 8) as u8, ca as u8, DUMMY];
            self.wait();
            self.dialog(|spi| {
                spi.transfer(&mut cmd);
                spi.transfer(head);
            });
        }
    }
}

impl<SPI: Instance, PINS> SpiNand<SPI, PINS> {
    /// 片选限定的一次通信过程。
    #[inline]
    fn dialog(&self, f: impl FnOnce(&Spi<SPI, PINS>) -> ()) {
        self.0.cs_low();
        f(&self.0);
        self.0.cs_high();
    }

    fn get_feature(&self, key: u8) -> u8 {
        let mut buf = [CMD_GET_FEATURE, key];

        self.dialog(|spi| {
            spi.transfer(&mut buf[..2]);
            spi.transfer(&mut buf[..1]);
        });

        buf[0]
    }

    /// 等待忙状态结束。
    fn wait(&self) {
        while self.get_feature(FEAT_STATUS) & 1 == 1 {
            core::hint::spin_loop();
        }
    }
}
