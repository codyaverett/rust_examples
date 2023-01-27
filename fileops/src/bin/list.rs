fn main() {
    // print cwd
    let cwd = std::env::current_dir().unwrap();
    println!("{:?}", cwd);

    // get a vector of all files in the current directory
    let files = std::fs::read_dir("./src").unwrap();
    files
        .into_iter()
        .map(|x| x)
        .collect::<Vec<_>>() // unwrap the Result
        .into_iter()
        .map(|x| x.path()) // get the path
        .into_iter()
        .map(|x| x.to_str().unwrap()) // convert to string
        .for_each(|x| println!("{}", x));
}
