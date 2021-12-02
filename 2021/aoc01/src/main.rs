use std::fs;
use std::env;
use itertools::Itertools;

#[derive(Debug, Copy, Clone)]
enum Error {
    FileReadError,
    InvalidInputError
}

fn lines(filename: &str) -> Result<Vec<String>, Error> {
    Ok(fs::read_to_string(filename)
        .map_err(|_| Error::FileReadError)?
        .split('\n')
        .map(|x| String::from(x.trim()))
        .filter(|x| x.len() > 0)
        .collect::<Vec<_>>())
}

fn parse(lines: &Vec<String>) -> Result<Vec<usize>, Error> {
    lines.iter().map(|x| x.parse().map_err(|_| Error::InvalidInputError)).collect()
}

fn count<I: Iterator<Item=usize>>(measurements: I) -> usize {
    measurements.tuple_windows().filter(|(a,b)| a < b).count()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please specify input file.");
    } else if let Ok(lines) = lines(&args[1]) {
        if let Ok(parsed) = parse(&lines) {
            println!("Result 1: {}", count(parsed.iter().copied()));
            println!("Result 2: {}", count(parsed.into_iter().tuple_windows::<(_,_,_)>().map(|(a,b,c)| a + b + c)));
        } else {
            println!("Invalid file format; expected line-wise integer values.");
        }
    } else {
        println!("Failed to read from file: {}", args[1]);
    }
}
