extern crate gtk;
extern crate gio;

use gtk::Grid;
use gtk::{ WidgetExt, WindowExt, ContainerExt, Cast };
use gtk::{ LabelExt, GridExt };
use gio::{ ApplicationExt };

const ROW: usize = 25;
const COL: usize = 80;

fn create_text_grid() -> Grid {
  let screen = gtk::Grid::new();
  let mut text: Vec<Vec<gtk::Label>> = Vec::new();
  for row in 0..ROW {
    text.push(Vec::new());
    for col in 0..COL {
      text[row].push(gtk::Label::new(None));
      text[row][col].set_markup("<span font_family=\"monospace\" size=\"13000\" background=\"black\"> </span>");
      screen.attach(&text[row][col], col as i32, row as i32, 1, 1);
    }
  }

  screen
}

fn update_screen_char(screen_buffer: &gtk::Grid, pos: usize, screen_char: u8) {
  use std::fmt::Write;
  let mut markup = String::new();
  let child = screen_buffer.get_child_at(0, 0).unwrap().downcast::<gtk::Label>().ok().unwrap();
  write!(markup, "<span font_family=\"monospace\" size=\"13000\" background=\"black\">{}</span>", screen_char as char).unwrap();
  child.set_markup(&markup);
}

pub fn init_display(activate_cb: fn())
{
  match gtk::Application::new("com.github.tomoyuki-nakabayashi.Rustemu86", gio::APPLICATION_HANDLES_OPEN) {
    Ok(app) => {
      app.connect_activate(move |app| {
        let win = gtk::ApplicationWindow::new(&app);
        win.set_default_size(720, 400);
        win.set_title("Rustemu86");

        let screen = create_text_grid();
        win.add(&screen);
        win.show_all();
        update_screen_char(&screen, 0, 'a' as u8);
        activate_cb();
      });

      app.run(&[""]);
    },
    Err(_) => {
      println!("Application start up error");
    }
  };
}
