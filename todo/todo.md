# TO DO

## Refactor

- [ ] Move InternalException to exception.
- [x] Define opcode as enum.
- [ ] Make isa module.

## Test & Debug

- [ ] Make integration test.

## Emulator model

## CPU Model

- [ ] Make uOP model to divide an instruction to RISC like instructions.
- [x] Divide FetchedInstruction into RexPrefix, LegacyPrefix, ... and, so on.
- [x] Move RIP to FetchUnit.
- [ ] Decoder owes decode.

## Instruction

- [ ] Load
- [ ] Store

## Peripheral

- [ ] Memory mapped serial.

## Load an x86 binary

- [ ] Load Elf format.
