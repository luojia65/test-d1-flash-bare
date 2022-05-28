# test-d1-flash-bare

Project state: can successfully build a workable flash binary image

For all following commands: use `--release` to build in release mode, and `--verbose` to print
more output to help debugging.

## Build flash binary

Prerequisites: you need to install Rust and build target `riscv64imac-unknown-none-elf`.

Use following command:

```
cargo make
```

This command should build the project, generate a binary, fix length and calculate checksum, finally
it would produce flashable binary at `target\riscv64imac-unknown-none-elf\debug\test-d1-flash-bt0.bin`.

It defaults to debug mode. If you want to build under release mode, use `cargo make --release`.
Due to configuration on cargo workspace, the output ELF still includes debug symbols even on release mode;
however this won't affect target flash binary for those symbols will get stripped on binary generation.

## Build and burn into flash

Before burning into flash, make sure that you have correct flash type installed on board,
otherwise this command line will fail.

Use one of the following command according to boards:

```sh
cargo flash nand
# or
cargo flash nor
```

It would do all `cargo make` functions, and burn it into NAND or NOR flash on a xfel connected board.
The D1 chip must be in FEL mode before running this command.

## Debug Rust code

You need to install any RISC-V bare metal GDB executables before continue.
Either T-Head GDB or generic GDB is all supported.

It's suggested to install T-Head's DebugServer for best comptability,
OpenOCD support is work in progress.

After prepared, the following command can help you debug Rust code on a D1 chip:

```
cargo gdb
```

You may use GDB commands on GDB command line to debug Rust code in this project.

## Dump assembly code

Auto detect objdump command and print assembly code:

```
cargo asm
```

Use `cargo asm --release` to dump under release build configuration.

## Notes

1. SPI flash driver

This repository includes SPI flash driver.

The Allwinner Nezha board uses 2GB NAND flash MX35LF2GE4AD from MXIC, refer to its [website](https://www.mxic.com.tw/en-us/products/NAND-Flash/Serial-NAND-Flash/Pages/spec.aspx?p=MX35LF2GE4AD&m=Serial+NAND&n=PM2794) for user manual and example applications.

SPI flash driver from this repository would eventually support most of flash brands and
be made into a separated universal flash support crate.

2. Key code of binary generation

Source: [xtask_finalize_d1_flash_bt0](https://github.com/luojia65/test-d1-flash-bare/blob/c3f67504965384a3f79e74aa7f587e9c5e17152d/xtask/src/main.rs#L143-L178)

3. Jump over header instuction

```
0000000000000000 <head_jump>:
       0: 6f 00 40 06   j       0x64
```
