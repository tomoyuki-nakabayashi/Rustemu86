# Rustemu86

[![Build Status](https://travis-ci.org/tomoyuki-nakabayashi/Rustemu86.svg?branch=master)](https://travis-ci.org/tomoyuki-nakabayashi/Rustemu86)
![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)

An emulator written in Rust.

Rustemu86 supports the following instruction set architectures (but incompletely!):
    - x86
    - x86_64
    - RISC-V (rv32i)

## How to execute

### RISC-V

I prepare some test binaries, i.e., rv32-iu test suits of riscv-tests and Zephyr's hello sample.

To boot Zephyr OS, execute the commands:

```
cd arch/riscv
cargo test boot_zephyr -- --nocapture
```

Then you can see the boot message of Zephyr OS!

```
***** Booting Zephyr OS zephyr-v1.13.0-3321-g7f956a9 *****
Hello World! qemu_riscv32
```

### x86_64

Prepare an x86 binary file before executing the following commands.

```
cd rustemu86
cargo run BINARY_FILE
```
