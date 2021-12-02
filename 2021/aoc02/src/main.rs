use std::fs;
use std::env;

#[derive(Debug, Clone)]
enum Error {
    FileReadError(String),
    InputParseError,
    MissingArgument
}

#[derive(Debug, Copy, Clone)]
enum NavigationCommand {
    Forward(usize),
    Up(usize),
    Down(usize),
}

#[derive(Copy, Clone)]
enum NavigationStrategy {
    Incorrect,
    Correct
}

struct Position {
    aim: usize,
    horizontal: usize,
    depth: usize,
}

fn lines(filename: &str) -> Result<Vec<String>, Error> {
    Ok(fs::read_to_string(filename)
        .map_err(|_| Error::FileReadError(String::from(filename)))?
        .split('\n')
        .map(|x| String::from(x.trim()))
        .filter(|x| x.len() > 0)
        .collect::<Vec<_>>())
}

fn parse(lines: Vec<String>) -> Result<Vec<NavigationCommand>, Error> {
    lines.iter().map(|command| {
        let tokens: Vec<&str> = command.split(' ').collect();
        let amount = tokens[1].parse().map_err(|_| Error::InputParseError)?;
        match tokens[0] {
            "down"    => Ok(NavigationCommand::Down(amount)),
            "up"      => Ok(NavigationCommand::Up(amount)),
            "forward" => Ok(NavigationCommand::Forward(amount)),
            _ => Err(Error::InputParseError)
        }
    }).collect()
}

impl Position {

    fn steer(&mut self, command: &NavigationCommand) -> () {
        match command {
            NavigationCommand::Down(k) => { self.aim += k },
            NavigationCommand::Up(k)   => { self.aim -= k },
            NavigationCommand::Forward(k) => {
                self.horizontal += k;
                self.depth += self.aim * k;
            }
        }
    }

    fn steer_naive(&mut self, command: &NavigationCommand) -> () {
        match command {
            NavigationCommand::Down(k)    => { self.depth += k },
            NavigationCommand::Up(k)      => { self.depth -= k },
            NavigationCommand::Forward(k) => { self.horizontal += k }
        }
    }

    fn navigate(
        &mut self,
        strategy: NavigationStrategy,
        commands: &Vec<NavigationCommand>
    ) -> () {
        let mut steer = |cmd| { match strategy {
            NavigationStrategy::Incorrect => self.steer_naive(cmd),
            NavigationStrategy::Correct => self.steer(cmd)
        }};
        for command in commands { steer(command) }
    }

    fn new() -> Self {
        Position { aim: 0, depth: 0, horizontal: 0 }
    }
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
    let commands = parse(lines)?;
    for strategy in [NavigationStrategy::Incorrect, NavigationStrategy::Correct] {
        let mut position = Position::new();
        position.navigate(strategy, &commands);
        println!("Checksum: {}", position.depth * position.horizontal);
    }
    Ok(())
} 


fn main() {
    match main_or_error() {
        Ok(_) => (),
        Err(Error::MissingArgument) => {
            println!("Please specify input file.");
        },
        Err(Error::InputParseError) => {
            println!("Please specify input file.");
        },
        Err(Error::FileReadError(name)) => {
            println!("Failed to read from file: {}", name);
        },
    }
}
