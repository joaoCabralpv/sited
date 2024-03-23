mod interface;
mod io;
use crossterm::terminal;



fn main() ->std::io::Result<()>{   
    terminal::enable_raw_mode()?;
    interface::hello();
    Ok(())
}