//! General purpose integer 32-bit register.

use num::FromPrimitive;
use std::fmt;

const NUM_OF_GPR: usize = 32;
const MAX_GPR_INDEX: usize = NUM_OF_GPR - 1;
const ZERO_REGISTER: usize = 0;

/// General purpose integer 32-bit register.
/// The slice `ram` contains zero register as well as others.
pub struct Gpr {
    ram: [u32; NUM_OF_GPR],
}

impl Gpr {
    /// Initialize all register as `0`.
    pub fn new() -> Gpr {
        Gpr {
            ram: [0u32; NUM_OF_GPR],
        }
    }

    /// Read data. Range check shouldn't be required.
    /// Because GprIndex must have a valid index.
    pub fn read_u32(&self, index: u32) -> u32 {
        let index = usize::from_u32(index).expect("invalid register index");
        assert!(
            index <= MAX_GPR_INDEX,
            "register index must be smaller than 31 but {}",
            index
        );
        self.ram[index]
    }

    /// Write data to the index. Always ignores write to zero register.
    pub fn write_u32(&mut self, index: u32, value: u32) {
        let index = usize::from_u32(index).expect("invalid register index");
        assert!(
            index <= MAX_GPR_INDEX,
            "register index must be smaller than 31 but {}",
            index
        );

        if index == ZERO_REGISTER {
            return;
        }
        self.ram[index] = value;
    }
}

impl fmt::Display for Gpr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, r"
zero: {:08x}, ra : {:08x}, sp : {:08x}, gp : {:08x}
tp  : {:08x}, t0 : {:08x}, t1 : {:08x}, t2 : {:08x}
s0  : {:08x}, s1 : {:08x}, a0 : {:08x}, a1 : {:08x}
a2  : {:08x}, a3 : {:08x}, a4 : {:08x}, a5 : {:08x}
a6  : {:08x}, a7 : {:08x}, s2 : {:08x}, s3 : {:08x}
s4  : {:08x}, s5 : {:08x}, s6 : {:08x}, s7 : {:08x}
s8  : {:08x}, s9 : {:08x}, s10: {:08x}, s11: {:08x}
t3  : {:08x}, t4 : {:08x}, t5 : {:08x}, t6 : {:08x}
",
self.ram[0], self.ram[1], self.ram[2], self.ram[3],
self.ram[4], self.ram[5], self.ram[6], self.ram[7],
self.ram[8], self.ram[9], self.ram[10], self.ram[11],
self.ram[12], self.ram[13], self.ram[14], self.ram[15],
self.ram[16], self.ram[17], self.ram[18], self.ram[19],
self.ram[20], self.ram[21], self.ram[22], self.ram[23],
self.ram[24], self.ram[25], self.ram[26], self.ram[27],
self.ram[28], self.ram[29], self.ram[30], self.ram[31],
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn write_zero_register() {
        let mut gpr = Gpr::new();

        gpr.write_u32(0, 1);
        assert_eq!(gpr.ram[0], 0);
    }

    #[test]
    fn read_after_write() {
        let mut gpr = Gpr::new();

        assert_eq!(gpr.read_u32(1), 0);
        gpr.write_u32(1, 1);
        assert_eq!(gpr.read_u32(1), 1);
    }
}
