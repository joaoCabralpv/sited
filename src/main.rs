use crossterm::style::Print;
use crossterm::{QueueableCommand, cursor};
use crossterm::terminal;
use std::io::{Write, stdout,self};
fn main() -> io::Result<()>{
    let mut stdout = stdout();
    crossterm::queue!(stdout,cursor::MoveTo(5,5),terminal::Clear(terminal::ClearType::All),Print("Hello, world"))?;
    stdout.flush()?;
    io::Result::Ok(())
}
