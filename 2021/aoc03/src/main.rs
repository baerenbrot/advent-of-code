use std::fs;
use std::env;

#[derive(Debug, Clone)]
enum Error {
    FileReadError(String),
    MissingArgument,
    InvalidDigit(char),
    ConversionFailed,
    BalancedBitCount(usize),
    InvalidLeftover(usize),
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

#[derive(Copy, Clone)]
enum LifeSupportDiagnostic {
    OxygenGenertorRating,
    CO2ScrubberRating,
}

fn life_support(
    what: LifeSupportDiagnostic,
    diagnostics: &Vec<String>
) -> Result<usize, Error> {
    let mut workspace: Vec<&String> = diagnostics.iter().collect();
    let (direction, default) = match what {
        LifeSupportDiagnostic::OxygenGenertorRating => (1, '1'),
        LifeSupportDiagnostic::CO2ScrubberRating => (-1, '0'),
    };
    'outer: for column in 0.. {
        let mut counter: isize = 0;
        for entry in &workspace {
            if let Some(digit) = entry.chars().nth(column) { 
                counter = match digit {
                    '0' => Ok(counter - direction),
                    '1' => Ok(counter + direction),
                    c => Err(Error::InvalidDigit(c))
                }?;
            } else {
                break 'outer;    
            }
        }
        let bit = match counter {
            t if t > 0 => '1',
            t if t < 0 => '0',
            _ => default
        };
        workspace = workspace
            .into_iter()
            .filter(|&entry| entry.chars().nth(column) == Some(bit))
            .collect();
        if workspace.len() <= 1 {
            break;
        }
    }

    match workspace.len() {
        1 => usize::from_str_radix(workspace.iter().next().unwrap(), 2)
            .map_err(|_| Error::ConversionFailed),
        t => Err(Error::InvalidLeftover(t))
    }
}

fn performance(diagnostics: &Vec<String>) -> Result<usize, Error> {
    let mut gamma: usize = 0;
    for column in 0.. {
        let mut counter: isize = 0;
        for entry in diagnostics {
            if let Some(digit) = entry.chars().nth(column) { 
                counter = match digit {
                    '0' => Ok(counter - 1),
                    '1' => Ok(counter + 1),
                    c => Err(Error::InvalidDigit(c))
                }?;
            } else {
                let epsilon = !gamma & ((1 << column) - 1); 
                return Ok(gamma * epsilon);
            }
        }
        if counter == 0 {
            return Err(Error::BalancedBitCount(column));
        }
        let bit = if counter > 0 {1} else {0};
        gamma = (gamma << 1) | bit;
    }
    panic!("Control flow left infinite loop unexpectedly.");
}


fn main_or_error() -> Result<(), Error> {
    let filename = filename()?;
    let lines = lines(&filename)?;
    let performance = performance(&lines)?;
    let o2 = life_support(LifeSupportDiagnostic::OxygenGenertorRating, &lines)?;
    let co2 = life_support(LifeSupportDiagnostic::CO2ScrubberRating, &lines)?;
    println!("Diagnostics: {}", performance);
    println!("LifeSupport: {}", o2 * co2);
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
        Err(Error::InvalidDigit(c)) => {
            println!("Invalid digit found: {}", c);
        },
        Err(Error::BalancedBitCount(c)) => {
            println!("Unexpectdly balanced bit count in column {}", c);
        },
        Err(Error::ConversionFailed) => {
            println!("Unable to convert bit string to integer.");
        },
        Err(Error::InvalidLeftover(c)) => {
            println!("A total of {} values remained after filtering.", c);
        }
    }
}
