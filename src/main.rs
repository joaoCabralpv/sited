mod App;
mod app;
mod exit;
mod io;
use crossterm::terminal;

fn main() -> std::io::Result<()> {
    terminal::enable_raw_mode()?;
    let mut app = app::App::new();
    while !app.update() {} // main app loop
    terminal::disable_raw_mode()?;
    Ok(())
}
