//! Serial Peripheral Interface (SPI)
use crate::ccu::{Clocks, Gating, Reset};
use crate::gpio::{
    portc::{PC2, PC4, PC5},
    Function,
};
use d1_pac::{spi0::RegisterBlock, CCU, SPI0};

/// D1 SPI peripheral
pub struct Spi<SPI: Instance, PINS> {
    inner: SPI,
    pins: PINS,
}

impl<SPI: Instance, PINS> Spi<SPI, PINS> {
    /// Create instance of Spi
    #[inline]
    pub fn new(spi: SPI, pins: PINS, clocks: &Clocks) -> Self
    where
        PINS: Pins<SPI>,
    {
        // 1. unwrap parameters
        // todo
        // 2. init peripheral clocks
        // note(unsafe): async read and write using ccu registers
        let ccu = unsafe { &*CCU::ptr() };
        SPI::assert_reset(ccu);
        SPI::gating_mask(ccu);
        SPI::deassert_reset(ccu);
        SPI::gating_pass(ccu);
        // 3. set interrupt configuration
        // todo
        // 4. calculate and set clock divider
        // todo
        // 5. additional configurations
        // todo
        // 6. return the instance
        Spi { inner: spi, pins }
    }
    // Close and release peripheral
    #[allow(unused)] // FIXME
    #[inline]
    pub fn free(self) -> (SPI, PINS) {
        use core::ptr;
        let inner: SPI = unsafe { ptr::read(&self.inner as *const _) };
        let pins: PINS = unsafe { ptr::read(&self.pins as *const _) };
        // self is closed via Drop trait
        (inner, pins)
    }
}

// Disable peripheral when drop; either next bootloading stage will initialize again,
// or we provide ownership of serial structure to next bootloading stage.
impl<SPI: Instance, PINS> Drop for Spi<SPI, PINS> {
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
