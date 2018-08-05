use cpu::Cpu;
use cpu::instruction::InstructionX86_64;
use cpu::decoder::ModRm;
use byteorder::{LittleEndian, ReadBytesExt};

pub fn fetch(cpu: &Cpu, program: &[u8]) -> InstructionX86_64 {
  let rip = cpu.rip as usize;
  let opcode = program[rip] as u32;
  let mut imm = &program[rip+1..rip+5];
  let imm: u64 = imm.read_u32::<LittleEndian>().unwrap().into();
  InstructionX86_64::new(0, opcode, ModRm::new_invalid(), 0, 0, imm)
}
