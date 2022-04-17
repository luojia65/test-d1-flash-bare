use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

const NEZHA_FLASH: &'static [u8] = b"
OUTPUT_ARCH(riscv)
ENTRY(head_jump)
MEMORY {
    FLASH : ORIGIN = 0x00000000, LENGTH = 256K
    SRAM : ORIGIN = 0x00020000, LENGTH = 32K
}
SECTIONS {
    .head.text : {
        *(.head.text)
    } > FLASH
    .head.data : {
        KEEP(*(.head.data))
    } > FLASH
    .text : {
        KEEP(*(.text.entry))
        *(.text .text.*)
    } > FLASH
    .rodata : ALIGN(4) {
        srodata = .;
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
        . = ALIGN(4);
        erodata = .;
    } > SRAM
    .data : ALIGN(4) {
        sdata = .;
        *(.data .data.*)
        *(.sdata .sdata.*)
        . = ALIGN(4);
        edata = .;
    } > SRAM
    sidata = LOADADDR(.data);
    .bss (NOLOAD) : ALIGN(4) {
        *(.bss.uninit)
        sbss = .;
        *(.bss .bss.*)
        *(.sbss .sbss.*)
        ebss = .;
    } > SRAM
    /DISCARD/ : {
        *(.eh_frame)
    }
}";

fn main() {
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join("link.ld"))
        .unwrap()
        .write_all(NEZHA_FLASH)
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());
}
