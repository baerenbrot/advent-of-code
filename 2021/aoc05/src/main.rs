use std::fs;
use std::env;
use num::rational::Rational64;
use std::collections::HashSet;
use std::collections::HashMap;

#[derive(Debug, Clone)]
enum Error {
    FileReadError(String),
    MissingArgument,
    PointParsingError(String),
    LineParsingError(String),
}

fn lines(filename: &str) -> Result<Vec<String>, Error> {
    Ok(fs::read_to_string(filename)
        .map_err(|_| Error::FileReadError(String::from(filename)))?
        .split('\n')
        .map(|x| String::from(x.trim()))
        .filter(|x| x.len() > 0)
        .collect::<Vec<_>>())
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> where T: From<i64> {
    fn new(spec: &str) -> Result<Self, Error> {
        let parsed: Option<Vec<i64>> = spec.trim().split(',').map(|x| x.parse().ok()).collect();
        if let Some(entries) = parsed {
            if entries.len() == 2 {
                return Ok(Point {
                    x: T::from(entries[0]),
                    y: T::from(entries[1]),
                })
            }
        }
        Err(Error::PointParsingError(spec.to_string()))
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum Line {
    Sloped(Rational64, Rational64),
    Vertical(i64)
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct LineSegment(Point<i64>, Point<i64>);

struct LineSegmentIterator<T>
{
    line: Line,
    point: Option<Point<T>>,
    end: Point<T>,
}

impl From<&LineSegment> for Line {
    fn from(segment: &LineSegment) -> Line {
        let LineSegment(p, q) = segment;
        match p.x - q.x {
            0 => Line::Vertical(p.x),
            d => {
                let a = Rational64::new(p.y - q.y, d);
                let b = Rational64::new(p.x * q.y - p.y * q.x, d);
                Line::Sloped(a, b)
            }
        }
    }
}

impl<T> std::fmt::Display for Point<T>
    where T: std::fmt::Display
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl std::fmt::Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Line::Vertical(x) => write!(f, "<x = {}>", x),
            Line::Sloped(a,b) => if *a.numer() == 0 {
                write!(f, "<x = {}>", b)
            } else {
                write!(f, "<y = {} * x + {}>", a, b)
            }
        }
    }
}

impl LineSegment {
    fn new(spec: &str) -> Result<Self, Error> {
        let points: Vec<&str> = spec.split("->").collect();
        if points.len() != 2 {
            Err(Error::LineParsingError(spec.to_string()))
        } else {
            let p = Point::new(points[0])?;
            let q = Point::new(points[1])?;
            let mut segment = LineSegment(p, q);
            if match Line::from(&segment) {
                Line::Vertical(_) => p.y > q.y,
                Line::Sloped(_,_) => p.x > q.x,
            } {
                segment = LineSegment(q, p);
            }
            Ok(segment)
        }
    }

    fn iter(&self) -> LineSegmentIterator<i64> {
        LineSegmentIterator {
            line: Line::from(self),
            point: Some(self.0),
            end: self.1
        }
    }
}

impl Iterator for LineSegmentIterator<i64>
{
    type Item = Point<i64>;

    fn next(&mut self) -> Option<Point<i64>> {
        let q = self.end;
        let p = self.point?;
        self.point = match self.line {
            Line::Vertical(x) => {
                if p.y < q.y { Some(Point{x: x, y: p.y + 1}) } else { None }
            },
            Line::Sloped(a,b) => {
                let mut next = None;
                for x in p.x+1..=q.x {
                    let y = a * x + b;
                    if y.is_integer() {
                        next = Some(Point{x: x, y: y.to_integer()});
                        break;
                    }
                }
                next
            }
        };
        Some(p)
    }
}

impl Line {
    fn is_on_grid(&self) -> bool {
        match self {
            Line::Vertical(_) => true,
            Line::Sloped(a,_) => *a.numer() == 0
        }
    }
}

struct OceanFloor {
    clouds: HashSet<LineSegment>
}

impl OceanFloor {
    fn new(lines: Vec<String>) -> Result<Self, Error> {
        Ok(OceanFloor{ clouds: lines
            .into_iter()
            .map(|x| LineSegment::new(&x))
            .collect::<Result<HashSet<_>, Error>>()?
        })
    }

    fn restrict_to_grid(mut self) -> Self {
        self.clouds = self.clouds.into_iter().filter(|line| {
            Line::from(line).is_on_grid()
        }).collect();
        self
    }

    fn count_hotspots(&self, minimum: usize) -> usize {
        let mut coverage: HashMap<Point<i64>, usize> = HashMap::new();
        for cloud in self.clouds.iter() {
            for point in cloud.iter() {
                coverage.insert(point, coverage.get(&point).copied().unwrap_or(0) + 1);
            }
        }
        coverage.into_values().filter(|&t| t >= minimum).count()
    }

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
    let full = OceanFloor::new(lines)?;
    println!("Full Intersection Count: {}", full.count_hotspots(2));
    let grid = full.restrict_to_grid();
    println!("Grid Intersection Count: {}", grid.count_hotspots(2));
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
        Err(Error::PointParsingError(spec)) => {
            println!("Failed to parse point: {}", spec);
        },
        Err(Error::LineParsingError(spec)) => {
            println!("Failed to parse line: {}", spec);
        }
    }
}
