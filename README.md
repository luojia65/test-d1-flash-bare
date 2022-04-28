# test-d1-flash-bare

Project state: can successfully build a workable flash binary image

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

Use following command:

```
cargo flash
```

It would do all `cargo make` functions, and burn it into NAND flash on a xfel connected board.
The D1 chip must be in FEL mode before running this command.

_TODO: support NOR flash_

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

1. Key code of binary generation

Source: [xtask_finalize_d1_flash_bt0](https://github.com/luojia65/test-d1-flash-bare/blob/c3f67504965384a3f79e74aa7f587e9c5e17152d/xtask/src/main.rs#L143-L178)

2. Jump over header instuction

```
0000000000000000 <head_jump>:
       0: 6f 00 40 06   j       0x64
```
