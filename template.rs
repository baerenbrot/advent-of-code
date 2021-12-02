use std::fs;
use std::env;

#[derive(Debug, Copy, Clone)]
enum Error {
    FileReadError,
}

fn lines(filename: &str) -> Result<Vec<String>, Error> {
    Ok(fs::read_to_string(filename)
        .map_err(|_| Error::FileReadError)?
        .split('\n')
        .map(|x| String::from(x.trim()))
        .filter(|x| x.len() > 0)
        .collect::<Vec<_>>())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please specify input file.");
    } else if let Ok(lines) = lines(&args[1]) {
        // work
    } else {
        println!("Failed to read from file: {}", args[1]);
    }
}
