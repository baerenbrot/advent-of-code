mod matrix;
mod vector;

use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::{HashSet, HashMap};

use vector::Vector;
use matrix::Matrix;

const MINIMUM_OVERLAP_FOR_ALIGNMENT: usize = 12;


#[derive(Copy,Clone)]
struct Transformation(Matrix, Vector);


struct Scan {
    blips: HashSet<Vector>,
    edges: HashMap<isize, HashSet<(Vector, Vector)>>,
    transformation: Option<Transformation>
}


#[derive(Debug,Clone)]
enum Error {
    InvalidPoint(String),
    MissingArgument,
    EmptyInput,
    InputFileNotFound,
}


impl Scan {
    fn refresh_edges(&mut self) {
        for &v in self.blips.iter() {
            for &w in self.blips.iter() {
                if v == w { continue; }
                let d = (w - v).abs();
                if !self.edges.contains_key(&d) {
                    self.edges.insert(d, HashSet::new());
                }
                self.edges.get_mut(&d).unwrap().insert((v,w));
            }
        }
    }
}


impl From<HashSet<Vector>> for Scan {
    fn from(blips: HashSet<Vector>) -> Self {
        let mut s = Scan{
            blips, edges: HashMap::new(), transformation: None};
        s.refresh_edges();
        s
    }
}


impl Scan {

    fn align(&mut self, them: &mut Self) -> Option<Transformation> {
        let e1: HashSet<isize> = self.edges.keys().copied().collect();
        let e2: HashSet<isize> = them.edges.keys().copied().collect();
        let mut shared: Vec<isize> = e1.intersection(&e2).copied().collect();
        shared.sort_by(|d1, d2| {
            let n1 = self.edges.get(&d1).unwrap().len();
            let n2 = self.edges.get(&d2).unwrap().len();
            n1.partial_cmp(&n2).unwrap()
        });
        for d in shared.iter() {
            for &(v1, v2) in self.edges.get(&d).unwrap().iter() {
                for &(w1, w2) in them.edges.get(&d).unwrap().iter() {
                    let v = v2 - v1;
                    let w = w2 - w1;
                    if let Some(a) = w.rotates_into(&v) {
                        let t = v1 - a * w1;
                        let h: HashSet<Vector> = them.blips.iter().map(|&w| a * w + t).collect();
                        let n = self.blips.iter().filter(|b| h.contains(b)).count();
                        if n >= MINIMUM_OVERLAP_FOR_ALIGNMENT {
                            h.into_iter().for_each(|b| {self.blips.insert(b);});
                            self.refresh_edges();
                            let t = Some(Transformation(a,t));
                            them.transformation = t;
                            return t;
                        }
                    }
                }
            }
        }
        None
    }

    fn parse(fd: File) -> Result<Vec<Scan>,Error> {
        let reader = BufReader::new(fd);
        let mut blips: HashSet<Vector> = HashSet::new();
        let mut scans: Vec<Scan> = Vec::new();
        let mut lines = reader.lines();
        while let Some(line) = lines.next() {
            let line = line.unwrap();
            let line = line.trim();
            if line.is_empty() {
                continue
            }
            if line.starts_with("---") {
                if !blips.is_empty() {
                    scans.push(Scan::from(blips));
                    blips = HashSet::new();
                }
            } else {
                let blip: Result<Vec<_>,_> = line.split(',').map(|t| t.parse()).collect();
                let blip = blip.map_err(|_| Error::InvalidPoint(String::from(line)))?;
                let blip = Vector{x: blip[0], y: blip[1], z: blip[2]};
                blips.insert(blip);
            }
        }
        if !blips.is_empty() {
            scans.push(Scan::from(blips));
        }
        Ok(scans)
    }

}


fn main_or_error() -> Result<(),Error> {
    let path = std::env::args().nth(1).ok_or(Error::MissingArgument)?;
    let file = File::open(path).map_err(|_| Error::InputFileNotFound)?;
    let mut scans = Scan::parse(file)?;

    let mut scanners: Vec<Vector> = Vec::with_capacity(scans.len());
    scanners.push(Vector::from((0,0,0)));
    
    scans.reverse();
    let mut core = scans.pop().ok_or(Error::EmptyInput)?;
    let mut done = false;

    while !done {
        done = true;
        for scan in scans.iter_mut() {
            if scan.transformation.is_none() {
                done = false;
                if let Some(Transformation(_,t)) = core.align(scan) {
                    scanners.push(t);
                }
            }
        }
    }

    println!("beacon count: {}", core.blips.len());

    let mut max_distance = 0;

    for &a in scanners.iter() {
        for &b in scanners.iter() {
            max_distance = std::cmp::max((b - a).abs(), max_distance);
        }
    }

    println!("max distance: {}", max_distance);

    Ok(())
}


fn main() {
    if let Err(error) = main_or_error() {
        println!("ERROR: {:?}", error);
    }
}
