use std::fs;

fn main() {
    // get list of files in current directory
    let files = fs::read_dir(".");

    files.unwrap().for_each(|file| {
        // unwrap the file
        let file = file.unwrap();

        // get the path
        let path = file.path();

        // print the path
        println!("{}", path.display());
    });
}
