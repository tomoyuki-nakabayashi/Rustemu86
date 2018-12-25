//! General purpose integer 32-bit register.

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
        Gpr { ram: [0u32; NUM_OF_GPR] }
    }

    /// Read data. Range check shouldn't be required.
    /// Because GprIndex must have a valid index.
    pub fn read_u32(&self, index: usize) -> u32 {
        assert!(index <= MAX_GPR_INDEX, "Invalid register index");
        self.ram[index]
    }

    /// Write data to the index. Always ignores write to zero register.
    pub fn write_u32(&mut self, index: usize, value: u32) {
        assert!(index <= MAX_GPR_INDEX, "Invalid register index");

        if index == ZERO_REGISTER {
            return;
        }
        self.ram[index] = value;
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