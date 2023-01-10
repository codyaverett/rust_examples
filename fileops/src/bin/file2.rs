use std::{fs::File, io::Read};

fn main() {
    let mut file = File::open("Cargo.toml").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("With file2:\n{}", contents);
}
