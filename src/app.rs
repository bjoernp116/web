use console_engine::ConsoleEngine;

pub struct App {
  engine: ConsoleEngine,
  size: (usize, usize),
}
impl App {
  pub fn new() -> App {
    let engine = ConsoleEngine::init_fill(30);
    let size = (
      engine.get_width(),
      engine.get_height()
    );
    App { engine, size }
  }
}