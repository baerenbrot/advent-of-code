use std::fs;
use std::env;


#[derive(Debug, Clone)]
enum Error {
    FileReadError(String),
    MissingArgument,
    NeverEntersTheBasement,
    InvalidCharacter(char)
}

fn fread(filename: &str) -> Result<String, Error> {
    fs::read_to_string(filename)
        .map_err(|_| Error::FileReadError(String::from(filename)))
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
    let mut floors = fread(&filename)?;
    floors = floors.trim().to_string();

    println!("Final Floor: {}", 
        floors.chars().try_fold(0, |current, direction|
            match direction {
                '(' => Ok(current + 1),
                ')' => Ok(current - 1),
                err => Err(Error::InvalidCharacter(err))
            }
        )?);

    println!("First Basement Step: {}",
        floors.chars()
            .scan(0, |floor, direction| {
                match direction {
                    '(' => {*floor += 1},
                    ')' => {*floor -= 1},
                     _  => {return None},
                };
                Some(*floor)
            })
            .enumerate()
            .find(|(_, floor)| *floor < 0)
            .ok_or(Error::NeverEntersTheBasement)?.0 + 1
        );

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
        Err(Error::NeverEntersTheBasement) => {
            println!("Error: Basement is never reached.");
        },
        Err(Error::InvalidCharacter(c)) => {
            println!("Encountered an invalid character: {}.", c);
        }
    }
}
