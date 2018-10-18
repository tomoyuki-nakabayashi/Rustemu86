use peripherals::interconnect::Interconnect;
use std::fmt;

/// The factory to create a CPU object.
pub fn cpu_factory<T>(interconnect: Interconnect) -> T
    where T: CpuModel + Pipeline
{
    T::new(interconnect)
}

/// All CPUs must implement CpuModel trait.
pub trait CpuModel {
    type Error;

    /// Create instance which holds an interface to the interconnect.
    fn new(interconnect: Interconnect) -> Self;

    /// Initialize cpu state and register including program counter.
    fn init(&mut self);

    /// Start execution of the program from the entry point.
    fn run(&mut self) -> Result<(), Self::Error>;
}

/// Instruction pipeline.
pub trait Pipeline {
    type Error;
    type Fetched;
    type Decoded;
    type Executed;

    /// Execute an instruction from the program.
    fn execute_an_instruction(&mut self, program: &[u8]) -> Result<(), Self::Error> {
        let result = self.fetch(program)
            .map(|inst| self.decode(&inst))?
            .map(|inst| self.execute(&inst))?;

        let result = result?;
        self.write_back(&result)
    }

    /// Fetch an instruction from the program.
    fn fetch(&self, program: &[u8]) -> Result<Self::Fetched, Self::Error>;

    /// Decode a fethced instruction.
    fn decode(&self, inst: &Self::Fetched) -> Result<Self::Decoded, Self::Error>;

    /// Execute a decoded instruction.
    fn execute(&self, inst: &Self::Decoded) -> Result<Self::Executed, Self::Error>;

    /// Write back the result of execution.
    /// Only this method updates the CPU state.
    fn write_back(&mut self, inst: &Self::Executed) -> Result<(), Self::Error>;
}

/// Emulation Error.
#[derive(Debug)]
pub struct EmulationError(String);

impl fmt::Display for EmulationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0)
    }
}