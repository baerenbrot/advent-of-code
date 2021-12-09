use std::fs;
use std::env;
use std::cmp::Eq;
use std::convert::TryInto;
use std::collections::HashSet;
use regex::Regex;
use itertools::Itertools;

#[derive(Clone, Debug)]
enum Error {
    ArgumentMissing,
    InputFileMissing,
    InvalidFormat(String),
    InvalidWire(char),
    CouldNotRewire,
    WiringStillBroken
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Wire {
    A, B, C, D, E, F, G
}

impl From<Wire> for usize {
    #[inline]
    fn from(t: Wire) -> usize { t as usize }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Signal(u8);

type Wiring = [Wire; 7];

struct Signals(Vec<Signal>);

struct SignalIterator {
    signal: u8,
    offset: u8
}

impl Iterator for SignalIterator {
    type Item = Wire;

    fn next(&mut self) -> Option<Wire> {
        let mut offset = self.offset;
        while offset <= 6 {
            self.offset += 1;
            if (self.signal >> offset) & 1 == 1 {
                break;
            }
            offset = self.offset;
        }
        match offset {
            0 => Some(Wire::A),
            1 => Some(Wire::B),
            2 => Some(Wire::C),
            3 => Some(Wire::D),
            4 => Some(Wire::E),
            5 => Some(Wire::F),
            6 => Some(Wire::G),
            _ => None
        }        
    }
}

impl Signal {
    fn new(wires: &HashSet<Wire>) -> Self {
        Signal(wires.iter().fold(0, |a, &w| a | (1 << usize::from(w))))
    }

    fn iter(&self) -> SignalIterator {
        let &Signal(s) = self;
        SignalIterator { signal: s, offset: 0 }
    }

    fn rewire(&self, wiring: Wiring) -> Self {
        let r: Vec<Wire> = self.iter().collect();
        Signal::new(&r.iter().map(|&w| wiring[usize::from(w)]).collect())
    }

    fn display(&self) -> Result<usize, Error> {
        SIGNAL_DEFAULTS
            .into_iter()
            .enumerate()
            .filter_map(|(k, s)| if s == *self { Some(k) } else { None })
            .next()
            .ok_or(Error::WiringStillBroken)
    }
}

impl Signals {
    fn new(signals: &str) -> Result<Self, Error> {
        let space = Regex::new(r"\s+").unwrap();
        let signals: Vec<HashSet<Wire>> = space.split(signals.trim())
            .map(|signal| signal.chars().map(|c| match c {
                'a' => Ok(Wire::A),
                'b' => Ok(Wire::B),
                'c' => Ok(Wire::C),
                'd' => Ok(Wire::D),
                'e' => Ok(Wire::E),
                'f' => Ok(Wire::F),
                'g' => Ok(Wire::G),
                _  => Err(Error::InvalidWire(c))
            }).collect::<Result<_,_>>()).collect::<Result<_,_>>()?;
        Ok(Signals(signals.iter().map(|t| Signal::new(t)).collect()))
    }
}

struct BrokenScreen {
    signals: [Signal; 0xA],
    display: [Signal; 0x4]
}

const SIGNAL_DEFAULTS: [Signal;10] = [
    Signal(0b1110111),
    Signal(0b0100100),
    Signal(0b1011101),
    Signal(0b1101101),
    Signal(0b0101110),
    Signal(0b1101011),
    Signal(0b1111011),
    Signal(0b0100101),
    Signal(0b1111111),
    Signal(0b1101111),
];


impl BrokenScreen {
    fn new(encoded: &str) -> Result<Self, Error> {
        let err = || Error::InvalidFormat(encoded.to_string());
        let mut parts = encoded.split('|');
        let mut read = || {Signals::new(parts.next().ok_or(err())?)};
        let Signals(signals) = read()?;
        let Signals(display) = read()?;
        Ok(BrokenScreen {
            signals: signals.try_into().map_err(|_| err())?,
            display: display.try_into().map_err(|_| err())?,
        })
    }

    fn is_valid(&self, wiring: Wiring) -> bool {
        let rewired: HashSet<Signal> = self.signals.iter().map(|signal| signal.rewire(wiring)).collect();
        rewired == HashSet::from_iter(SIGNAL_DEFAULTS)
    }

    fn rewire(&self) -> Option<Wiring> {
        let all = [Wire::A,Wire::B,Wire::C,Wire::D,Wire::E,Wire::F,Wire::G];
        for permutation in all.into_iter().permutations(7) {
            let wiring: Wiring = permutation.try_into().unwrap();
            if self.is_valid(wiring) { return Some(wiring); }
        }
        None
    }
}

fn main_or_error() -> Result<(), Error> {
    let file_name = env::args().nth(1).ok_or(Error::ArgumentMissing)?;
    let file_data = fs::read_to_string(&file_name)
        .map_err(|_| Error::InputFileMissing)?;
    let line_breaks = Regex::new(r"\s*\n\s*").unwrap();

    let mut part1sum = 0;
    let mut part2sum = 0;

    for (k, line) in line_breaks.split(&file_data.trim()).enumerate() {
        let screen = BrokenScreen::new(line)?;
        let wiring = screen.rewire().ok_or(Error::CouldNotRewire)?;
        let display: Vec<usize> = screen.display
            .iter().map(|t| t.rewire(wiring).display()).collect::<Result<_,_>>()?;
        part2sum += display.iter().copied().fold(0, |a, d| a * 10 + d);
        part1sum += display.iter().copied()
            .filter(|&t| t == 1 || t == 4 || t == 7 || t == 8).count();
        println!("Display Digits {:3}: {}", k,
            display.iter().map(|t| t.to_string()).join("-"));
    }

    println!("Part 1: {}", part1sum);
    println!("Part 2: {}", part2sum);

    Ok(())
}


fn main() {
    match main_or_error() {
        Ok(()) => {},
        Err(Error::ArgumentMissing) => {
            println!("ArgumentMissing!");
        },
        Err(Error::InputFileMissing) => {
            println!("InputFileMissing!");
        },
        Err(Error::InvalidFormat(definition)) => {
            println!("Invalid Format: {}", definition);
        },
        Err(Error::InvalidWire(_)) => {
            println!("InvalidWire!");
        },
        Err(Error::CouldNotRewire) => {
            println!("CouldNotRewire!");
        },
        Err(Error::WiringStillBroken) => {
            println!("WiringStillBroken!");
        },
    }
}
