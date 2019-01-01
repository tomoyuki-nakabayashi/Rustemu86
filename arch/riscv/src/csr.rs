//! Control and Status Register

use num::FromPrimitive;

const NUM_OF_CSR: usize = 4096;
const MAX_CSR_INDEX: usize = NUM_OF_CSR - 1;

/// Control and Status Register.
pub struct Csr {
    ram: [u32; NUM_OF_CSR],
}

impl Csr {
    /// Initialize all register as `0`.
    pub fn new() -> Csr {
        Csr {
            ram: [0u32; NUM_OF_CSR],
        }
    }

    /// Read data.
    pub fn read_u32(&self, index: u32) -> u32 {
        let index = usize::from_u32(index).expect("invalid register index");
        assert!(
            index <= MAX_CSR_INDEX,
            "register index must be smaller than 255 but {}",
            index
        );
        self.ram[index]
    }

    /// Write data to the index.
    pub fn write_u32(&mut self, index: u32, value: u32) {
        let index = usize::from_u32(index).expect("invalid register index");
        assert!(
            index <= MAX_CSR_INDEX,
            "register index must be smaller than 31 but {}",
            index
        );
        self.ram[index] = value;
    }
}
