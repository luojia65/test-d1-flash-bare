# test-d1-flash-bare

Project state: can't get serial output yet, working on image formats.

## Build flash binary

Prerequisites: you need to install Rust and build target `riscv64imac-unknown-none-elf`.

Use following command:

```
cargo make
```

Your flash binary should be at: `target\riscv64imac-unknown-none-elf\debug\test-d1-flash-bt0.bin`.

It defaults to debug mode. If you want to build under release mode, use `cargo make --release`.

## Dump assembly code

```
cargo asm
```

Use `cargo asm --release` to dump under release build configuration.

## Notes

Jump over header instuction

```
0000000000000000 <head_jump>:
       0: 6f 00 40 06   j       0x64
```
