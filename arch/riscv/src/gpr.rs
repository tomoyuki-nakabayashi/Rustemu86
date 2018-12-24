//! General purpose integer 32-bit register.

const NUM_OF_GPR: usize = 32;
const MAX_GPR_INDEX: usize = NUM_OF_GPR - 1;

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
    pub fn read_u32(&self, index: GprIndex) -> u32 {
        self.ram[index.0]
    }

    /// Write data to the index. Always ignores write to zero register.
    pub fn write_u32(&mut self, index: GprIndex, value: u32) {
        match index {
            GprIndex(0) => (),
            _ => self.ram[index.0] = value,
        };
    }
}

/// Index for general purpose register.
/// Using `GprIndex` guarantees read/write operation of `Gpr` always success.
#[derive(Debug, Clone, Copy)]
pub struct GprIndex(usize);

impl GprIndex {
    /// Create a new GprIndex.
    pub fn try_from(index: usize) -> Result<GprIndex, ()> {
        match index {
            0...MAX_GPR_INDEX => Ok(GprIndex(index)),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn write_zero_register() {
        let mut gpr = Gpr::new();

        let index = GprIndex::try_from(0).unwrap();
        gpr.write_u32(index, 1);
        assert_eq!(gpr.ram[0], 0);
    }

    #[test]
    fn read_after_write() {
        let mut gpr = Gpr::new();

        let index = GprIndex::try_from(31).unwrap();
        assert_eq!(gpr.read_u32(index), 0);
        gpr.write_u32(index, 1);
        assert_eq!(gpr.read_u32(index), 1);
    }
}