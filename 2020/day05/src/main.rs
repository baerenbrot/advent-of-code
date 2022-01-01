use std::io::Lines;
use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;

#[derive(Debug, Copy, Clone)]
enum Error {
    FileReadError,
    FileFormatError,
}

fn lines(filename: &str) -> Result<Lines<BufReader<File>>, Error> {
    Ok(BufReader::new(File::open(filename).map_err(|_| Error::FileReadError)?).lines())
}

fn boarding_pass_ids(filename: &str) -> Result<Vec<usize>, Error> {
    Ok(lines(filename)?
        .map(|x| x.map_err(|_| Error::FileFormatError))
        .filter_map(|x| x.ok())
        .map(|x| x.replace("F","0").replace("B","1").replace("L","0").replace("R","1"))
        .map(|x| usize::from_str_radix(&x, 2).unwrap())
        .collect())
}

fn main() {
    let mut ids = boarding_pass_ids("input.txt").unwrap();
    
    ids.sort();
    ids.reverse();

    let mut iter = ids.iter();

    if let Some(max) = iter.next() {
        println!("Max ID: {}", max);
        let mut previous = max;
        for id in iter {
            if *id == previous - 2 {
                println!("My ID: {}", id + 1);
                break;
            } else {
                previous = id;
            }
        }
    }
}
