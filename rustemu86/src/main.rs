use gui::display::{start_with_gtk, GtkVgaTextBuffer};
use peripherals::memory_access::MemoryAccess;
use peripherals::uart16550::{uart_factory, Target};
use rustemu86::{
    loader::{load, map_to_memory},
    options::parse_args,
    start_emulation,
};

fn start_rustemu86(screen: GtkVgaTextBuffer) {
    let options = parse_args();
    let mut reader = load(&options.file_path).unwrap();
    let program = map_to_memory(&mut reader).unwrap();
    let display: Box<dyn MemoryAccess> = Box::new(screen);
    let serial = uart_factory(Target::Stdout);

    let _result = start_emulation(program, options.emulation_mode, serial, display);
}

fn main() {
    start_with_gtk(start_rustemu86);
}
