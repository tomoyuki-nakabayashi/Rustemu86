extern crate rustemu86;

use display::GtkVgaTextBuffer;
use rustemu86::display;

fn start_rustemu86(screen: GtkVgaTextBuffer) {
    let options = rustemu86::options::parse_args();
    let mut reader = rustemu86::loader::load(&options.file_path).unwrap();
    let program = rustemu86::loader::map_to_memory(&mut reader).unwrap();
    let _result = rustemu86::start_emulation(program, options.emulation_mode, screen);
}

fn main() {
    display::start_with_gtk(start_rustemu86);
}
