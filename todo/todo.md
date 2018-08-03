# TO DO

## Refactor

- [ ] Remove magic numbers of opcode.
- [ ] CPU module.
- [ ] Instructions module.
- [ ] Make instruction data model instead &[u8]
- [x] Reconsider interface of instructions.

## Test & Debug

- [ ] Make integration test.

## Emulator model

## CPU Model

- [ ] fetch() returns Result.
- [ ] Make instruction decoder.
  - [x] Make decoded instruction structure.
- [x] Make instruction executor.
- [ ] Make uOP model to divide an instruction to RISC like instructions.

## Instruction

- [ ] Load
- [ ] Store

## Peripheral

- [ ] Memory mapped serial.

## Load an x86 binary

- [ ] Load Elf format.
