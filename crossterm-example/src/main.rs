use crossterm::{
    execute,
    terminal::{size, ScrollUp, SetSize},
    Result,
};
use std::io::{stdout, Write};

fn main() -> Result<()> {
    let (cols, rows) = size()?;

    let mut stdout = stdout();
    execute!(stdout, SetSize(cols, rows), ScrollUp(3))?;
    write!(stdout, "Hello, world!")?;

    Ok(())
}
