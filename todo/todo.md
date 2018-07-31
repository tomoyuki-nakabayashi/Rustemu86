# TO DO

## Refactor

- [ ] Remove magic numbers of opcode.
- [ ] CPU module.
- [ ] Instruction module.
- [x] Fix emulation mode using generics.

## Test & Debug

- [ ] Interactive mode.
- [ ] Make integration test.

## Emulator model

- [ ] Prepare Emulator structure to manage all of the emulation.

## CPU Model

- [ ] fetch() returns Result.
- [ ] Make instruction decoder.
  - [ ] Make decoded instruction structure.
- [ ] Make instruction executor.
- [ ] Make uOP model to divide an instruction to RISC like instructions.

## Instruction

- [ ] Load
- [ ] Store
- [ ] Branch

## Peripheral

- [ ] Memory mapped serial.

## Load an x86 binary

- [ ] Load Elf format.
