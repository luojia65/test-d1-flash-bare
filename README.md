# test-d1-flash-bare

Prerequisites: you need to install Rust and build target `riscv64imac-unknown-none-elf`.

Build flash binary

```
cargo make
```

Your flash binary should be at: `target\riscv64imac-unknown-none-elf\debug\test-d1-flash-bt0.bin`.

Dump assembly code

```
rust-objdump target\riscv64imac-unknown-none-elf\debug\test-d1-flash-bt0 -d
```

Jump over header instuction

```
0000000000000000 <head_jump>:
       0: 6f 00 40 06   j       0x64
```
