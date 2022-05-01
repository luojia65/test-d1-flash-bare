#![feature(naked_functions, asm_sym, asm_const)]
#![no_std]
#![no_main]
// mod hal;
#[macro_use]
mod ccu;
mod gpio;
mod log;
mod time;
mod uart;
use crate::ccu::Clocks;
use crate::time::U32Ext;

// use crate::hal::{pac_encoding::UART0_BASE, Serial};
use core::arch::asm;
use core::panic::PanicInfo;
use d1_pac::Peripherals;

const CCU_BASE: usize = 0x0200_1000;

const APB0_CLK: usize = CCU_BASE + 0x0520; // 0x0200_1520
const APB1_CLK: usize = CCU_BASE + 0x0524; // 0x0200_1524

const GPIO_BASE_ADDR: u32 = 0x0200_0000;
const GPIO_PB_CFG0: u32 = GPIO_BASE_ADDR + 0x0030;
const GPIO_PB_CFG1: u32 = GPIO_BASE_ADDR + 0x0034;
const GPIO_PB_DATA: u32 = GPIO_BASE_ADDR + 0x0040;
const GPIO_PB_DRV0: u32 = GPIO_BASE_ADDR + 0x0044;
const GPIO_PB_DRV1: u32 = GPIO_BASE_ADDR + 0x0048;
const GPIO_PB_PULL: u32 = GPIO_BASE_ADDR + 0x0054;
const GPIO_PC_CFG0: u32 = GPIO_BASE_ADDR + 0x0060;
const GPIO_PC_DATA: u32 = GPIO_BASE_ADDR + 0x0070;

const PER_HART_STACK_SIZE: usize = 4 * 1024; // 4KiB
const SBI_STACK_SIZE: usize = 1 * PER_HART_STACK_SIZE;
#[link_section = ".bss.uninit"]
static mut SBI_STACK: [u8; SBI_STACK_SIZE] = [0; SBI_STACK_SIZE];

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
    init_bss();
    light_up_led();
    // there was configure_ccu_clocks, but ROM code have already done configuring for us
    // configure_gpio_pf_port();
    // configure_gpio_uart_peripheral();
    use gpio::Gpio;
    use uart::{Config, Parity, Serial, StopBits, WordLength};
    let p = Peripherals::take().unwrap();
    let gpio = Gpio::new(p.GPIO);
    let mut pb5 = gpio.portb.pb5.into_output();
    let mut pc1 = gpio.portc.pc1.into_output();
    // fixme: these are risc-v jtag pins, remove #[allow(unused)] in the future
    #[allow(unused)]
    let pf0 = gpio.portf.pf0.into_function_4();
    #[allow(unused)]
    let pf1 = gpio.portf.pf1.into_function_4();
    #[allow(unused)]
    let pf3 = gpio.portf.pf3.into_function_4();
    #[allow(unused)]
    let pf5 = gpio.portf.pf5.into_function_4();
    let clocks = Clocks {
        uart_clock: 24_000_000.hz(), // hard coded
    };
    let config = Config {
        baudrate: 115200.bps(),
        wordlength: WordLength::Eight,
        parity: Parity::None,
        stopbits: StopBits::One,
    };
    let serial = Serial::new(p.UART0, config, &clocks); // fixme: don't drop this struct
    println!("OREBOOT");
    println!("Test");
    loop {
        println!("Rust🦀");
        for _ in 0..100000 {}
    }
}

use core::ptr::{read_volatile, write_volatile};

fn light_up_led() {
    // GPIO port C pin 1 (LED on Lichee RV module)
    // Change into output mode
    let pc_cfg0 = unsafe { read_volatile(GPIO_PC_CFG0 as *const u32) };
    let mut val = pc_cfg0 & 0xffffff0f | 0b0001 << 4;
    unsafe { write_volatile(GPIO_PC_CFG0 as *mut u32, val) };
    // Set pin to HIGH
    let pc_dat0 = unsafe { read_volatile(GPIO_PC_DATA as *const u32) };
    val = pc_dat0 | 0b1 << 1;
    unsafe { write_volatile(GPIO_PC_DATA as *mut u32, val) };

    // GPIO port B pin 5 (available on Nezha)
    let pb_cfg0 = unsafe { read_volatile(GPIO_PB_CFG0 as *const u32) };
    let mut val = pb_cfg0 & 0xff0fffff | 0b0001 << 20;
    unsafe { write_volatile(GPIO_PB_CFG0 as *mut u32, val) };
    // Set pin to HIGH
    let pc_dat0 = unsafe { read_volatile(GPIO_PB_DATA as *const u32) };
    val = pc_dat0 | 0b1 << 5;
    unsafe { write_volatile(GPIO_PB_DATA as *mut u32, val) };
}

// fn configure_gpio_pf_port() {
//     let pf_cfg0 = unsafe { read_volatile(0x0200_00f0 as *const u32) };
//     // PF5 Select: R-JRAG-CK
//     // PF3 Select: R-JRAG-DO
//     // PF1 Select: R-JRAG-DI
//     // PF0 Select: R-JRAG-MS
//     let new_value = (pf_cfg0 & 0xff0f0f00) | 0x00404044;
//     unsafe { write_volatile(0x0200_00f0 as *mut u32, new_value) };
// }

// fn configure_gpio_uart_peripheral() {
//     // PB1 Select: UART0-RX
//     // PB0 Select: UART0-TX
//     let pb_cfg1 = unsafe { read_volatile(GPIO_PB_CFG1 as *const u32) };
//     let new_value = (pb_cfg1 & 0xffffff00) | 0b0110 | 0b0110 << 4;
//     unsafe { write_volatile(GPIO_PB_CFG1 as *mut u32, new_value) };

//     // pull-ups
//     let mut val = unsafe { read_volatile(GPIO_PB_PULL as *mut u32) };
//     val = val | 1 << 16 | 1 << 18;
//     unsafe { write_volatile(GPIO_PB_PULL as *mut u32, val) };

//     // PB8 + PB9 drive level 3
//     unsafe { write_volatile(GPIO_PB_DRV1 as *mut u32, 0x0001_1133) };
// }

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
