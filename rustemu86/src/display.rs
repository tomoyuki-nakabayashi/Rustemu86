use conrod::{self, widget, Colorable, Positionable, Widget};
use conrod::backend::glium::glium::{self, Surface};

pub fn init_display(activate_cb: fn()) {
  const WIDTH: u32 = 720;
  const HEIGHT: u32 = 400;

  let mut events_loop = glium::glutin::EventsLoop::new();
  let window = glium::glutin::WindowBuilder::new()
            .with_title("Hello Conrod!")
            .with_dimensions((WIDTH, HEIGHT).into());
  let context = glium::glutin::ContextBuilder::new()
            .with_vsync(true);
  let display = glium::Display::new(window, context, &events_loop).unwrap();

  activate_cb();
}
