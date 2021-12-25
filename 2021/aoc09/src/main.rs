use std::fs::File;
use std::io::{BufReader, BufRead};
use std::env;
use std::collections::{HashMap, HashSet, VecDeque};
use std::cmp::{Eq, PartialEq};

#[derive(Debug, Clone)]
enum Error {
    FileNotFound,
    FileReadError,
    InvalidCharacter(char),
    ArgumentMissing,
    InvalidMap
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
struct Spot {
    x: usize,
    y: usize,
    height: usize,
}

impl Spot {
    #[inline]
    fn risk(&self) -> usize {
        self.height + 1
    }

    fn adjacent_coordinates(&self) -> [(usize, usize); 4] {
        let x = self.x;
        let y = self.y;
        [
            (x.wrapping_add(1), y),
            (x.wrapping_sub(1), y),
            (x, y.wrapping_add(1)),
            (x, y.wrapping_sub(1)),
        ]
    }
}

struct Map {
    width: usize,
    spots: HashMap<(usize,usize),Spot>
}

struct LowPoints<'a> {
    cursor: usize,
    map: &'a Map
}

impl<'a> Iterator for LowPoints<'a> {
    type Item = Spot;

    fn next(&mut self) -> Option<Spot> {
        let width = self.map.width;
        let mut cursor = self.cursor;
        let mut result: Option<Spot> = None;
        while result.is_none() {
            let x = cursor / width;
            let y = cursor % width;
            let s = self.map.get((x,y))?;
            if s.adjacent_coordinates()
                .into_iter()
                .filter_map(|c| self.map.get(c))
                .all(|n| s.height < n.height)
            {
                result = Some(s);
            }
            cursor += 1;
        }
        self.cursor = cursor;
        result
    }
}

impl Map {
    fn read_from(filename: &str) -> Result<Self,Error> {
        let file = File::open(filename).map_err(|_| Error::FileNotFound)?;
        let mut spots = HashMap::new();
        for (x, line) in BufReader::new(file).lines().enumerate() {
            let line = line.map_err(|_| Error::FileReadError)?;
            for (y, c) in line.chars().enumerate() {
                let height = c.to_digit(10).ok_or(Error::InvalidCharacter(c))? as usize;
                spots.insert((x,y), Spot{x,y,height});
            }
        }
        let depth = spots.keys().map(|(x,_)| x).max().copied().unwrap_or(0) + 1;
        let width = spots.keys().map(|(_,y)| y).max().copied().unwrap_or(0) + 1;
        if spots.len() == depth * width {
            Ok(Map{spots, width})
        } else {
            Err(Error::InvalidMap)
        }
    }

    fn lows(&self) -> LowPoints {
        LowPoints{cursor: 0, map: self}
    }

    fn get(&self, t: (usize, usize)) -> Option<Spot> {
        self.spots.get(&t).copied()
    }

    fn basin(&self, center: Spot) -> HashSet<Spot> {
        let mut queue: VecDeque<Spot> = VecDeque::new();
        let mut basin: HashSet<Spot> = HashSet::new();
        queue.push_back(center);
        while let Some(spot) = queue.pop_front() {
            basin.insert(spot);
            queue.extend(spot
                .adjacent_coordinates()
                .into_iter()
                .filter_map(|c| self.get(c))
                .filter(|s| s.height < 9)
                .filter(|s| !basin.contains(s))
            );
        }
        return basin;
    }
}

fn main_or_error() -> Result<(), Error> {
    let file_name = env::args().nth(1).ok_or(Error::ArgumentMissing)?;
    let map = Map::read_from(&file_name)?;
    let lows: Vec<Spot> = map.lows().collect();
    println!("Risk Level: {}", lows.iter().map(|s| s.risk()).sum::<usize>());
    let mut basins: Vec<usize> = lows.iter().map(|&spot| map.basin(spot).len()).collect();
    basins.sort_by(|a, b| b.cmp(a));
    println!("Basin Check: {}", basins.iter().take(3).product::<usize>());
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
