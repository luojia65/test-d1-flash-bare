//! Log system for BT0
//!
//! todo: make module `log` crate comptaible
use crate::uart;
use alloc::boxed::Box;
use core::fmt;
use core::lazy::OnceCell;
use embedded_hal::serial::nb::Write;
use nb::block;
use spin::Mutex;

static LOGGER: LockedLogger = LockedLogger {
    inner: Mutex::new(EmbeddedHalLogger {
        write: OnceCell::new(),
    }),
};

struct LockedLogger {
    inner: Mutex<EmbeddedHalLogger>,
}

struct EmbeddedHalLogger {
    write: OnceCell<Box<dyn Write<u8, Error = uart::Error> + Send>>,
}

impl fmt::Write for EmbeddedHalLogger {
    #[inline]
    fn write_str(&mut self, s: &str) -> fmt::Result {
        if let Some(serial) = self.write.get_mut() {
            for byte in s.as_bytes() {
                block!(serial.write(*byte)).unwrap();
            }
            block!(serial.flush()).unwrap();
        }
        Ok(())
    }
}

#[inline]
pub fn set_logger<T: Write<u8, Error = uart::Error> + Send + 'static>(logger: T) {
    let lock = LOGGER.inner.lock();
    lock.write.set(Box::new(logger)).ok();
    drop(lock);
}

#[inline]
#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use fmt::Write;
    LOGGER.inner.lock().write_fmt(args).unwrap();
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
