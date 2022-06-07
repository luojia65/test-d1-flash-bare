#![feature(naked_functions, asm_sym, asm_const)]
#![feature(default_alloc_error_handler)]
#![feature(let_chains)]
#![feature(once_cell)]
#![no_std]
#![no_main]

use core::intrinsics::transmute;
use core::ptr::{read_volatile, write_volatile};
use core::{arch::asm, panic::PanicInfo};
use d1_pac::Peripherals;
use embedded_hal::digital::{blocking::OutputPin, PinState};

#[macro_use]
mod logging;
mod ccu;
mod gpio;
mod jtag;
mod mctl;
mod spi;
mod spi_flash;
mod time;
mod uart;

use ccu::Clocks;
use gpio::Gpio;
use jtag::Jtag;
use spi::Spi;
use spi_flash::{SpiNand, SpiNor};
use time::U32Ext;
use uart::{Config, Parity, Serial, StopBits, WordLength};

const STACK_SIZE: usize = 1 * 1024; // 1KiB

const GPIO_BASE_ADDR: u32 = 0x0200_0000;
const GPIO_PC_CFG0: u32 = GPIO_BASE_ADDR + 0x0060;
const GPIO_PC_DATA: u32 = GPIO_BASE_ADDR + 0x0070;

#[link_section = ".bss.uninit"]
static mut SBI_STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

/// Jump over head data to executable code.
///
/// # Safety
///
/// Naked function.
#[naked]
#[link_section = ".head.text"]
#[export_name = "head_jump"]
pub unsafe extern "C" fn head_jump() {
    asm!(
        ".option push",
        ".option rvc",
        "c.j    0x60",
        ".option pop",
        // sym start,
        options(noreturn)
    )
}

// todo: option(noreturn) generates an extra `unimp` insn

#[repr(C)]
pub struct HeadData {
    magic: [u8; 8],
    checksum: u32,
    length: u32,
    pub_head_size: u32,
    fel_script_address: u32,
    fel_uenv_length: u32,
    dt_name_offset: u32,
    dram_size: u32,
    boot_media: u32,
    string_pool: [u32; 13],
}

const STAMP_CHECKSUM: u32 = 0x5F0A6C39;

// clobber used by KEEP(*(.head.data)) in link script
#[link_section = ".head.data"]
pub static HEAD_DATA: HeadData = HeadData {
    magic: *b"eGON.BT0",
    checksum: STAMP_CHECKSUM, // real checksum filled by blob generator
    length: 0,                // real size filled by blob generator
    pub_head_size: 0,
    fel_script_address: 0,
    fel_uenv_length: 0,
    dt_name_offset: 0,
    dram_size: 0,
    boot_media: 0,
    string_pool: [0; 13],
};

/// Jump over head data to executable code.
///
/// # Safety
///
/// Naked function.
#[naked]
#[export_name = "start"]
#[link_section = ".text.entry"]
pub unsafe extern "C" fn start() -> ! {
    asm!(
        // 1. clear cache and processor states
        "csrw   mie, zero",
        "li     t2, 0x30013",
        "csrs   0x7c2, t2", // MCOR
        // 2. initialize programming langauge runtime
        // clear bss segment
        "la     t0, sbss",
        "la     t1, ebss",
        "1:",
        "bgeu   t0, t1, 1f",
        "sd     x0, 0(t0)",
        "addi   t0, t0, 4",
        "j      1b",
        "1:",
        // does not init data segment as BT0 runs in sram
        // 3. prepare stack
        "la     sp, {stack}",
        "li     t0, {stack_size}",
        "add    sp, sp, t0",
        "la     a0, {head_data}",
        "j      {main}",
        "j      {cleanup}",
        stack      =   sym SBI_STACK,
        stack_size = const STACK_SIZE,
        head_data  =   sym HEAD_DATA,
        main       =   sym main,
        cleanup    =   sym cleanup,
        options(noreturn)
    )
}

const SDRAM_BASE: usize = 0x40000000;

