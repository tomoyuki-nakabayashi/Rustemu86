extern crate gtk;
extern crate gio;

use gtk::Grid;
use gtk::{ WidgetExt, WindowExt, ContainerExt, Cast };
use gtk::{ LabelExt, GridExt };
use gio::{ ApplicationExt };
use std::fmt;
use std::convert::From;
use num::FromPrimitive;
use bit_field::BitField;

const ROW: usize = 25;
const COL: usize = 80;

enum_from_primitive! {
  #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  #[repr(u8)]
  enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
  }
}

impl fmt::Display for Color {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Color::Black => "black",
        Color::Blue => "blue",
        Color::Green => "green",
        Color::Cyan => "cyan",
        Color::Red => "red",
        Color::Magenta => "magenta",
        Color::Brown => "brown",
        Color::LightGray => "lightgray",
        Color::DarkGray => "darkgray",
        Color::LightBlue => "lightblue",
        Color::LightGreen => "lightgreen",
        Color::LightCyan => "lightcyan",
        Color::LightRed => "red",
        Color::Pink => "pink",
        Color::Yellow => "yellow",
        Color::White => "white",
      }
    )
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    background: Color,
    foreground: Color,
}

impl From<u16> for ScreenChar {
  fn from(item: u16) -> Self {
    let color_code = (item >> 8) as u8;
    ScreenChar {
      ascii_character: item as u8,
      background: Color::from_u8(color_code.get_bits(4..8)).unwrap(),
      foreground: Color::from_u8(color_code.get_bits(0..4)).unwrap(),
    }
  }
}

pub trait VgaTextMode {
  fn write(&self, address_offset: usize, screen_char: u16);
}

pub struct GtkVgaTextBuffer (Option<Grid>);

impl GtkVgaTextBuffer {
  pub fn new() -> GtkVgaTextBuffer {
    GtkVgaTextBuffer(None)
  }

  fn get_child_at(&self, address_offset: usize) -> gtk::Label {
    const SINGLE_CHAR_BYTE: usize = 2;
    let x = ((address_offset / SINGLE_CHAR_BYTE) % COL) as i32;
    let y = (address_offset / (COL * SINGLE_CHAR_BYTE)) as i32;
    let buffer = self.0.as_ref().unwrap();
    buffer.get_child_at(x, y).unwrap().downcast::<gtk::Label>().ok().unwrap()
  }
}

impl VgaTextMode for GtkVgaTextBuffer {
  fn write(&self, address_offset: usize, screen_char: u16) {
    use std::fmt::Write;
    let screen_char = ScreenChar::from(screen_char);
    let mut markup = String::new();
    write!(markup,
      "<span font_family=\"monospace\" size=\"13000\" foreground=\"{}\" background=\"{}\">{}</span>",
      screen_char.foreground,
      screen_char.background,
      screen_char.ascii_character as char).unwrap();

    let child = self.get_child_at(address_offset);
    child.set_markup(&markup);
  }
}

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

pub fn start_with_gtk(start_emulation: fn(GtkVgaTextBuffer)) {
  match gtk::Application::new("com.github.tomoyuki-nakabayashi.Rustemu86", gio::APPLICATION_HANDLES_OPEN) {
    Ok(app) => {
      app.connect_activate(move |app| {
        let win = gtk::ApplicationWindow::new(&app);
        win.set_default_size(720, 400);
        win.set_title("Rustemu86");

        let screen = create_text_grid();
        win.add(&screen);
        win.show_all();
        let text_mode = GtkVgaTextBuffer(Some(screen));
        text_mode.write(0, 'a' as u16 | (Color::Yellow as u16) << 8);
        start_emulation(text_mode);
      });

      app.run(&[""]);
    },
    Err(_) => {
      println!("Application start up error");
    }
  };
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn create_screen_char() {
    let data: u16 = (Color::Blue as u16) << 12 | (Color::White as u16) << 8 | 'a' as u16;
    let decoded = ScreenChar::from(data);

    assert_eq!(decoded.ascii_character, 'a' as u8);
    assert_eq!(decoded.background, Color::Blue);
    assert_eq!(decoded.foreground, Color::White);
  }
}