//! Debug Interface. This may work the target processor is being halted.

/// These get/set functions are only used for test / debug.
pub trait DebugInterface {
    fn set_pc(&mut self, pc: u32);

    fn get_pc(&self) -> u32;

    fn set_gpr(&mut self, index: u32, value: u32);

    fn get_gpr(&self, index: u32) -> u32;

    fn get_csr(&self, index: u32) -> u32;
}
