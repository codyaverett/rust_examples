use crossterm::{
    execute,
    terminal::{size, ScrollUp, SetSize},
    Result,
};
use std::io::{stdout, Write};

fn main() -> Result<()> {
    let (cols, rows) = size()?;
    // Resize terminal and scroll up.
    execute!(stdout(), SetSize(10, 10), ScrollUp(5))?;
    println!("Hello, world!");

    // Be a good citizen, cleanup
    execute!(stdout(), SetSize(cols, rows))?;
    Ok(())
}
