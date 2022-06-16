#![feature(naked_functions, asm_sym, asm_const)]
#![feature(default_alloc_error_handler)]
#![no_std]
#![no_main]

use core::intrinsics::transmute;
use core::ptr::{read_volatile, write_volatile};
use core::{arch::asm, panic::PanicInfo};
use d1_pac::{Peripherals, SPI0};
use embedded_hal::digital::blocking::OutputPin;

#[macro_use]
mod logging;
mod ccu;
mod flash;
mod gpio;
mod jtag;
mod mctl;
mod spi;
mod time;
mod uart;

use ccu::Clocks;
#[cfg(feature = "nand")]
use flash::SpiNand;
#[cfg(feature = "nor")]
use flash::SpiNor;
use gpio::Gpio;
use jtag::Jtag;
use mctl::RAM_BASE;
use spi::Spi;
use time::U32Ext;
use uart::{Config, Parity, Serial, StopBits, WordLength};

const STACK_SIZE: usize = 1 * 1024; // 1KiB

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

const ORE_ADDR: usize = RAM_BASE;
const ORE_SIZE: usize = 0x1 << 16; // 64K
const DTF_SIZE: usize = 0x1 << 16; // 64K
const LIN_ADDR: usize = ORE_ADDR + 0x20_0000; // Linux expects to be at 0x4020_0000
const LIN_SIZE: usize = 0x00ee_0000;
const DTB_ADDR: usize = LIN_ADDR + 0x100_0000; // dtb must be 2MB aligned and behind Linux
const DTB_SIZE: usize = 0x0000_e000;
const DTB_END1: usize = DTB_ADDR + 0x6000;
const DTB_END2: usize = DTB_ADDR + 0x6010;
const DTB_END3: usize = DTB_ADDR + 0x6020;

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
        // function `main` returns address of next stage,
        // it drops all peripherals it holds when goes out of scope
        // now, jump to dram code
        "j      {finish}",
        stack      =   sym SBI_STACK,
        stack_size = const STACK_SIZE,
        head_data  =   sym HEAD_DATA,
        main       =   sym main,
        finish     =   sym finish,
        options(noreturn)
    )
}

// map well-known addresses to expected values
fn addr_to_exp(a: usize) -> u32 {
    match a {
        ORE_ADDR => 0x30401073, // oreboot (mie)
        LIN_ADDR => 0x0000a0d1, // Linux
        DTB_ADDR => 0xedfe0dd0, // dtb (d00dfeed)
        DTB_END1 => 0x646e2c76, // dtb near end
        DTB_END2 => 0x6d6d0079, // dtb near end
        DTB_END3 => 0x756f6474, // dtb near end
        _ => 0x0,
    }
}

/// helper for debugging
fn dump(addr: usize) {
    let val = unsafe { read_volatile(addr as *mut u32) };
    println!("DUMP {:08x}:{:x}", val, addr);
}

/// helper for debugging
fn check_val(addr: usize, val: u32) {
    let rval = unsafe { read_volatile(addr as *mut u32) };
    if rval != val {
        println!("MISMATCH {:x} r{:08x} :: {:08x}", addr, rval, val);
    }

    match addr {
        ORE_ADDR | LIN_ADDR | DTB_ADDR | DTB_END1 | DTB_END2 | DTB_END3 => {
            let exp = addr_to_exp(addr);
            let ok = if rval == exp as u32 { "OK" } else { "NO" };
            println!("{:08x}@{:08x} - {:08x}? {}", rval, addr, exp, ok);
        }
        _ => {}
    };
}

#[cfg(feature = "nor")]
fn load(
    skip: usize,
    base: usize,
    size: usize,
    f: &mut SpiNor<
        SPI0,
        (
            gpio::Pin<'C', 2, gpio::Function<2>>,
            gpio::Pin<'C', 3, gpio::Function<2>>,
            gpio::Pin<'C', 4, gpio::Function<2>>,
            gpio::Pin<'C', 5, gpio::Function<2>>,
        ),
    >,
) {
    let chunks = 16;
    for i in 0..size / chunks {
        let off = skip + i * 4 * chunks;
        let buf = f.copy_into([(off >> 16) as u8, (off >> 8) as u8, off as u8]);

        for j in 0..chunks {
            let jw = 4 * j;
            let addr = base + i * 4 * chunks + jw;
            // transform bytes from slice to u32
            let val = u32::from_le_bytes(buf[jw..(jw + 4)].try_into().unwrap());
            unsafe { write_volatile(addr as *mut u32, val) };
            // enable for debugging
            if false {
                check_val(addr, val);
            }
        }
        // progress indicator each 2MB
        if (off - skip) % 0x10_0000 == 0 {
            print!("➡️");
            // for debugging
            // println!("a {:x} o {:08x} v {:08x}", addr, off, val);
        }
    }
    println!(".");
}

