#![feature(naked_functions, asm_sym, asm_const)]

#![no_std]
#![no_main]
use core::arch::asm;
use core::panic::PanicInfo;

// const PER_HART_STACK_SIZE: usize = 4 * 4096; // 16KiB
// const SBI_STACK_SIZE: usize = 1 * PER_HART_STACK_SIZE; 
// #[link_section = ".bss.uninit"]
// static mut SBI_STACK: [u8; SBI_STACK_SIZE] = [0; SBI_STACK_SIZE];

#[naked]
#[link_section = ".text.entry"]
#[export_name = "_start"]
pub unsafe extern "C" fn start() -> ! {
    // asm!(
    //     "la     sp, {stack}",
    //     "li     t0, {per_hart_stack_size}",
    //     "add    sp, sp, t0",
    //     "j      {main}",
    //     per_hart_stack_size = const PER_HART_STACK_SIZE,
    //     stack = sym SBI_STACK,
    //     main = sym main,
    //     options(noreturn)
    // )
    asm!(
        "li     t0, 0x02500000",
        "li     t1, 82", // R
        "1:",
        "sb     t1, 0(t0)",
        "j      1b",
        options(noreturn)
    )
}

// extern "C" fn main() {
//     init_bss();
//     let p = d1_pac::Peripherals::take().unwrap();
//     let uart = p.UART0;
//     loop {
//         uart.thr().write(|w| unsafe { w.thr().bits(b'R') });
//         while !uart.usr.read().rfne().bit_is_set() {}
//     }
// }

#[cfg_attr(not(test), panic_handler)]
#[allow(unused)]
fn panic(info: &PanicInfo) -> ! {
    loop {}
}

// fn init_bss() {
//     extern "C" {
//         static mut ebss: u32;
//         static mut sbss: u32;
//         static mut edata: u32;
//         static mut sdata: u32;
//         static sidata: u32;
//     }
//     unsafe {
//         r0::zero_bss(&mut sbss, &mut ebss);
//         r0::init_data(&mut sdata, &mut edata, &sidata);
//     }
// }
