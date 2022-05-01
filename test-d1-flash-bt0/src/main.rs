#![feature(naked_functions, asm_sym, asm_const)]
#![no_std]
#![no_main]
// mod hal;
#[macro_use]
mod ccu;
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
const CCU_UART_BGR: usize = CCU_BASE + 0x090C;

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

const UART0_BASE: u32 = 0x0250_0000;
const UART0_THR: u32 = UART0_BASE;
const UART0_DLH: u32 = UART0_BASE + 0x0004;
const UART0_FCR: u32 = UART0_BASE + 0x0008;
const UART0_LCR: u32 = UART0_BASE + 0x000C;
const UART0_MCR: u32 = UART0_BASE + 0x0010;
const UART0_LSR: u32 = UART0_BASE + 0x0014;

const UART_BAUD: u32 = 115200;

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
    configure_gpio_pf_port();
    configure_uart_peripheral();
    configure_ccu_clocks();
    println!("Println!");
    uart0_putchar_ore(b'O');
    uart0_putchar_ore(b'R');
    uart0_putchar_ore(b'E');
    uart0_putchar_ore(b'B');
    uart0_putchar_ore(b'O');
    uart0_putchar_ore(b'O');
    uart0_putchar_ore(b'T');
    uart0_putchar_ore(b'\r');
    uart0_putchar_ore(b'\n');

    uart0_putchar(b'T');
    uart0_putchar(b'e');
    uart0_putchar(b's');
    uart0_putchar(b't');
    uart0_putchar(b'\r');
    uart0_putchar(b'\n');
    loop {
        uart0_putchar(b'R');
        uart0_putchar(b'u');
        uart0_putchar(b's');
        uart0_putchar(b't');
        uart0_putchar(b' ');
        uart0_putchar(0xF0);
        uart0_putchar(0x9F);
        uart0_putchar(0xA6);
        uart0_putchar(0x80);
        uart0_putchar(b'\r');
        uart0_putchar(b'\n');
        for _ in 0..100000 {}
    }

    // let p = d1_pac::Peripherals::take().unwrap();
    // let uart = p.UART0;
    // loop {
    //     uart.thr().write(|w| unsafe { w.thr().bits(b'R') });
    //     while !uart.usr.read().rfne().bit_is_set() {}
    // }

    /*
    unsafe { asm!("la a0, {}", sym HEAD_DATA) };
    init_bss();
    let p = Peripherals::take().unwrap();
    let s = Serial::new(UART0_BASE);
    // let s = Serial::new(p.UART0).unwrap();
    writeln!(s, "Nezha"); // TODO

    /*
    let ubgr = ccu::UART_BGR;
    let c = CCU::new(p.UART0).unwrap();
    // CCU init
    // reset
    // p.CCU.UART_BGR.modify();

    // ccu::uart_bgr::W::uart0_rst
    ccu::uart_bgr::.clear_bit(0);
    for i in 1..100 {}
    p.CCU.bgr.modify(CCU_UART_BGR::UART0_RST.val(1));
    */

    // gate
    /*
    p.CCU.bgr.modify(CCU_UART_BGR::UART0_GATING.val(0));
    for i in 1..100 {}
    p.CCU.bgr.modify(CCU_UART_BGR::UART0_GATING.val(1));
    */

    let uart = p.UART0;
    loop {
        uart.thr().write(|w| unsafe { w.thr().bits(b'R') });
        while !uart.usr.read().rfne().bit_is_set() {}
    }
    */
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

fn configure_gpio_pf_port() {
    let pf_cfg0 = unsafe { read_volatile(0x0200_00f0 as *const u32) };
    // PF5 Select: R-JRAG-CK
    // PF3 Select: R-JRAG-DO
    // PF1 Select: R-JRAG-DI
    // PF0 Select: R-JRAG-MS
    let new_value = (pf_cfg0 & 0xff0f0f00) | 0x00404044;
    unsafe { write_volatile(0x0200_00f0 as *mut u32, new_value) };
}

const PARITY: u32 = 0;
const STOP: u32 = 0;
const DLEN: u32 = 3;
const UART_SET: u32 = ((PARITY & 0x03) << 3) | ((STOP & 0x01) << 2) | (DLEN & 0x03);

