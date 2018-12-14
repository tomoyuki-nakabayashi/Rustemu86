use gui::display::{start_with_gtk, GtkVgaTextBuffer};
use peripherals::uart16550::{uart_factory, Target};
use peripherals::memory_access::MemoryAccess;
use rustemu86;

fn start_rustemu86(screen: GtkVgaTextBuffer) {
    let options = rustemu86::options::parse_args();
    let mut reader = rustemu86::loader::load(&options.file_path).unwrap();
    let program = rustemu86::loader::map_to_memory(&mut reader).unwrap();
    let display: Box<dyn MemoryAccess> = Box::new(screen);
    let serial = uart_factory(Target::Stdout);

    let _result = rustemu86::start_emulation(program, options.emulation_mode, serial, display);
}

fn main() {
    gui::display::start_with_gtk(start_rustemu86);
}
