extern crate gtk;
extern crate gio;

use gtk::Grid;
use gtk::{ WidgetExt, WindowExt, ContainerExt };
use gtk::{ LabelExt, GridExt };
use gio::{ ApplicationExt };

fn create_text_grid() -> Grid {
  let grid = gtk::Grid::new();
  let mut text: Vec<Vec<gtk::Label>> = Vec::new();
  for row in 0..25 {
    text.push(Vec::new());
    for col in 0..80 {
      text[row].push(gtk::Label::new("a"));
      grid.attach(&text[row][col], col as i32, row as i32, 1, 1);
    }
  }

  grid
}

pub fn init_display(activate_cb: fn())
{
  match gtk::Application::new("com.github.tomoyuki-nakabayashi.Rustemu86", gio::APPLICATION_HANDLES_OPEN) {
    Ok(app) => {
      app.connect_activate(move |app| {
        let win = gtk::ApplicationWindow::new(&app);
        win.set_default_size(720, 400);
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
