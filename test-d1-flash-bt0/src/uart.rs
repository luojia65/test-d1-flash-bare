//! Uart peripheral on BT0 stage
use crate::time::Bps;
use core::ops::Deref;
use d1_pac::uart::{
    lcr::{DLS_A, EPS_A, PEN_A, STOP_A},
    RegisterBlock,
};

/// D1 serial peripheral
///
/// Parameter PA: Physical Address
#[derive(Debug)]
pub struct Serial<UART> {
    inner: UART,
}

#[allow(unused)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Config {
    pub baudrate: Bps,
    pub wordlength: WordLength,
    pub parity: Parity,
    pub stopbits: StopBits,
}

#[allow(unused)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WordLength {
    Five,
    Six,
    Seven,
    Eight,
}

#[allow(unused)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Parity {
    None,
    Odd,
    Even,
}

/// Stop Bit configuration parameter for serial.
#[allow(unused)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum StopBits {
    /// 1 stop bit
    One,
    /// 2 stop bits, or 1.5 bits when WordLength is Five
    Two,
}

impl<UART> Serial<UART> {
    /// Create instance of Uart
    pub fn new(uart: UART, config: impl Into<Config>) -> Self
    where
        UART: Deref<Target = RegisterBlock>,
    {
        // 1. unwrap parameters
        let Config {
            baudrate,
            wordlength,
            parity,
            stopbits,
        } = config.into();
        let bps = baudrate.0;
        // 2. init peripheral clocks
        // todo
        // 3. calculate and setbaudrate
        // todo: calculate with peripheral clock
        let uart_clk = (24000000 + 8 * bps) / (16 * bps);
        let (dlh, dll) = ((uart_clk >> 8) as u8, (uart_clk & 0xff) as u8);
        uart.lcr.modify(|_, w| w.dlab().divisor_latch());
        uart.dlh().write(|w| unsafe { w.dlh().bits(dlh) });
        uart.dll().write(|w| unsafe { w.dll().bits(dll) });
        uart.lcr.modify(|_, w| w.dlab().rx_buffer());
        // 4. additional configurations
        let dls = match wordlength {
            WordLength::Five => DLS_A::FIVE,
            WordLength::Six => DLS_A::SIX,
            WordLength::Seven => DLS_A::SEVEN,
            WordLength::Eight => DLS_A::EIGHT,
        };
        let stop = match stopbits {
            StopBits::One => STOP_A::ONE,
            StopBits::Two => STOP_A::TWO,
        };
        let (pen, eps) = match parity {
            Parity::None => (PEN_A::DISABLED, EPS_A::ODD /* chosen randomly */),
            Parity::Odd => (PEN_A::ENABLED, EPS_A::ODD),
            Parity::Even => (PEN_A::ENABLED, EPS_A::EVEN),
        };
        uart.lcr.modify(
            |_, w| {
                w.dls()
                    .variant(dls)
                    .stop()
                    .variant(stop)
                    .pen()
                    .variant(pen)
                    .eps()
                    .variant(eps)
                    .bc()
                    .clear_bit()
            }, // todo: break control
        );
        // todo: pin configuration
        uart.mcr.write(|w| {
            w.dtr()
                .deasserted()
                .rts()
                .deasserted()
                .loop_()
                .normal()
                .afce()
                .disabled()
                .function()
                .uart()
        });
        // todo: fifo configuration
        uart.fcr().write(|w| {
            w.fifoe()
                .set_bit()
                .rfifor()
                .set_bit()
                .xfifor()
                .set_bit()
                .dmam()
                .mode_0()
                .tft()
                .half_full()
                .rt()
                .two_less_than_full()
        });
        // 5. return the instance
        Serial { inner: uart }
    }
}

/// Error types that may happen when serial transfer
#[derive(Debug)]
pub struct Error {
    kind: embedded_hal::serial::ErrorKind,
}

impl embedded_hal::serial::Error for Error {
    fn kind(&self) -> embedded_hal::serial::ErrorKind {
        self.kind
    }
}

impl<UART> embedded_hal::serial::ErrorType for Serial<UART>
where
    UART: Deref<Target = RegisterBlock>,
{
    type Error = Error;
}

impl<UART> embedded_hal::serial::nb::Write<u8> for Serial<UART>
where
    UART: Deref<Target = RegisterBlock>,
{
    fn write(&mut self, word: u8) -> nb::Result<(), Self::Error> {
        if self.inner.usr.read().tfnf().is_full() {
            return Err(nb::Error::WouldBlock);
        }
        self.inner.thr().write(|w| unsafe { w.thr().bits(word) });
        Ok(())
    }
    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        if self.inner.usr.read().tfe().is_empty() {
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }
}
