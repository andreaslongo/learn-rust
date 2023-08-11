use std::env;
use std::error::Error;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

struct Config<'a> {
    query: &'a String,
    file_path: &'a String,
}

impl Config<'_> {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = &args[1];
        let file_path = &args[2];

        Ok(Config { query, file_path })
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("Searching for '{}'", config.query);
    println!("In file '{}'", config.file_path);

    let contents = fs::read_to_string(config.file_path)?;

    println!("With text:\n{contents}");
    Ok(())
}
