use byteorder::{ReadBytesExt, LittleEndian};
use register_file::RegisterFile;
use register_file::Reg64Id;

pub fn mov_imm64(rf: &mut RegisterFile, inst: &[u8]) {
  let dest = Reg64Id::from_u8(inst[0] - 0xb8).unwrap();
  let mut imm = &inst[2..];
  let imm: u64 = imm.read_u32::<LittleEndian>().unwrap().into();

  rf.write64(dest, imm);
}