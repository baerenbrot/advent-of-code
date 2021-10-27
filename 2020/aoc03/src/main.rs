use std::collections::HashSet;
use std::io::Lines;
use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;

#[derive(Debug, Copy, Clone)]
enum Error {
    FileReadError,
    FileFormatError,
    InconsistentLineLengths
}

fn lines(filename: &str) -> Result<Lines<BufReader<File>>, Error> {
    Ok(BufReader::new(File::open(filename).map_err(|_| Error::FileReadError)?).lines())
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum SquareType {
    Open,
    Tree
}

struct Area {
    map: Vec<Vec<SquareType>>
}

impl Area {
    fn new(filename: &str) -> Result<Self, Error> {

        fn parse_entry(c: char) -> Result<SquareType, Error> {
            match c {
                '.' => Ok(SquareType::Open),
                '#' => Ok(SquareType::Tree),
                 _  => Err(Error::FileFormatError)
            }
        }

        fn parse_line(line: String) -> Result<Vec<SquareType>, Error> {
            line.chars()
                .filter(|&c| c != '\n')
                .map(parse_entry)
                .collect()
        }

        let map = lines(filename)?
            .map(|line| parse_line(line.map_err(|_|Error::FileReadError)?))
            .collect::<Result<Vec<Vec<SquareType>>, Error>>()?;

        let lengths: HashSet<usize> = map
            .iter()
            .map(|line| line.len())
            .collect();

        if lengths.len() != 1 {
            Err(Error::InconsistentLineLengths)
        } else {
            Ok(Area{map})
        }
    }

    fn count_trees(&self, right: usize, down: usize) -> usize {
        let mut latitude: usize = 0;
        let mut longitude: usize = 0;
        let mut treecount: usize = 0;
        while longitude < self.map.len() {
            let contour = &self.map[longitude];
            latitude = latitude % contour.len();
            if contour[latitude] == SquareType::Tree {
                treecount += 1;
            }
            longitude += down;
            latitude += right;
        }
        treecount
    }
}

fn main() {
    let area = Area::new("input.txt").unwrap();
    let mut checksum: usize = 1;

    for (right,down) in [(1,1),(3,1),(5,1),(7,1),(1,2)] {
        checksum *= area.count_trees(right, down)
    }

    println!("trees on 3/1 path: {}", area.count_trees(3, 1));
    println!("tree checksum: {}", checksum);
}
