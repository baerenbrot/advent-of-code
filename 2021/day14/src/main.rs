use std::env::args;
use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
use std::io::{BufReader,BufRead};
use regex::Regex;
use itertools::Itertools;

#[derive(Clone,Debug)]
enum Error {
    ArgumentMissing,
    InvalidInsertion(String),
    InvalidArgumentFormat,
    ReadError,
    FileMissing,
}

#[derive(Clone,Debug)]
struct Counter<T>(HashMap<T,u64>);

impl<T> Counter<T> where T: Hash + Eq {
    fn new() -> Self { Counter(HashMap::new()) }
    #[inline]
    fn get(&self, key: &T) -> u64 { self.0.get(key).copied().unwrap_or(0) }
    #[inline]
    fn set(&mut self, key: T, value: u64) { self.0.insert(key, value); }
    #[inline]
    fn add(&mut self, key: T, value: u64) { 
        if value > 0 {
            let new = self.get(&key) + value;
            self.set(key, new);
        }
    }
    #[inline]
    fn sub(&mut self, key: T, value: u64) { 
        if value > 0 {
            let new = self.get(&key) - value;
            self.set(key, new);
        }
    }
    #[inline]
    fn inc(&mut self, key: T) { self.add(key, 1) }

    fn iter(&self) -> impl Iterator<Item=(&T,&u64)> {
        self.0.iter()
    }
    fn max(&self) -> u64 {
        self.0.values().copied().max().unwrap_or(0)
    }
    fn min(&self) -> u64 {
        self.0.values().copied().min().unwrap_or(0)
    }
    fn sum(&self) -> u64 {
        self.0.values().sum()
    }
}

type Sequence = (u8,u8);

#[derive(Clone,Copy,Debug,Hash,Eq,PartialEq)]
struct Process {
    marker: Sequence,
    link: u8,
}

#[derive(Clone,Debug)]
struct Polymer {
    sequences: Counter<Sequence>,
    molecules: Counter<u8>,
    processes: HashMap<Sequence,Process>,
}

impl Process {
    fn new(definition: &str) -> Result<Self,Error> {
        let pattern = Regex::new(r"^([A-Z])([A-Z])\s*->\s*([A-Z])$").unwrap();
        let capture = pattern.captures(definition)
            .ok_or(Error::InvalidInsertion(definition.to_string()))?;
        let convert = |k: usize| capture[k].chars().next().unwrap() as u8;
        Ok(Process{marker: (convert(1), convert(2)), link: convert(3)})
    }
    #[inline]
    fn m1(&self) -> (u8,u8) { (self.marker.0, self.link) }
    #[inline]
    fn m2(&self) -> (u8,u8) { (self.link, self.marker.1) }
}

impl Polymer {
    fn new(path: &str) -> Result<Self,Error> {
        let file = File::open(path).map_err(|_| Error::FileMissing)?;
        let mut iter = BufReader::new(file).lines();
        let mut read = || {
            iter.next().ok_or(Error::ReadError)?.map_err(|_| Error::ReadError)
        };
        let mut processes: HashMap<Sequence,Process> = HashMap::new();
        let template: Vec<u8> = read()?.trim().chars().map(|c| c as u8).collect();
        while let Ok(line) = read() {
            let line = line.trim();
            if line.len() > 0 {
                let process = Process::new(line)?;
                processes.insert(process.marker, process);
            }
        }
        let mut sequences: Counter<Sequence> = Counter::new();
        let mut molecules: Counter<u8> = Counter::new();
        template.iter().copied()
            .map(|k: u8| {molecules.inc(k); k})
            .tuple_windows()
            .for_each(|s: Sequence| {sequences.inc(s);});
        Ok(Polymer{sequences,molecules,processes})
    }

    fn mutate_once(&mut self) {
        let scan: Counter<Process> = Counter(self.processes.values()
            .map(|&p| (p, self.sequences.get(&p.marker))).collect::<HashMap<_,_>>());
        for (process, &count) in scan.iter() {
            self.molecules.add(process.link, count);
            self.sequences.add(process.m1(), count);
            self.sequences.add(process.m2(), count);
        }
        for (process, &count) in scan.iter() {
            self.sequences.sub(process.marker, count);
        }
    }

    fn mutate(&mut self, age: usize) {
        for _ in 0..age {
            self.mutate_once();
        }
    }

    fn checksum(&self) -> u64 {
        self.molecules.max() - self.molecules.min()
    }

    fn len(&self) -> u64 {
        self.molecules.sum()
    }
}

fn main_or_error() -> Result<(),Error> {
    let path = args().nth(1).ok_or(Error::ArgumentMissing)?;
    let time = args().nth(2).map(|t| t.parse()
        .map_err(|_| Error::InvalidArgumentFormat)).unwrap_or(Ok(1))?;
    let mut polymer = Polymer::new(&path)?;
    polymer.mutate(time);
    println!("Length after {}: {}", time, polymer.len());
    println!("Checksum: {}", polymer.checksum());
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
