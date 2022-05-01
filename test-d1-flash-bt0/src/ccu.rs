use crate::time::Hz;
use d1_pac::ccu::RegisterBlock as CcuRb;

#[derive(Debug)]
pub struct Clocks {
    pub uart_clock: Hz, // todo: peripheral clock, and should we use them in DDR?
}

pub trait Gating {
    fn gating_pass(ccu: &CcuRb);
    fn gating_mask(ccu: &CcuRb);
}

pub trait Reset {
    fn deassert_reset(ccu: &CcuRb);
    fn assert_reset(ccu: &CcuRb);
}

impl Gating for d1_pac::UART0 {
    #[inline]
    fn gating_pass(ccu: &CcuRb) {
        ccu.uart_bgr.modify(|_, w| w.uart0_gating().pass())
    }
    #[inline]
    fn gating_mask(ccu: &CcuRb) {
        ccu.uart_bgr.modify(|_, w| w.uart0_gating().mask())
    }
}

impl Reset for d1_pac::UART0 {
    #[inline]
    fn deassert_reset(ccu: &CcuRb) {
        ccu.uart_bgr.modify(|_, w| w.uart0_rst().deassert())
    }
    #[inline]
    fn assert_reset(ccu: &CcuRb) {
        ccu.uart_bgr.modify(|_, w| w.uart0_rst().assert())
    }
}
