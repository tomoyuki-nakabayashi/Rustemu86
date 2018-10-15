use peripherals::interconnect::Interconnect;
use std::fmt;

/// All CPUs must implement CpuModel trait.
pub trait CpuModel {
    /// Each CPU has own execution pipeline.
    type Pipeline;

    /// Create instance which holds an interface to the interconnect.
    fn new(interconnect: Interconnect) -> Self;

    /// Initialize cpu state and register including program counter.
    fn init(&self mut);

    /// Start execution of the program from the entry point.
    fn run(&self mut) -> Result<(), EmulationError>;
}

/// Instruction pipeline.
pub trait Pipeline {
    type Fetched;
    type Decoded;
    type Executed;

    /// Execute an instruction from the program.
    fn execute_an_instruction(&mut self, program: &[u8]) -> Result<(), EmulationError> {
        let result = Self.fetch(program)
            .map(|inst| Self.decode(&inst))
            .map(|inst| Self.execute(&inst))?;

        self.write_back(&result)
    }

    /// Fetch an instruction from the program.
    fn fetch(program: &[u8]) -> Result<Fetched, EmulationError>;

    /// Decode a fethced instruction.
    fn decode(inst: &Fetched) -> Result<Decoded, EmulationError>;

    /// Execute a decoded instruction.
    fn execute(inst: &Decoded) -> Result<Executed, EmulationError>;

    /// Write back the result of execution.
    /// Only this method updates the CPU state.
    fn write_back(&mut self, inst: Executed) -> Result<(), EmulationError>;
}

/// Emulation Error.
#[derive(Debug)]
pub struct EmulationError(String);

impl fmt::Display for EmulationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0)
    }
}