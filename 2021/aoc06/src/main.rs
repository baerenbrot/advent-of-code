use std::fs;
use std::env;

#[derive(Debug, Clone)]
enum Error {
    FileReadError(String),
    ParsingError(String),
    InvalidTurn(usize),
    MissingArgument
}

const SPWAN_TURN: usize = 8;
const RESET_TURN: usize = 6;

fn file_read(filename: &str) -> Result<String, Error> {
    Ok(fs::read_to_string(filename)
        .map_err(|_| Error::FileReadError(String::from(filename)))?)
}

fn file_name() -> Result<String, Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        Err(Error::MissingArgument)
    } else {
        Ok(args[1].clone())
    }
}

type Swarm = [usize; SPWAN_TURN + 1];

fn read_fishes(spec: &String) -> Result<Swarm,Error> {
    let mut counts: Swarm = [0; SPWAN_TURN + 1];
    for _turn in spec.split(',')
        .map(|t| t.trim().parse::<usize>().map_err(|_| Error::ParsingError(t.to_string())))
    {
        let turn = _turn?;
        if turn > SPWAN_TURN {
            return Err(Error::InvalidTurn(turn));
        }
        counts[turn] += 1;
    }
    Ok(counts)
}

fn age(swarm: &mut Swarm) {
    let spawns = swarm[0];
    swarm.rotate_left(1);
    swarm[SPWAN_TURN]  = spawns;
    swarm[RESET_TURN] += spawns;
}

fn main_or_error() -> Result<(), Error> {
    let file_name = file_name()?;
    let fish_data = file_read(&file_name)?;
    let mut swarm = read_fishes(&fish_data)?;

    for _ in 0..80 { age(&mut swarm); }
    println!("Fishes: {}", swarm.iter().sum::<usize>());

    for _ in 80..256 { age(&mut swarm); }
    println!("Fishes: {}", swarm.iter().sum::<usize>());

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
        Err(Error::ParsingError(name)) => {
            println!("Failed to parse entry as integer: {}", name);
        },  
        Err(Error::InvalidTurn(turn)) => {
            println!("Not a valid turn number: {}", turn);
        },
    }
}
