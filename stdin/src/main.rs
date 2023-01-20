use std::io;

use atty::Stream;

fn load_stdin() -> io::Result<String> {
    if atty::is(Stream::Stdin) {
        return Err(io::Error::new(io::ErrorKind::Other, "stdin not redirected"));
    }
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    return Ok(buffer);
}

fn main() -> io::Result<()> {
    println!("line: {}", load_stdin()?);
    Ok(())
}
