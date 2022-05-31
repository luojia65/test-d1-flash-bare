use crate::spi::{Instance, Spi};

mod consts {
    #![allow(unused_variables)]

    pub(super) const CMD_GET_FEATURE: u8 = 0x0f;
    pub(super) const CMD_READ_ID: u8 = 0x9f;
    pub(super) const CMD_READ_GAGE: u8 = 0x13;
    pub(super) const CMD_READ_CACHE_X2: u8 = 0x3b;
    pub(super) const FEAT_STATUS: u8 = 0xc0;
    pub(super) const LEN_PAGE_BITS: u32 = 11;
    pub(super) const LEN_PAGE: u32 = 1 << LEN_PAGE_BITS;
    pub(super) const LEN_PAGE_MASK: u32 = LEN_PAGE - 1;
}

use consts::*;

/// NAND Flash with SPI.
pub struct SpiNand<SPI: Instance, PINS>(Spi<SPI, PINS>);

impl<SPI: Instance, PINS> SpiNand<SPI, PINS> {
    #[inline]
    pub fn new(inner: Spi<SPI, PINS>) -> Self {
        Self(inner)
    }
    #[inline]
    pub fn free(self) -> Spi<SPI, PINS> {
        self.0
    }
}

impl<SPI: Instance, PINS> SpiNand<SPI, PINS> {
    /// Reads hardware ID.
    #[inline]
    pub fn read_id(&self) -> [u8; 3] {
        let mut buf = [0u8; 3];

        self.wait();
        self.0.transfer([CMD_READ_ID], 1, &mut buf, false);

        buf
    }

    /// Copies bytes from `base` address to `buf`.
    #[inline]
    pub fn copy_into(&mut self, mut base: u32, mut buf: &mut [u8]) {
        while !buf.is_empty() {
            // 在每页执行读操作
            let mut cmd = u32::to_be_bytes(base >> LEN_PAGE_BITS);
            cmd[0] = CMD_READ_GAGE;
            self.wait();
            self.0.transfer(cmd, 0, [], false);
            // 计算页内偏移
            let ca = base & LEN_PAGE_MASK;
            let (head, tail) = buf.split_at_mut(buf.len().min((LEN_PAGE - ca) as _));
            base += head.len() as u32;
            buf = tail;
            // 读入
            let mut cmd = u32::to_be_bytes(ca);
            cmd[1] = CMD_READ_CACHE_X2;
            self.wait();
            self.0.transfer(&cmd[1..], 1, head, true);
        }
    }
}

impl<SPI: Instance, PINS> SpiNand<SPI, PINS> {
    #[inline]
    fn get_feature(&self, key: u8) -> u8 {
        let mut feature = 0u8;

        self.0.transfer(
            [CMD_GET_FEATURE, key],
            0,
            core::slice::from_mut(&mut feature),
            false,
        );

        feature
    }

    /// 等待忙状态结束。
    #[inline]
    fn wait(&self) {
        while self.get_feature(FEAT_STATUS) & 1 == 1 {
            core::hint::spin_loop();
        }
    }
}
