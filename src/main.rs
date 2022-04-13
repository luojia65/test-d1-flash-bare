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
#[link_section = ".text.entry"]
#[export_name = "_start"]
pub unsafe extern "C" fn start() -> ! {
    asm!(
        "la     sp, {stack}",
        "li     t0, {per_hart_stack_size}",
        "add    sp, sp, t0",
        "j      {main}",
        per_hart_stack_size = const PER_HART_STACK_SIZE,
        stack = sym SBI_STACK,
        main = sym main,
        options(noreturn)
    )
}

extern "C" fn main() {
    loop {}
}

#[cfg_attr(not(test), panic_handler)]
#[allow(unused)]
fn panic(info: &PanicInfo) -> ! {
    loop {}
}
