use compatible::{Result, CompatibleException};
use compatible::isa::opcode::OpcodeCompat;
use num::FromPrimitive;

pub(super) struct FetchedInst {
    opcode: OpcodeCompat,
    next_ip: Box<dyn Fn(u64) -> u64>,
}

impl FetchedInst {
    pub(super) fn increment_ip(&self, ip: u64) -> u64 {
        (self.next_ip)(ip)
    }
}

pub(super) fn fetch(program: &[u8]) -> Result<FetchedInst> {
    if let Some(opcode) = OpcodeCompat::from_u8(program[0]) {
        match opcode {
            OpcodeCompat::Hlt => {
                Ok( FetchedInst{ opcode: OpcodeCompat::Hlt, next_ip: Box::new(|ip| ip + 1), } )
            }
            OpcodeCompat::Xor => {
                Ok( FetchedInst{ opcode: OpcodeCompat::Xor, next_ip: Box::new(|ip| ip + 2), } )
            }
        }
    } else {
        Err(CompatibleException ("Undefined opcode.".to_string()) )
    }
}