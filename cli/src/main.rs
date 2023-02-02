use clap::Parser;

#[derive(Parser)]
struct Cli {
    // The pattern to look for
    pattern: String,
    // Note: PathBuf is like a String but for file system paths that work cross-platform.
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();

    // We can use the `args` variable to access the arguments passed to the program.
    println!("Looking for {} in {}", args.pattern, args.path.display());

    let content = std::fs::read_to_string(&args.path).expect("could not read file");

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

    /*
        Exercise for the reader:
        This is not the best implementation:
        It will read the whole file into memory – however large the file may be.
        Find a way to optimize it!
        (One idea might be to use a BufReader instead of read_to_string().)
    */
}
