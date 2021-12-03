use std::fs;
use std::env;

#[derive(Debug, Clone)]
enum Error {
    FileReadError(String),
    MissingArgument
}

fn lines(filename: &str) -> Result<Vec<String>, Error> {
    Ok(fs::read_to_string(filename)
        .map_err(|_| Error::FileReadError(String::from(filename)))?
        .split('\n')
        .map(|x| String::from(x.trim()))
        .filter(|x| x.len() > 0)
        .collect::<Vec<_>>())
}

fn filename() -> Result<String, Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        Err(Error::MissingArgument)
    } else {
        Ok(args[1].clone())
    }
}

fn main_or_error() -> Result<(), Error> {
    let filename = filename()?;
    let lines = lines(&filename)?;
    
    Ok(())
} 

fn main() {
    match main_or_error() {
        Ok(_) => (),
        Err(Error::MissingArgument) => {
            println!("Please specify input file.");
        },
        Err(Error::FileReadError(name)) => {
            println!("Failed to read from file: {}", name);
        },
    }
}
