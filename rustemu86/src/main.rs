extern crate rustemu86;
extern crate gtk;
extern crate gio;

use gtk::{ WidgetExt, WindowExt, ContainerExt };
use gtk::LabelExt;
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
        win.set_title("Rustemu86");
        let label = gtk::Label::new(None);
        win.add(&label);
        label.set_markup("<span foreground=\"blue\">Hello</span>");
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
