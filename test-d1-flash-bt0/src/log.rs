//! Log system for BT0
//!
//! todo: be rust's `log` crate comptaible
use crate::uart::Serial;
use core::fmt;
use d1_pac::UART0;
use embedded_hal::serial::nb::Write;
use nb::block;

struct SerialUart0;

impl fmt::Write for SerialUart0 {
    #[inline]
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let mut serial: Serial<UART0> = unsafe { core::mem::transmute(()) };
        for byte in s.as_bytes() {
            block!(serial.write(*byte)).unwrap();
        }
        block!(serial.flush()).unwrap();
        Ok(())
    }
}

#[inline]
#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use fmt::Write;
    SerialUart0.write_fmt(args).unwrap();
}

#[macro_export(local_inner_macros)]
macro_rules! print {
    ($($arg:tt)*) => ({
        $crate::log::_print(core::format_args!($($arg)*));
    });
}

#[macro_export(local_inner_macros)]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::log::_print(core::format_args!(core::concat!($fmt, "\r\n") $(, $($arg)+)?));
    }
}
