use peripherals::interconnect::Interconnect;
use rustemu86::DebugMode;

/// The factory to create a CPU object.
pub fn cpu_factory<T>(mmio: Interconnect, debug: Box<dyn DebugMode>) -> T
where
    T: CpuModel + Pipeline,
{
    T::new(mmio, debug)
}

/// All CPUs must implement CpuModel trait.
pub trait CpuModel {
    type Error;

    /// Create instance which holds an interface to the interconnect.
    fn new(mmio: Interconnect, debug: Box<dyn DebugMode>) -> Self;

    /// Initialize x86_64 state and register including program counter.
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
        let result = self
            .fetch(program)
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
