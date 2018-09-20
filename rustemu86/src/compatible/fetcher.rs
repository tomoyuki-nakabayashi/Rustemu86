use compatible::{Result, CompatibleException};
use compatible::isa::opcode::OpcodeCompat;
use num::FromPrimitive;

pub(crate) struct FetchedInst {
    opcode: OpcodeCompat,
    inst_bytes: u64,
}

impl FetchedInst {
    pub(crate) fn get_opcode(&self) -> OpcodeCompat {
        self.opcode
    }

    pub(super) fn increment_ip(&self, ip: u64) -> u64 {
        ip + self.inst_bytes
    }
}

pub(super) fn fetch(program: &[u8]) -> Result<FetchedInst> {
    if let Some(opcode) = OpcodeCompat::from_u8(program[0]) {
        match opcode {
            OpcodeCompat::Hlt => {
                Ok( FetchedInst{ opcode: OpcodeCompat::Hlt, inst_bytes: 1, } )
            }
            OpcodeCompat::Xor => {
                Ok( FetchedInst{ opcode: OpcodeCompat::Xor, inst_bytes: 2, } )
            }
        }
    } else {
        Err(CompatibleException ("Undefined opcode.".to_string()) )
    }
}