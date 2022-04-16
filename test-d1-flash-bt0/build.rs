use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

const NEZHA_FLASH: &'static [u8] = b"
OUTPUT_ARCH(riscv)
ENTRY(head_jump)
SECTIONS {
    .head.text : {
        *(.head.text)
    }
    .head.data : {
        KEEP(*(.head.data))
    }
    stext = .;
    .text : {
        KEEP(*(.text.entry))
        *(.text .text.*)
    }
    . = ALIGN(4);
    etext = .;
    srodata = .;
    .rodata : {
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
    }
    . = ALIGN(4);
    erodata = .;
    sdata = .;
    .data : {
        sidata = LOADADDR(.data);
        *(.data .data.*)
        *(.sdata .sdata.*)
    }
    . = ALIGN(4);
    edata = .;
    .bss : {
        *(.bss.uninit)
        sbss = .;
        *(.bss .bss.*)
        *(.sbss .sbss.*)
    }
    . = ALIGN(4);
    ebss = .;
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
