//! General Purpose Input-Output
use core::marker::PhantomData;
use core::ptr::{read_volatile, write_volatile};
use d1_pac::GPIO;

/// Individual GPIO pin
pub struct Pin<const P: char, const N: u8, MODE = Disabled> {
    _mode: PhantomData<MODE>,
}

impl<const P: char, const N: u8, MODE> Pin<P, N, MODE> {
    /// Disables the pin
    #[inline]
    pub fn into_disabled(self) -> Pin<P, N, Disabled> {
        self.into_mode()
    }
    /// Configures the pin to operate as an input pin
    #[inline]
    pub fn into_input(self) -> Pin<P, N, Input> {
        self.into_mode()
    }
    /// Configures the pin to operate as an output pin
    #[inline]
    pub fn into_output(self) -> Pin<P, N, Output> {
        self.into_mode()
    }
    /// Configures the pin to operate as an external interrupt
    #[inline]
    pub fn into_external_interrupt(self) -> Pin<P, N, Eint> {
        self.into_mode()
    }
}
impl<const P: char, const N: u8, MODE> Pin<P, N, MODE> {
    #[inline(always)]
    const fn new() -> Self {
        Self { _mode: PhantomData }
    }
    #[inline(always)]
    fn into_mode<M: PinMode>(mut self) -> Pin<P, N, M> {
        self.set_mode::<M>();
        Pin::new()
    }
    // this function violates type parameter rule; caller must ensure
    // a correct type parameter change after calling this function.
    #[inline(always)]
    fn set_mode<M: PinMode>(&mut self) {
        let (cfg_reg_offset, cfg_reg_idx) = self.cfg_reg();
        let cfg_reg_ptr = unsafe { (GPIO::ptr() as *mut u32).add(cfg_reg_offset) };
        let mut new_cfg = unsafe { read_volatile(cfg_reg_ptr) };
        new_cfg &= !(0xF << cfg_reg_idx);
        new_cfg |= (M::VALUE as u32) << cfg_reg_idx;
        unsafe { write_volatile(cfg_reg_ptr, new_cfg) };
    }
    #[inline(always)]
    const fn cfg_reg(&self) -> (usize, usize) {
        let port_offset_in_u32 = (P as usize - b'A' as usize) * 0xC;
        let (cfg_reg_offset, cfg_idx) = match N {
            0..=7 => (0x0, N << 2),
            8..=15 => (0x1, (N - 8) << 2),
            16..=23 => (0x2, (N - 16) << 2),
            _ => unreachable!(),
        };
        (port_offset_in_u32 + cfg_reg_offset, cfg_idx as usize)
    }
}

macro_rules! define_gpio {
    ($(
        $PortX: ident, $portx: ident, $P: expr, [
            $($PXi: ident:
                ($pxi: ident, $i: expr, $mode: ty),
                ($doc_name: expr, $pinout: expr),
                ($f2: tt, $f3: tt, $f4: tt, $f5: tt, $f6: tt, $f7: tt, $f8: tt),
            )+
        ]
    )+) => {
/// Gpio peripheral
pub struct Gpio {
    $(pub $portx: $portx::$PortX,)+
    _inner: GPIO,
}

impl Gpio {
    pub fn new(inner: GPIO) -> Self {
        // todo: ensure APB0 clock okay
        Self {
            $($portx: $portx::$PortX {
                $($pxi: Pin::new(),)+
            },)+
            _inner: inner,
        }
    }
}
$(#[allow(unused)] pub mod $portx {
    use super::*;
    $(
    #[doc = concat!("Pin ",$doc_name," at ",$pinout)]
    pub type $PXi<MODE = $mode> = Pin<$P, $i, MODE>;
    )+
    #[doc = concat!("GPIO port ",$P)]
    pub struct $PortX {
        $(pub $pxi: $PXi,)+
    }
    $(impl $PXi {
        define_gpio!(@func $PXi, into_function_2, 2, $f2);
        define_gpio!(@func $PXi, into_function_3, 3, $f3);
        define_gpio!(@func $PXi, into_function_4, 4, $f4);
        define_gpio!(@func $PXi, into_function_5, 5, $f5);
        define_gpio!(@func $PXi, into_function_6, 6, $f6);
        define_gpio!(@func $PXi, into_function_7, 7, $f7);
        define_gpio!(@func $PXi, into_function_8, 8, $f8);
    })+
})+
    };
    (@func $PXi: ident, $into_fn: ident, $fi: expr, x) => {}; // generate nothing
    (@func $PXi: ident, $into_fn: ident, $fi: expr, $doc: expr) => {
        #[doc = concat!("Configures the pin to operate as alternate function ",$fi,": ",$doc)]
        #[inline] pub fn $into_fn(self) -> $PXi<Function<$fi>> {
            self.into_mode()
        }
    };
}

define_gpio! {
    PortB, portb, 'B', [
        PB0: (pb0, 0, Disabled), ("PB0", "1"), ("PWM3", "IR-TX", "TWI2-SCK", "SPI1-WP/DBI-TE", "UART0-TX", "UART2-TX", "OWA-OUT"),
        PB1: (pb1, 1, Disabled), ("PB1", "1"), ("PWM4", "I2S2-DOUT3", "TWI2-SDA", "I2S2-DIN3", "UART0-RX", "UART2-RX", "IR-RX"),
        PB5: (pb5, 5, Disabled), ("PB5", "1"), ("LCD0-D9", "I2S2-BCLK", "TWI1-SDA", "PWM0", "LCD0-D21", "UART5-RX", x),
    ]
    PortC, portc, 'C', [
        PC1: (pc1, 1, Disabled), ("PC1", "1"), ("UART2-RX", "TWI2-SDA", x, x, x, x, x),
    ]
    PortF, portf, 'F', [
        PF0: (pf0, 0, Disabled), ("PF0", "1"), ("SDC0-D1", "JTAG-MS", "R-JTAG-MS", "I2S2-DOUT1", "I2S2-DIN0", x, x),
        PF1: (pf1, 1, Disabled), ("PF1", "1"), ("SDC0-D0", "JTAG-DI", "R-JTAG-DI", "I2S2-DOUT0", "I2S2-DIN1", x, x),
        PF3: (pf3, 3, Disabled), ("PF3", "1"), ("SDC0-CMD", "JTAG-DO", "R-JTAG-DO", "I2S2-BCLK", x, x, x),
        PF5: (pf5, 5, Disabled), ("PF5", "1"), ("SDC0-D2", "JTAG-CK", "R-JTAG-CK", "I2S2-LRCK", x, x, x),
    ]
}

/// Input mode (type state)
pub struct Input;
/// Output mode (type state)
pub struct Output;
/// Function modes (type state)
///
/// N should be in 2..=8.
pub struct Function<const N: u8>;
/// External interrupt mode (type state)
pub struct Eint;
/// Disabled mode (type state)
pub struct Disabled;

pub trait PinMode {
    const VALUE: u8;
}

impl PinMode for Input {
    const VALUE: u8 = 1;
}

impl PinMode for Output {
    const VALUE: u8 = 2;
}

impl<const N: u8> PinMode for Function<N> {
    const VALUE: u8 = N;
}

impl PinMode for Eint {
    const VALUE: u8 = 14;
}

impl PinMode for Disabled {
    const VALUE: u8 = 15;
}
