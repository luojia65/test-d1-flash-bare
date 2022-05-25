//! Serial Peripheral Interface (SPI)
use core::marker::PhantomData;

use crate::ccu::{Clocks, Gating, Reset};
use crate::gpio::{
    portc::{PC2, PC4, PC5},
    Function,
};
use d1_pac::{
    spi0::{
        spi_gcr::{EN_A, MODE_A},
        spi_tcr::{CPHA_A, CPOL_A, SPOL_A, SS_LEVEL_A, SS_OWNER_A, SS_SEL_A},
        RegisterBlock,
    },
    CCU, SPI0,
};

/// D1 SPI peripheral
pub struct Spi<SPI: Instance, PINS> {
    inner: SPI,
    pins: PINS,
    stub: Stub<SPI>,
}

/// Allows free for Spi safely.
struct Stub<SPI: Instance>(PhantomData<SPI>);

impl<SPI: Instance, PINS> Spi<SPI, PINS> {
    /// Create instance of Spi
    #[inline]
    pub fn new(spi: SPI, pins: PINS, _clocks: &Clocks) -> Self
    where
        PINS: Pins<SPI>,
    {
        // 1. unwrap parameters
        // todo
        // 2. init peripheral clocks
        // note(unsafe): async read and write using ccu registers
        let ccu = unsafe { &*CCU::ptr() };
        SPI::deassert_reset(ccu);
        SPI::gating_pass(ccu);
        // 不必使用 CCU 重置，因为 SPI 有自己的软件重置
        spi.spi_gcr.write(|w| w.srst().set_bit());
        // 3. set interrupt configuration
        // on BT0 stage we disable all spi interrupts, by setting the gcr.sret
        // 4. calculate and set clock divider
        // todo
        // 5. additional configurations
        #[rustfmt::skip]
        spi.spi_gcr.write(|w| w
            .mode().variant(MODE_A::MASTER)
            .en()  .variant(EN_A::ENABLE)
        );
        #[rustfmt::skip]
        spi.spi_tcr.write(|w| w
            .ss_level().variant(SS_LEVEL_A::HIGH)
            .ss_owner().variant(SS_OWNER_A::SOFTWARE)
            .ss_sel()  .variant(SS_SEL_A::SS0)
            .spol()    .variant(SPOL_A::LOW)
            .cpol()    .variant(CPOL_A::LOW)
            .cpha()    .variant(CPHA_A::P1)
        );
        // 6. return the instance
        Spi {
            inner: spi,
            pins,
            stub: Stub(PhantomData),
        }
    }

    /// 拉低片选使能读写
    pub fn cs_low(&self) {
        self.inner.spi_tcr.modify(|r, w| {
            unsafe { w.bits(r.bits()) }
                .ss_level()
                .variant(SS_LEVEL_A::LOW)
        })
    }

    /// 拉高片选结束读写
    pub fn cs_high(&self) {
        self.inner.spi_tcr.modify(|r, w| {
            unsafe { w.bits(r.bits()) }
                .ss_level()
                .variant(SS_LEVEL_A::HIGH)
        })
    }

    /// 读一字节
    pub fn read_byte(&self) -> u8 {
        todo!()
    }

    /// 写一字节
    pub fn write_byte(&self) -> u8 {
        todo!()
    }

    /// 插入空周期等待数据
    pub fn insert_dummy_cycle(&self) -> u8 {
        todo!()
    }

    /// Close and release peripheral
    #[inline]
    pub fn free(self) -> (SPI, PINS) {
        let Self {
            inner,
            pins,
            stub: _, // spi is closed via Drop trait of stub
        } = self;
        (inner, pins)
    }
}

// Disable peripheral when drop; either next bootloading stage will initialize again,
// or we provide ownership of serial structure to next bootloading stage.
impl<SPI: Instance> Drop for Stub<SPI> {
    #[inline]
    fn drop(&mut self) {
        let ccu = unsafe { &*CCU::ptr() };
        SPI::assert_reset(ccu);
        SPI::gating_mask(ccu);
    }
}

pub trait Instance: Gating + Reset + core::ops::Deref<Target = RegisterBlock> {}

impl Instance for d1_pac::SPI0 {}

pub trait Pins<SPI> {}

// parameter order: sck, miso, mosi

impl Pins<SPI0> for (PC2<Function<2>>, PC5<Function<2>>, PC4<Function<2>>) {}