fn configure_uart_like_xboot() {
    // disable interrupts
    let mut dlh = unsafe { read_volatile(UART0_DLH as *const u32) };
    dlh = dlh & 0b1111_1101;
    unsafe { write_volatile(UART0_DLH as *mut u32, dlh) };
    // enable FIFO
    let mut fcr = unsafe { read_volatile(UART0_FCR as *const u32) };
    fcr = fcr | 1;
    unsafe { write_volatile(UART0_FCR as *mut u32, fcr) };

    // from xboot
    unsafe { write_volatile(UART0_MCR as *mut u32, 0x3) };

    //   // DLAB
    //   let mut lcr = unsafe { read_volatile(UART0_LCR as *const u32) };
    //   // lcr = lcr | 1 << 7;
    //   lcr = lcr | 0x80;
    //   unsafe { write_volatile(UART0_LCR as *mut u32, lcr) };
    //   // config clock rate, etc
    //   let mut dlh = unsafe { read_volatile(UART0_DLH as *mut u32) };
    //   dlh = dlh & 0xffffff00;
    //   unsafe { write_volatile(UART0_DLH as *mut u32, dlh) };
    //   let mut dll = unsafe { read_volatile(UART0_THR as *mut u32) };
    //   dll = dll & 0xffffff00 | 13;
    //   unsafe { write_volatile(UART0_THR as *mut u32, dll) };
    //   // DLAB
    //   let mut lcr = unsafe { read_volatile(UART0_LCR as *const u32) };
    //   lcr = lcr & 0x0111_1111;
    //   unsafe { write_volatile(UART0_LCR as *mut u32, lcr) };

    /*  ---------  */
    let mut lcr = unsafe { read_volatile(UART0_LCR as *mut u32) };
    lcr = lcr | 0x80;
    unsafe { write_volatile(UART0_LCR as *mut u32, lcr) };

    let uart_clk = (24000000 + 8 * UART_BAUD) / (16 * UART_BAUD);
    let dlh = uart_clk >> 8;
    unsafe { write_volatile(UART0_DLH as *mut u32, dlh) };
    let dll = uart_clk & 0xff;
    // THR is also RBR and DLL
    unsafe { write_volatile(UART0_THR as *mut u32, dll) };

    let mut lcr = unsafe { read_volatile(UART0_LCR as *mut u32) };
    lcr = lcr & 0b01111111; // ~0x80
    unsafe { write_volatile(UART0_LCR as *mut u32, lcr) };

    lcr = UART_SET;
    unsafe { write_volatile(UART0_LCR as *mut u32, lcr) };
    /*  ---------  */

    // unsafe { write_volatile(UART0_FCR as *mut u32, 1) };
}

fn configure_uart_like_oreboot() {
    // asd
    // unsafe { write_volatile(UART0_MCR as *mut u32, 3) };

    // disable interrupts
    let mut dlh = unsafe { read_volatile(UART0_DLH as *const u32) };
    dlh = dlh & 0b1111_1101;
    unsafe { write_volatile(UART0_DLH as *mut u32, dlh) };
    // enable FIFO
    let mut fcr = unsafe { read_volatile(UART0_FCR as *const u32) };
    fcr = fcr | 1;
    unsafe { write_volatile(UART0_FCR as *mut u32, fcr) };

    // disable tx
    let tx = unsafe { read_volatile(UART0_THR as *mut u32) };
    unsafe { write_volatile(UART0_THR as *mut u32, tx | 1) };

    // DLAB
    let mut lcr = unsafe { read_volatile(UART0_LCR as *const u32) };
    // lcr = lcr | 1 << 7;
    lcr = lcr | 0x80;
    unsafe { write_volatile(UART0_LCR as *mut u32, lcr) };
    // config clock rate, etc
    let mut dlh = unsafe { read_volatile(UART0_DLH as *mut u32) };
    dlh = dlh & 0xffffff00;
    unsafe { write_volatile(UART0_DLH as *mut u32, dlh) };
    let mut dll = unsafe { read_volatile(UART0_THR as *mut u32) };
    dll = dll & 0xffffff00 | 13;
    unsafe { write_volatile(UART0_THR as *mut u32, dll) };
    // DLAB
    let mut lcr = unsafe { read_volatile(UART0_LCR as *const u32) };
    lcr = lcr & 0x0111_1111;
    unsafe { write_volatile(UART0_LCR as *mut u32, lcr) };

    // enable tx
    let mut tx = unsafe { read_volatile(UART0_THR as *mut u32) };
    tx = tx & 0xfffffffe;
    unsafe { write_volatile(UART0_THR as *mut u32, tx) };

    unsafe { write_volatile(UART0_FCR as *mut u32, 0b0111) };
}

