use std::fs;
use std::env;

#[derive(Debug, Clone)]
enum Error {
    FileReadError(String),
    MissingArgument,
    InvalidBoxFormat(String),
}

#[derive(Debug, Copy, Clone)]
struct Box {
    length: usize,
    width: usize,
    height: usize,
}

impl Box {
    fn new(spec: &str) -> Result<Box, Error> {
        let parsed: Option<Vec<usize>> = spec
            .split('x').map(|p| p.parse().ok()).collect();
        if let Some(p) = parsed {
            if p.len() == 3 {
                return Ok(Box { length: p[0], width: p[1], height: p[2] })
            }
        }
        Err(Error::InvalidBoxFormat(spec.to_string()))
    }

    fn ribbon(&self) -> usize {
        let sides = [self.length, self.width, self.height];
        let largest = sides.iter().max().unwrap();
        let girth = sides.iter().sum::<usize>() - largest;
        let bow = sides.iter().product::<usize>();
        2 * girth + bow
    }

    fn wrapping(&self) -> usize {
        let l = self.length;
        let w = self.width;
        let h = self.height;
        let sides = [l*w, w*h, h*l];
        let slack = sides.iter().min().unwrap();
        2 * sides.iter().sum::<usize>() + slack
    }
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
    let boxes = lines.iter().map(|p| Box::new(p)).collect::<Result<Vec<_>,_>>()?;
    println!("Required wrapping: {}",
        boxes.iter().map(|b| b.wrapping()).sum::<usize>());
    println!("Required ribbons : {}",
        boxes.iter().map(|b| b.ribbon()).sum::<usize>());
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
        Err(Error::InvalidBoxFormat(s)) => {
            println!("This box has an unknown format: {}", s);
        },
    }
}
