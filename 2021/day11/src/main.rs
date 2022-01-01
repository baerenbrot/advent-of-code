use std::fs::File;
use std::io::{BufReader, BufRead};
use std::env;
use std::collections::{HashMap, HashSet};
use std::cmp::{Eq, PartialEq};

#[derive(Debug, Clone)]
enum Error {
    FileNotFound,
    FileReadError,
    InvalidCharacter(char),
    ArgumentMissing,
    InvalidMap
}

const MAX_ENERGY: usize = 9;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
struct Octopus(usize);

type Coordinate = (usize, usize);

struct Map {
    width: usize,
    depth: usize,
    spots: HashMap<Coordinate,Octopus>
}

impl Map {
    fn read_from(filename: &str) -> Result<Self,Error> {
        let file = File::open(filename).map_err(|_| Error::FileNotFound)?;
        let mut spots = HashMap::new();
        for (y, line) in BufReader::new(file).lines().enumerate() {
            let line = line.map_err(|_| Error::FileReadError)?;
            for (x, c) in line.chars().enumerate() {
                let energy = c.to_digit(10).ok_or(Error::InvalidCharacter(c))? as usize;
                spots.insert((x,y), Octopus(energy));
            }
        }
        let width = spots.keys().map(|&(x,_)| x).max().unwrap_or(0) + 1;
        let depth = spots.keys().map(|&(_,y)| y).max().unwrap_or(0) + 1;
        if spots.len() == depth * width {
            Ok(Map{spots, width, depth})
        } else {
            Err(Error::InvalidMap)
        }
    }

    #[inline]
    fn get(&self, c: Coordinate) -> Option<&Octopus> {
        self.spots.get(&c)
    }

    #[inline]
    fn set(&mut self, c: Coordinate, energy: usize) -> bool {
        self.spots.insert(c, Octopus(energy));
        energy > MAX_ENERGY
    }

    #[inline]
    fn size(&self) -> usize {
        self.width * self.depth
    }

    fn step(&mut self) -> usize {
        let mut flashing: Vec<Coordinate> = Vec::new();
        let mut exhausted: HashSet<Coordinate> = HashSet::new();

        for y in 0..self.depth {
            for x in 0..self.width {
                let c = (x,y);
                if let Some(&Octopus(energy)) = self.get(c) {
                    if self.set(c, energy + 1) {
                        flashing.push(c);
                        exhausted.insert(c);
                    }
                }
            }
        }

        while let Some(center) = flashing.pop() {
            let x = center.0;
            let y = center.1;
            for c in [
                (x.wrapping_add(1), y),
                (x.wrapping_sub(1), y),
                (x, y.wrapping_add(1)),
                (x, y.wrapping_sub(1)),
                (x.wrapping_add(1), y.wrapping_add(1)),
                (x.wrapping_sub(1), y.wrapping_add(1)),
                (x.wrapping_add(1), y.wrapping_sub(1)),
                (x.wrapping_sub(1), y.wrapping_sub(1)),
            ] {
                if c.0 <= self.width && c.1 <= self.depth && !exhausted.contains(&c) {
                    if let Some(&Octopus(energy)) = self.get(c) {
                        if self.set(c, energy + 1) {
                            flashing.push(c);
                            exhausted.insert(c);
                        }
                    }
                }
            }
        }
        let result = exhausted.len();

        for c in exhausted {
            self.set(c, 0);
        }

        result
    }
}

fn main_or_error() -> Result<(), Error> {
    let file_name = env::args().nth(1).ok_or(Error::ArgumentMissing)?;   
    let step_count: usize = env::args().nth(2).map(|t| t.parse::<usize>()).unwrap_or(Ok(100)).unwrap();
    let mut map = Map::read_from(&file_name)?;
    let mut count: usize = 0;
    let mut synchronized: bool = false;

    for k in 1.. {
        let flashes = map.step();
        if k <= step_count {
            count += flashes;
            if k == step_count {
                println!("Flashes after {}: {}", k, count);
            }
        } else if synchronized {
            break
        }
        if !synchronized && flashes == map.size() {
            println!("Synchronization achieved after {} steps.", k);
            synchronized = true;
        }
    }
    Ok(())
}


fn main() {
    match main_or_error() {
        Ok(()) => {},
        Err(e) => {
            println!("Error: {:?}.", e);
        }
    }
}
