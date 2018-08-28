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
const SINGLE_CHAR_BYTE: usize = 2;

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

impl ScreenChar {
  fn set_ascii(&mut self, character: u8) {
    self.ascii_character = character;
  }

  fn set_color(&mut self, color_code: u8) {
    self.background = Color::from_u8(color_code.get_bits(4..8)).unwrap();
    self.foreground = Color::from_u8(color_code.get_bits(0..4)).unwrap();
  }
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
  fn write_u8(&self, addr: usize, byte: u8);
  fn write_u16(&self, addr: usize, screen_char: u16);
}

pub struct GtkVgaTextBuffer {
  gtk_grid: Option<Grid>,
  buffer: [[ScreenChar; 80]; 25],
}

impl GtkVgaTextBuffer {
  pub fn new() -> GtkVgaTextBuffer {
    GtkVgaTextBuffer{
      gtk_grid: None,
      buffer: [[ScreenChar {
        ascii_character: ' ' as u8,
        background: Color::Black,
        foreground: Color::Black }; 80]; 25],
    }
  }

  fn get_child_at(&self, address_offset: usize) -> gtk::Label {
//    let (y, x) = get_child_position(address_offset);
    let x = ((address_offset / SINGLE_CHAR_BYTE) % COL) as i32;
    let y = (address_offset / (COL * SINGLE_CHAR_BYTE)) as i32;
    let buffer = self.gtk_grid.as_ref().unwrap();
    buffer.get_child_at(x, y).unwrap().downcast::<gtk::Label>().ok().unwrap()
  }

  fn draw(&self, row: usize, col: usize) {
    use std::fmt::Write;
    let ref screen_char = self.buffer[row][col];

    let mut markup = String::new();
    write!(markup,
      "<span font_family=\"monospace\" size=\"13000\" foreground=\"{}\" background=\"{}\">{}</span>",
      screen_char.foreground,
      screen_char.background,
      screen_char.ascii_character as char).unwrap();

    let child = self.get_child_at((col + row*COL) * 2);
    child.set_markup(&markup);
  }
}

impl VgaTextMode for GtkVgaTextBuffer {
  fn write_u8(&self, addr: usize, byte: u8) {
    let (row, col) = get_child_position(addr);
  }

  fn write_u16(&self, addr: usize, screen_char: u16) {
    use std::fmt::Write;
    let screen_char = ScreenChar::from(screen_char);
    let mut markup = String::new();
    write!(markup,
      "<span font_family=\"monospace\" size=\"13000\" foreground=\"{}\" background=\"{}\">{}</span>",
      screen_char.foreground,
      screen_char.background,
      screen_char.ascii_character as char).unwrap();

    let child = self.get_child_at(addr);
    child.set_markup(&markup);
  }
}

fn get_child_position(addr: usize) -> (i32, i32) {
  let row = (addr / (COL * SINGLE_CHAR_BYTE)) as i32;
  let col = ((addr / SINGLE_CHAR_BYTE) % COL) as i32;
  (row, col)
}

fn create_text_grid() -> Grid {
  let screen = gtk::Grid::new();
  let mut text: Vec<Vec<gtk::Label>> = Vec::new();
  for row in 0..ROW {
    text.push(Vec::new());
    for col in 0..COL {
      text[row].push(gtk::Label::new(None));
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
        let mut text_mode = GtkVgaTextBuffer::new();
        text_mode.gtk_grid = Some(screen);
        for row in 0..ROW {
          for col in 0..COL {
            text_mode.draw(row, col);
          }
        }
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
