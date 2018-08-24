extern crate gtk;
extern crate gio;

use gtk::Grid;
use gtk::{ WidgetExt, WindowExt, ContainerExt };
use gtk::{ LabelExt, GridExt };
use gio::{ ApplicationExt };

fn create_text_grid() -> Grid {
  let grid = gtk::Grid::new();
  let mut row: Vec<gtk::Label> = Vec::new();
  for i in 0..80 {
    row.push(gtk::Label::new("a"));
  }

  for (col, label) in row.iter().by_ref().enumerate() {
    grid.attach(label, 10*(col as i32), 10, 10, 10);
  }

  grid
}

pub fn init_display(activate_cb: fn())
{
  match gtk::Application::new("com.github.tomoyuki-nakabayashi.Rustemu86", gio::APPLICATION_HANDLES_OPEN) {
    Ok(app) => {
      app.connect_activate(move |app| {
        let win = gtk::ApplicationWindow::new(&app);
        win.set_default_size(640, 480);
        win.set_title("VGA text mode");

        let grid = create_text_grid();
        win.add(&grid);
        win.show_all();
        activate_cb();
      });

      app.run(&[""]);
    },
    Err(_) => {
      println!("Application start up error");
    }
  };
}