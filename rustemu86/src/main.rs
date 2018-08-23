extern crate rustemu86;
extern crate gtk;
extern crate gio;

use gtk::{ WidgetExt, WindowExt, ContainerExt };
use gtk::{ LabelExt, GridExt };
use gio::{ ApplicationExt };

fn start_rustemu86() {
  let args = rustemu86::args::parse_args();
  let mut reader = rustemu86::loader::load(&args.file_path).unwrap();
  let program = rustemu86::loader::map_to_memory(&mut reader).unwrap();
  let result = rustemu86::start_emulation(program, args.emulation_mode);
  assert!(result.is_ok());
}

fn main() {
  match gtk::Application::new("com.github.tomoyuki-nakabayashi.Rustemu86", gio::APPLICATION_HANDLES_OPEN) {
    Ok(app) => {
      app.connect_activate(|app| {
        let win = gtk::ApplicationWindow::new(&app);
        win.set_default_size(640, 480);
        win.set_title("VGA text mode");
        let grid = gtk::Grid::new();
        win.add(&grid);
        let label1 = gtk::Label::new("a");
        let label2 = gtk::Frame::new("b");
        grid.attach(&label1, 10, 10, 10, 10);
        grid.attach(&label2, 20, 10, 10, 10);
        win.show_all();
        start_rustemu86();
      });

      app.run(&[""]);
    },
    Err(_) => {
      println!("Application start up error");
    }
  };
}
