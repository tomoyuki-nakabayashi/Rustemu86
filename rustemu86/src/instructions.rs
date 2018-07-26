use byteorder::{ReadBytesExt, LittleEndian};
use rustemu86::Rustemu86;
use register_file::RegisterFile;
use register_file::GeneralRegisterId;
use register_file::GeneralRegisterId::{RegRax, RegRcx, RegRdx, RegRbx};

pub fn mov_imm(mut cpu: Rustemu86, inst: &[u8]) {
  let dest = GeneralRegisterId::from_u8(inst[0] & 0b00000111).unwrap();
  let mut imm = &inst[2..];
  let imm: u64 = imm.read_u32::<LittleEndian>().unwrap().into();

  cpu.rf.write64(dest, imm);
}