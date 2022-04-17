#![feature(naked_functions, asm_sym, asm_const)]
#![no_std]
#![no_main]
use core::arch::asm;
use core::panic::PanicInfo;

const PER_HART_STACK_SIZE: usize = 4 * 4096; // 16KiB
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
    length: 0, // real size filled by blob generator
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
        "la     sp, {stack}",
        "li     t0, {per_hart_stack_size}",
        "add    sp, sp, t0",
        "j      {main}",
        stack = sym SBI_STACK,
        per_hart_stack_size = const PER_HART_STACK_SIZE,
        main = sym main,
        options(noreturn)
    )
    // asm!(
    //     // open uart clock gate and reset gate
    //     "li     t0, 0x0200190C",
    //     "li     t1, (1 << 0) | (1 << 16)",
    //     "sw     t1, 0(t0)",
    //     // set gpio B8,B9 to uart0, B9 drive level 3
    //     "li     t0, 0x02000000",
    //     "lw     t1, 0x34(t0)",
    //     "ori    t1, t1, 0b01100110",
    //     "sw     t1, 0x34(t0)",
    //     "lw     t1, 0x48(t0)",
    //     "ori    t1, t1, 0b00110000",
    //     "sw     t1, 0x48(t0)",
    //     // write one char to uart
    //     "li     t0, 0x02500000",
    //     "li     t1, 82", // R
    //     "1:",
    //     "sb     t1, 0(t0)",
    //     "j      1b", // todo: remove when there's uart output
    //     "j      {}",
    //     sym main,
    //     options(noreturn)
    // )
}

extern "C" fn main() {
    init_bss();
    configure_gpio_pf_port();
    // let p = d1_pac::Peripherals::take().unwrap();
    // let uart = p.UART0;
    // loop {
    //     uart.thr().write(|w| unsafe { w.thr().bits(b'R') });
    //     while !uart.usr.read().rfne().bit_is_set() {}
    // }
}

use core::ptr::{read_volatile, write_volatile};

fn configure_gpio_pf_port() {
    let pf_cfg0 = unsafe { read_volatile(0x0200_00f0 as *const u32) };
    // PF5 Select: R-JRAG-CK
    // PF3 Select: R-JRAG-DO
    // PF1 Select: R-JRAG-DI
    // PF0 Select: R-JRAG-MS
    let new_value = (pf_cfg0 & 0xff0f0f00) | 0x00404044;
    unsafe { write_volatile(0x0200_00f0 as *mut u32, new_value) };
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
