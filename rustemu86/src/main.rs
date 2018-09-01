extern crate rustemu86;

use display::GtkVgaTextBuffer;
use rustemu86::display;

fn start_rustemu86(screen: GtkVgaTextBuffer) {
    let args = rustemu86::args::parse_args();
    let mut reader = rustemu86::loader::load(&args.file_path).unwrap();
    let program = rustemu86::loader::map_to_memory(&mut reader).unwrap();
    let _result = rustemu86::start_emulation(program, args.emulation_mode, screen);
}

fn main() {
    display::start_with_gtk(start_rustemu86);
}
