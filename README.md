# test-d1-flash-bare

Prerequisites: you need to install Rust and build target `riscv64imac-unknown-none-elf`.

Build project

```
cargo build
```

Dump assembly code

```
rust-objdump target\riscv64imac-unknown-none-elf\debug\test-d1-flash-bare -d
```

Get flash binary

```
rust-objcopy target\riscv64imac-unknown-none-elf\release\test-d1-flash-bare --binary-architecture=riscv64 --strip-all -O binary target\flash.bin
```