extern "C" fn main() -> usize {
    // there was configure_ccu_clocks, but ROM code have already done configuring for us
    let p = Peripherals::take().unwrap();
    // rom provided clock frequency, it's hard coded in bt0 stage
    let clocks = Clocks {
        psi: 600_000_000.hz(),
        apb1: 24_000_000.hz(),
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
    let mut pc1 = gpio.portc.pc1.into_output();
    pc1.set_high().unwrap();

    /*
    // blinky
    for _ in 0..2 {
        for _ in 0..1000_0000 {
            core::hint::spin_loop();
        }
        pc1.set_low().unwrap();
        for _ in 0..1000_0000 {
            core::hint::spin_loop();
        }
        pc1.set_high().unwrap();
    }
    */

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
    println!("{}M 🐏", ram_size);

    // prepare spi interface to use in flash
    let sck = gpio.portc.pc2.into_function_2();
    let scs = gpio.portc.pc3.into_function_2();
    let mosi = gpio.portc.pc4.into_function_2();
    let miso = gpio.portc.pc5.into_function_2();
    let spi = Spi::new(
        p.SPI0,
        (sck, scs, mosi, miso),
        spi::MODE_3,
        24_000_000.hz(),
        &clocks,
    );

    let payload_addr = RAM_BASE;

    #[cfg(feature = "nor")]
    {
        let mut flash = SpiNor::new(spi);

        // e.g., GigaDevice (GD) is 0xC8 and GD25Q128 is 0x4018
        // see flashrom/flashchips.h for details and more
        let id = flash.read_id();
        println!("NOR flash: {:x}/{:x}{:x}", id[0], id[1], id[2],);

        println!("Load... 💾");
        // println!("oreboot");
        let skip = 0x1 << 15; // 32K, the size of boot0
        let size = ORE_SIZE >> 2;
        load(skip, ORE_ADDR, size, &mut flash);

        // println!("LinuxBoot");
        // 32K + oreboot + dtfs + 4K, see oreboot dtfs
        let skip = skip + ORE_SIZE + DTF_SIZE + 0x1000;
        let size = (LIN_SIZE) >> 2;
        load(skip, LIN_ADDR, size, &mut flash);

        // println!("dtb");
        // 32K + oreboot + dtfs + 4K + kernel
        let skip = skip + LIN_SIZE;
        let size = (DTB_SIZE) >> 2;
        load(skip, DTB_ADDR, size, &mut flash);

        let _ = flash.free().free();
    }

    #[cfg(feature = "nand")]
    {
        let mut flash = SpiNand::new(spi);
        println!("NAND flash: {:x}", flash.read_id());
        let mut page = [0u8; 256];
        flash.copy_into(0, &mut page);
        let _ = flash.free().free();
    }

    // debug helpers
    /*
    dump(LIN_ADDR + 0x00186f00);
    dump(LIN_ADDR + 0x00186f04);
    dump(LIN_ADDR + 0x00186f08);
    dump(LIN_ADDR + 0x00286f00);
    dump(LIN_ADDR + 0x00286f04);
    dump(LIN_ADDR + 0x00286f08);
    dump(LIN_ADDR + 0x00386f00);
    dump(LIN_ADDR + 0x00386f04);
    dump(LIN_ADDR + 0x00386f08);
    dump(LIN_ADDR + 0x00486f00);
    dump(LIN_ADDR + 0x00486f04);
    dump(LIN_ADDR + 0x00486f08);

    // println!("final checks - are those right?");
    // oreboot first instruction
    check_val(ORE_ADDR, addr_to_exp(ORE_ADDR));
    // Linux first instruction
    check_val(LIN_ADDR, addr_to_exp(LIN_ADDR));
    // dtb begin
    check_val(DTB_ADDR, addr_to_exp(DTB_ADDR));
    // dtb end
    check_val(DTB_END1, addr_to_exp(DTB_END1));
    check_val(DTB_END2, addr_to_exp(DTB_END2));
    check_val(DTB_END3, addr_to_exp(DTB_END3));
    */

    for _ in 0..1000_0000 {
        core::hint::spin_loop();
    }

    println!("Run payload at 0x{:x}", RAM_BASE);
    unsafe {
        let f: unsafe extern "C" fn() = transmute(payload_addr);
        f();
    }

    // returns an address of dram payload; now cpu would jump to this address
    // and run code inside
    RAM_BASE
}

// jump to dram
extern "C" fn finish(address: extern "C" fn()) -> ! {
    unsafe { asm!("jr {}", in(reg) address) }
    loop {
        unsafe { asm!("wfi") }
    }
}

#[cfg_attr(not(test), panic_handler)]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        println!(
            "panic in file '{}' line {}",
            location.file(),
            location.line(),
        );
    } else {
        println!("panic at unknown location");
    };
    loop {
        core::hint::spin_loop();
    }
}