fn configure_uart_peripheral() {
    // PB1 Select: UART0-RX
    // PB0 Select: UART0-TX
    let pb_cfg1 = unsafe { read_volatile(GPIO_PB_CFG1 as *const u32) };
    let new_value = (pb_cfg1 & 0xffffff00) | 0b0110 | 0b0110 << 4;
    unsafe { write_volatile(GPIO_PB_CFG1 as *mut u32, new_value) };

    // pull-ups
    let mut val = unsafe { read_volatile(GPIO_PB_PULL as *mut u32) };
    val = val | 1 << 16 | 1 << 18;
    unsafe { write_volatile(GPIO_PB_PULL as *mut u32, val) };

    // PB8 + PB9 drive level 3
    unsafe { write_volatile(GPIO_PB_DRV1 as *mut u32, 0x0001_1133) };

    // UART4_GATING: Pass
    // UART0_GATING: Pass
    /* UART bus gating reset */
    // reset
    let ccu_uart_bgr = unsafe { read_volatile(CCU_UART_BGR as *const u32) };
    let mut new_value = ccu_uart_bgr & 0b1111_1111_1111_1110_1111_1111_1111_1111;
    unsafe { write_volatile(CCU_UART_BGR as *mut u32, new_value) };
    for _ in 1..100 {}
    let ccu_uart_bgr = unsafe { read_volatile(CCU_UART_BGR as *const u32) };
    new_value = ccu_uart_bgr | 1 << 16;
    unsafe { write_volatile(CCU_UART_BGR as *mut u32, new_value) };
    // gating
    let ccu_uart_bgr = unsafe { read_volatile(CCU_UART_BGR as *const u32) };
    let mut new_value = ccu_uart_bgr & 0b1111_1111_1111_1111_1111_1111_1111_1110;
    unsafe { write_volatile(CCU_UART_BGR as *mut u32, new_value) };
    for _ in 1..100 {}
    let ccu_uart_bgr = unsafe { read_volatile(CCU_UART_BGR as *const u32) };
    new_value = ccu_uart_bgr | 0x1;
    unsafe { write_volatile(CCU_UART_BGR as *mut u32, new_value) };

    // configure_uart_like_xboot();
    // configure_uart_like_oreboot();
    configure_uart_by_peripheral();
}

fn configure_uart_by_peripheral() {
    use uart::{Config, Parity, Serial, StopBits, WordLength};
    let p = Peripherals::take().unwrap();
    let clocks = Clocks {
        uart_clock: 24_000_000.hz(), // hard coded
    };
    let config = Config {
        baudrate: 115200.bps(),
        wordlength: WordLength::Eight,
        parity: Parity::None,
        stopbits: StopBits::One,
    };
    let serial = Serial::new(p.UART0, config, &clocks);
}

fn configure_ccu_clocks() {
    let pll_cpu_ctrl = unsafe { read_volatile(0x0200_1000 as *const u32) };
    // 11010111 11111100 00000000 11111100
    // 11001000 00000000 00101001 00000000
    // PLL CPU control
    // Enable: 1
    // LDO Enable: 1
    // Lock enable: 0
    // PLL output gate: enable
    // PLL N: 42
    // PLL Unlock level: 21-29 clock cycles
    // PLL Lock level: 24-26 clock cycles
    // PLL M: 1
    let new_value = (pll_cpu_ctrl & 0xD7FC00FC) | 0xC8002900;
    unsafe { write_volatile(0x0200_1000 as *mut u32, new_value) };
    // APB0 clock configuration; APB0_CLK = source frequency / (N * M)
    // Clock source: PLL_PERI(1x)
    // Divide factor N: 2 (1 << _0x1_)
    // Divide factor M: 3 (_0x2_ + 1)
    unsafe { write_volatile(APB0_CLK as *mut u32, 0x0300_0102) };
    // unsafe { write_volatile(APB1_CLK as *mut u32, 0x0300_0102) };
    // RISC-V Clock
    // Clock source: PLL_CPU
    // Divide factor N: 2
    // Divide factor M: 1
    unsafe { write_volatile(0x0200_1d00 as *mut u32, 0x0500_0100) };
}

fn uart0_putchar_ore(a: u8) {
    loop {
        let uart0_lsr = unsafe { read_volatile(UART0_LSR as *const u32) };
        if uart0_lsr & (1 << 6) != 0 {
            // TX FIFO is empty
            break;
        }
    }
    // write to uart transmitting holding register
    unsafe { write_volatile(UART0_THR as *mut u32, a as u32) };
}

fn uart0_putchar(a: u8) {
    loop {
        let uart0_status = unsafe { read_volatile(0x0250_007C as *const u32) };
        if uart0_status & 0x2 != 0 {
            // TX FIFO is empty
            break;
        }
    }
    // write to uart transmitting holding register
    unsafe { write_volatile(UART0_THR as *mut u32, a as u32) };
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
