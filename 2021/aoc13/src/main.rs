use std::env::args;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufReader,BufRead};
use regex::Regex;

#[derive(Clone,Debug)]
enum Error {
    ArgumentMissing,
    FormatError,
    InvalidFold(String),
    ReadError,
    FileMissing,
    ImpossibleFold(Dot, Fold),
    NoFoldRemaining,
}

#[derive(Clone,Debug)]
enum Axis {X=0,Y=1}

#[derive(Clone,Debug)]
struct Fold {
    axis: Axis,
    offset: usize
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Dot {
    y: usize,
    x: usize,
}

impl Fold {
    fn new(definition: &str) -> Result<Self,Error> {
        let pattern = Regex::new(r"^fold along (y|x)=(\d{1,9})\s*$").unwrap();
        let capture = pattern.captures(definition).ok_or(Error::InvalidFold(definition.to_string()))?;
        let offset: usize = capture[2].parse().unwrap();
        match capture[1].chars().next().unwrap() {
            'x' => Ok(Fold{axis: Axis::X, offset}),
            'y' => Ok(Fold{axis: Axis::Y, offset}),
            ___ => panic!()
        }
    }

    fn apply(&self, dot: &Dot) -> Result<Dot,Error> {
        let x = dot.x;
        let y = dot.y;
        let t = self.offset;
        match self.axis {
            Axis::X if x > t => Ok(Dot{x:2*t-x,y}),
            Axis::Y if y > t => Ok(Dot{x,y:2*t-y}),
            Axis::X if x < t => Ok(*dot),
            Axis::Y if y < t => Ok(*dot),
            _ => Err(Error::ImpossibleFold(dot.clone(), self.clone()))
        }
    }
}

impl Dot {
    fn new(definition: &str) -> Result<Self,Error> {
        let pattern = Regex::new(r"^(\d{1,9}),\s*(\d{1,9})\s*$").unwrap();
        let capture = pattern.captures(definition).ok_or(Error::FormatError)?;
        let x = capture[1].parse().unwrap();
        let y = capture[2].parse().unwrap();
        Ok(Dot{x,y})
    }
}

struct Instructions {
    dots: HashSet<Dot>,
    folds: Vec<Fold>
}

impl Instructions {
    fn new(path: &str) -> Result<Self,Error> {
        let mut dots: HashSet<Dot> = HashSet::new();
        let mut folds: Vec<Fold> = Vec::new();
        let file = File::open(path).map_err(|_| Error::FileMissing)?;
        let definition: Vec<_> = BufReader::new(file).lines()
            .map(|line| line.map_err(|_| Error::ReadError)).collect::<Result<_,_>>()?;
        let mut it = definition.iter();
        while let Some(line) = it.next() {
            if line.is_empty() { break; }
            dots.insert(Dot::new(line)?);
        }
        while let Some(line) = it.next() {
            folds.push(Fold::new(line)?);
        }
        if folds.is_empty() {
            Err(Error::FormatError)
        } else {
            folds.reverse();
            Ok(Instructions{dots,folds})
        }
    }

    fn fold_one(&mut self) -> Result<(),Error> {
        if let Some(fold) = self.folds.pop() {
            self.dots = self.dots.iter().map(|d| fold.apply(d)).collect::<Result<HashSet<_>,_>>()?;
            Ok(())
        } else {
            Err(Error::NoFoldRemaining)
        }
    }

    fn fold_all(&mut self) -> Result<(),Error> {
        while !self.folds.is_empty() {
            self.fold_one()?;
        }
        Ok(())
    }

    fn print(&self) -> String {
        let x_max = self.dots.iter().map(|d| d.x).max().unwrap();
        let y_max = self.dots.iter().map(|d| d.y).max().unwrap();
        let mut representation = String::new();
        for y in 0..=y_max {
            for x in 0..=x_max {
                representation.push(if self.dots.contains(&Dot{x,y}) {'#'} else {' '});
            }
            if y != y_max {
                representation.push('\n');
            }
        }
        representation
    }
}

fn main_or_error() -> Result<(),Error> {
    let path = args().nth(1).ok_or(Error::ArgumentMissing)?;
    let mut instructions = Instructions::new(&path)?;
    instructions.fold_one()?;
    println!("First Fold Dot Count: {}", instructions.dots.len());
    instructions.fold_all()?;
    println!("After Folding:\n{}", instructions.print());
    Ok(())
}

fn main() {
    match main_or_error() {
        Ok(()) => {},
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}
