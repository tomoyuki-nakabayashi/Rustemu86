use bit_field::BitField;
use peripherals::error::MemoryAccessError;
use peripherals::memory_access::MemoryAccess;
use gio::ApplicationExt;
use gtk::{Cast, Grid};
use gtk::{ContainerExt, GridExt, LabelExt, WidgetExt, WindowExt};
use num::{FromPrimitive, Integer};
use enum_primitive::*;
use std::fmt;

const ROW: usize = 25;
const COL: usize = 80;
const SINGLE_CHAR_BYTE: usize = 2;

enum_from_primitive! {
  #[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

pub struct GtkVgaTextBuffer {
    gtk_grid: Option<Grid>,
    buffer: [[ScreenChar; COL]; ROW],
}

impl GtkVgaTextBuffer {
    pub fn new() -> GtkVgaTextBuffer {
        GtkVgaTextBuffer {
            gtk_grid: None,
            buffer: [[ScreenChar {
                ascii_character: b' ',
                background: Color::Black,
                foreground: Color::Black,
            }; COL]; ROW],
        }
    }

    fn get_child_at(&self, row: i32, col: i32) -> Result<gtk::Label, &str> {
        let screen = self.gtk_grid.as_ref().expect("Buffer is not initialized.");
        if let Some(child) = screen.get_child_at(col, row) {
            child
                .downcast::<gtk::Label>()
                .or(Err("Fail to retreave Label."))
        } else {
            Err("Child not found.")
        }
    }

    fn draw(&self, row: usize, col: usize) {
        use std::fmt::Write;
        let screen_char = &self.buffer[row][col];

        let mut markup = String::new();
        write!(markup,
      "<span font_family=\"monospace\" size=\"13000\" foreground=\"{}\" background=\"{}\">{}</span>",
      screen_char.foreground,
      screen_char.background,
      screen_char.ascii_character as char).unwrap();

        let child = self.get_child_at(row as i32, col as i32).unwrap();
        child.set_markup(&markup);
    }

    fn draw_all(&self) {
        for row in 0..ROW {
            for col in 0..COL {
                self.draw(row, col);
            }
        }
    }
}

impl MemoryAccess for GtkVgaTextBuffer {
    /// Cannot read a while.
    fn read_u8(&self, _addr: usize) -> Result<u8, MemoryAccessError> {
        Err(MemoryAccessError::NoPermission)
    }

    fn write_u8(&mut self, addr: usize, byte: u8) -> Result<(), MemoryAccessError> {
        let (row, col) = convert_addr_to_axis(addr);
        if addr.is_even() {
            self.buffer[row][col].set_ascii(byte);
        } else {
            self.buffer[row][col].set_color(byte);
        }
        self.draw(row, col);
        Ok(())
    }
}

fn convert_addr_to_axis(addr: usize) -> (usize, usize) {
    let row = addr / (COL * SINGLE_CHAR_BYTE);
    let col = (addr / SINGLE_CHAR_BYTE) % COL;
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
    match gtk::Application::new(
        "com.github.tomoyuki-nakabayashi.Rustemu86",
        gio::APPLICATION_HANDLES_OPEN,
    ) {
        Ok(app) => {
            app.connect_activate(move |app| {
                let win = gtk::ApplicationWindow::new(&app);
                win.set_default_size(720, 400);
                win.set_title("Rustemu86");

                let screen = create_text_grid();
                win.add(&screen);
                win.show_all();

                let mut text_buffer = GtkVgaTextBuffer::new();
                text_buffer.gtk_grid = Some(screen);
                text_buffer.draw_all();

                start_emulation(text_buffer);
            });

            app.run(&[""]);
        }
        Err(_) => {
            println!("Application start up error");
        }
    };
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn convert_addr() {
        assert_eq!(convert_addr_to_axis(0), (0, 0));
        assert_eq!(convert_addr_to_axis(1), (0, 0));
        assert_eq!(convert_addr_to_axis(2), (0, 1));
        assert_eq!(convert_addr_to_axis(COL * SINGLE_CHAR_BYTE), (1, 0));
    }
}
