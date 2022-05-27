//! Log system for BT0
//!
//! todo: make module `log` crate comptaible
use crate::gpio::{
    portb::{PB8, PB9},
    Function,
};
use crate::uart::Serial;
use core::fmt;
use d1_pac::UART0;
use embedded_hal::serial::nb::Write;
use nb::block;
use spin::{Mutex, Once};

static LOGGER: Once<LockedLogger> = Once::new();

type S = Serial<UART0, (PB8<Function<6>>, PB9<Function<6>>)>;

struct LockedLogger {
    inner: Mutex<S>,
}

impl fmt::Write for S {
    #[inline]
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.as_bytes() {
            block!(self.write(*byte)).unwrap();
        }
        block!(self.flush()).unwrap();
        Ok(())
    }
}

#[inline]
pub fn set_logger(serial: S) {
    LOGGER.call_once(|| LockedLogger {
        inner: Mutex::new(serial),
    });
}

#[inline]
#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use fmt::Write;
    LOGGER.wait().inner.lock().write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::logging::_print(core::format_args!($($arg)*));
    }
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\r\n"));
    ($($arg:tt)*) => {
        $crate::logging::_print(core::format_args!($($arg)*));
        $crate::print!("\r\n");
    }
}
