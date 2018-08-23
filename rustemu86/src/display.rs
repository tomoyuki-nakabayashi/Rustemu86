extern crate gtk;
extern crate gio;

use gtk::{ WidgetExt, WindowExt, ContainerExt };
use gtk::{ LabelExt, GridExt };
use gio::{ ApplicationExt };

pub fn init_display(activate_cb: fn())
{
  match gtk::Application::new("com.github.tomoyuki-nakabayashi.Rustemu86", gio::APPLICATION_HANDLES_OPEN) {
    Ok(app) => {
      app.connect_activate(move |app| {
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
        activate_cb();
      });

      app.run(&[""]);
    },
    Err(_) => {
      println!("Application start up error");
    }
  };
}