extern "C" fn main() {
    // there was configure_ccu_clocks, but ROM code have already done configuring for us
    let p = Peripherals::take().unwrap();
    let clocks = Clocks {
        uart_clock: 24_000_000.hz(), // hard coded
    };
    let gpio = Gpio::new(p.GPIO);

    // configure jtag interface
    let tms = gpio.portf.pf0.into_function_4();
    let tck = gpio.portf.pf5.into_function_4();
    let tdi = gpio.portf.pf1.into_function_4();
    let tdo = gpio.portf.pf3.into_function_4();
    let _jtag = Jtag::new((tms, tck, tdi, tdo));

    // light up led
    let mut pb5 = gpio.portb.pb5.into_output();
    pb5.set_high().unwrap();
    // FIXME: This is broken. It worked before. Breakage happened in commit:
    // fd7f6b8bc2eebb25f888ded040566e591f037e9a
    let mut pc1 = gpio.portc.pc1.into_output();
    pc1.set_high().unwrap();

    // Change into output mode
    let pc_cfg0 = unsafe { read_volatile(GPIO_PC_CFG0 as *const u32) };
    let mut val = pc_cfg0 & 0xffffff0f | 0b0001 << 4;
    unsafe { write_volatile(GPIO_PC_CFG0 as *mut u32, val) };
    // Set pin to HIGH
    let pc_dat0 = unsafe { read_volatile(GPIO_PC_DATA as *const u32) };
    val = pc_dat0 | 0b1 << 1;
    unsafe { write_volatile(GPIO_PC_DATA as *mut u32, val) };

    // prepare serial port logger
    let tx = gpio.portb.pb8.into_function_6();
    let rx = gpio.portb.pb9.into_function_6();
    let config = Config {
        baudrate: 115200.bps(),
        wordlength: WordLength::Eight,
        parity: Parity::None,
        stopbits: StopBits::One,
    };
    let serial = Serial::new(p.UART0, (tx, rx), config, &clocks);
    crate::logging::set_logger(serial);

    println!("oreboot 🦀");

    let ram_size = mctl::init();
    println!("How much 🐏? {}", ram_size);

    // prepare spi interface
    let sck = gpio.portc.pc2.into_function_2();
    let scs = gpio.portc.pc3.into_function_2();
    let mosi = gpio.portc.pc4.into_function_2();
    let miso = gpio.portc.pc5.into_function_2();
    let spi = Spi::new(p.SPI0, (sck, scs, mosi, miso), &clocks);
    let mut flash = SpiNor::new(spi);

    // e.g., GigaDevice (GD) is 0xC8 and GD25Q128 is 0x4018
    // see flashrom/flashchips.h for details and more
    let id = flash.read_id();
    println!(
        " | SPI flash\n  \\ vendor ID: {:x}\n   \\ flash ID: {:x}{:x}\n",
        id[0], id[1], id[2],
    );

    // 32K, the size of boot0
    let base = 0x1 << 15;
    let size: usize = 15400;
    for i in 0..size {
        let off = base + i * 4;
        let buf = flash.copy_into([(off >> 16) as u8, (off >> 8) as u8 % 255, off as u8 % 255]);

        let addr = SDRAM_BASE + i * 4;
        let val = u32::from_le_bytes([buf[3], buf[2], buf[1], buf[0]]);
        unsafe { write_volatile(addr as *mut u32, val) };
        let rval = unsafe { read_volatile(addr as *mut u32) };

        if rval != val {
            println!("MISMATCH {addr} r{:08x} :: {:08x}", rval, val);
        }
        /*
        if i < 10 || i == 256 {
            println!("{:08x} :: {:08x}", val, rval);
        }
        */
    }

    let spi = flash.free();
    let (_spi, _pins) = spi.free();

    unsafe {
        for _ in 0..10_000_000 {
            core::arch::asm!("nop")
        }
    }
    let addr = SDRAM_BASE;
    println!("Run payload at {:#x}", addr);
    unsafe {
        let f: extern "C" fn() = transmute(addr);
        f();
    }
}

// should jump to dram but not reach there
extern "C" fn cleanup() -> ! {
    loop {
        unsafe { asm!("wfi") };
    }
}

#[cfg_attr(not(test), panic_handler)]
fn panic(info: &PanicInfo) -> ! {
    println!("{info}");
    loop {
        core::hint::spin_loop();
    }
}
