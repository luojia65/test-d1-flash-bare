#![feature(naked_functions, asm_sym, asm_const)]
#![feature(default_alloc_error_handler)]
#![no_std]
#![no_main]
extern crate alloc;
#[macro_use]
mod ccu;
mod gpio;
mod jtag;
mod log;
mod time;
mod uart;
use crate::ccu::Clocks;
use crate::time::U32Ext;
use embedded_hal::digital::blocking::OutputPin;
use buddy_system_allocator::LockedHeap;
use d1_pac::Peripherals;
use core::arch::asm;
use core::panic::PanicInfo;

const PER_HART_STACK_SIZE: usize = 1 * 1024; // 1KiB
const SBI_STACK_SIZE: usize = 1 * PER_HART_STACK_SIZE;
#[link_section = ".bss.uninit"]
static mut SBI_STACK: [u8; SBI_STACK_SIZE] = [0; SBI_STACK_SIZE];

const SBI_HEAP_SIZE: usize = 2 * 1024; // 2KiB
#[link_section = ".bss.uninit"]
static mut HEAP_SPACE: [u8; SBI_HEAP_SIZE] = [0; SBI_HEAP_SIZE];
#[global_allocator]
static SBI_HEAP: LockedHeap<32> = LockedHeap::empty();

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

#[naked]
#[export_name = "start"]
#[link_section = ".text.entry"]
pub unsafe extern "C" fn start() -> ! {
    asm!(
        "csrw   mie, zero",
        "li     t2, 0x30013",
        "csrs   0x7c2, t2", // MCOR
        "la     sp, {stack}",
        "li     t0, {per_hart_stack_size}",
        "add    sp, sp, t0",
        "la     a0, {head_data}",
        "j      {main}",
        stack = sym SBI_STACK,
        per_hart_stack_size = const PER_HART_STACK_SIZE,
        head_data = sym HEAD_DATA,
        main = sym main,
        options(noreturn)
    )
}

extern "C" fn main() {
    // init program and peripherals
    init_bss();
    unsafe { SBI_HEAP.lock().init(&HEAP_SPACE as *const _ as usize, SBI_HEAP_SIZE) };
    // there was configure_ccu_clocks, but ROM code have already done configuring for us
    use gpio::Gpio;
    use jtag::Jtag;
    use uart::{Config, Parity, Serial, StopBits, WordLength};
    let p = Peripherals::take().unwrap();
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
    let mut pc1 = gpio.portc.pc1.into_output();
    pc1.set_high().unwrap();

    // prepare serial port
    let tx = gpio.portb.pb8.into_function_6();
    let rx = gpio.portb.pb9.into_function_6();
    let clocks = Clocks {
        uart_clock: 24_000_000.hz(), // hard coded
    };
    let config = Config {
        baudrate: 115200.bps(),
        wordlength: WordLength::Eight,
        parity: Parity::None,
        stopbits: StopBits::One,
    };
    // fixme: don't drop this struct
    let serial = Serial::new(p.UART0, (tx, rx), config, &clocks);

    println!("OREBOOT");
    println!("Test");
    loop {
        println!("Rust🦀");
        for _ in 0..100000 {}
    }
}

#[cfg_attr(not(test), panic_handler)]
#[allow(unused)]
fn panic(info: &PanicInfo) -> ! {
    loop {}
}

#[inline]
fn init_bss() {
    extern "C" {
        static mut ebss: u32;
        static mut sbss: u32;
        static mut edata: u32;
        static mut sdata: u32;
        static sidata: u32;
    }
    unsafe {
        r0::zero_bss(&mut sbss, &mut ebss);
        r0::init_data(&mut sdata, &mut edata, &sidata);
    }
}
