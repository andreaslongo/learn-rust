use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    println!("Searching for '{}'", config.query);
    println!("In file '{}'", config.file_path);

    let contents =
        fs::read_to_string(config.file_path).expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}

struct Config<'a> {
    query: &'a String,
    file_path: &'a String,
}

impl Config<'_> {
    fn new(args: &[String]) -> Config {
        let query = &args[1];
        let file_path = &args[2];

        Config { query, file_path }
    }
}

// https://doc.rust-lang.org/book/ch12-03-improving-error-handling-and-modularity.html#fixing-the-error-handling
