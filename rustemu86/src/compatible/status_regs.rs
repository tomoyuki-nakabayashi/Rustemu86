use compatible::{Result, CompatibleException};
use compatible::isa::opcode::OpcodeCompat;

#[derive(Debug, PartialEq, Clone, Copy)]
pub(super) enum CpuState {
    Halted,
    Running
}
