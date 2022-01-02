use std::env::args;
use petgraph::graph::{NodeIndex, DiGraph};
use petgraph::algo::astar;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader,BufRead};

#[derive(Clone,Debug)]
enum Error {
    ArgumentMissing,
    ReadError,
    NoPathFound,
    FileMissing,
    InvalidArgument,
    ZeroScale,
    NonSquareMap,
    InvalidState,
    InvalidCharacter(char),
}

#[derive(Clone,Copy,Hash,PartialEq,Eq,PartialOrd,Ord)]
struct Point{
    x: usize,
    y: usize,
}

#[derive(Clone,Copy,Hash,PartialEq,Eq,PartialOrd,Ord)]
struct Node {
    spot: Point,
    risk: u32
}

struct NavigationalSystem {
    map: DiGraph<Node,u32>,
    who: HashMap<Point,NodeIndex>,
}

impl NavigationalSystem {
    fn read(path: &str) -> Result<Self,Error> {
        let file = File::open(path).or(Err(Error::FileMissing))?;
        let mut map: DiGraph<Node,u32> = DiGraph::new();
        let mut who: HashMap<Point,NodeIndex> = HashMap::new();
        let chr = |c: char| c.to_digit(10).ok_or(Error::InvalidCharacter(c));
        for (y,row) in BufReader::new(file).lines().enumerate() {
            row.or(Err(Error::ReadError))?
                .trim().chars().map(chr)
                .collect::<Result<Vec<_>,_>>()?
                .into_iter()
                .enumerate()
                .for_each(|(x,risk)| {
                    let spot = Point{x,y};
                    let node = Node{spot,risk};
                    who.insert(spot, map.add_node(node));
                }
            );
        }
        NavigationalSystem{map,who}.scanned()
    }

    fn source(&self) -> Result<NodeIndex,Error> {
        self.node(0,0)
    }

    fn target(&self) -> Result<NodeIndex,Error> {
        let (width,depth) = self.dimensions()?;
        self.node(width-1,depth-1)
    }

    fn node(&self, x:usize, y:usize) -> Result<NodeIndex,Error> {
        self.who.get(&Point{x:x,y:y}).copied().ok_or(Error::InvalidState)
    }

    fn dimensions(&self) -> Result<(usize,usize),Error> {
        self.who.keys().max().ok_or(Error::InvalidState).and_then(|&Point{x,y}| {
            let width = x+1;
            let depth = y+1;
            if width * depth != self.who.len() {
                Err(Error::NonSquareMap)
            } else {
                Ok((width, depth))
            }
        })
    }

    fn navigate(&self) -> Result<(u32,Vec<NodeIndex>),Error> {
        let target = self.target()?;
        let source = self.source()?;
        astar(&self.map, source, |node| node == target, |edge| *edge.weight(), |_| 0)
            .ok_or(Error::NoPathFound)
    }

    fn scanned(mut self) -> Result<Self,Error> {
        self.who.iter().for_each(|(&pt, &v)| {
            for (x,y) in [
                (pt.x.wrapping_sub(1), pt.y),
                (pt.x, pt.y.wrapping_sub(1)),
                (pt.x.wrapping_add(1), pt.y),
                (pt.x, pt.y.wrapping_add(1)),
            ] {
                self.who.get(&Point{x,y}).map(|&w| {
                    self.map.update_edge(v, w, self.map[w].risk);
                    self.map.update_edge(w, v, self.map[v].risk);
                });
            }
        });
        Ok(self)
    }

    fn scaled(mut self, scale: usize) -> Result<Self,Error> {
        let (width, depth) = self.dimensions()?;
        if scale < 1 {
            return Err(Error::ZeroScale);
        } else if scale > 1 {
            for dx in 0..scale {
                for dy in 0..scale {
                    if (dx,dy) == (0,0) {
                        continue;
                    }
                    let x_offset = dx * width;
                    let y_offset = dy * depth;
                    for x in 0..width {
                        for y in 0..depth {
                            let risk = self.map[self.node(x,y)?].risk;
                            let risk = risk + (dx as u32) + (dy as u32);
                            let risk = risk - 1;
                            let risk = risk % 9;
                            let risk = risk + 1;
                            let spot = Point{x:x+x_offset,y:y+y_offset};
                            let node = Node{spot,risk};
                            self.who.insert(spot, self.map.add_node(node));
                        }
                    }
                }
            }
        }
        self.scanned()
    }
}

fn main_or_error() -> Result<(),Error> {
    let path = args().nth(1).ok_or(Error::ArgumentMissing)?;
    let scale: usize = args().nth(2).map(|p| p.parse()).unwrap_or(Ok(1)).or(Err(Error::InvalidArgument))?;
    let (cost, _) = NavigationalSystem::read(&path)?.scaled(scale)?.navigate()?;
    println!("Cost: {}", cost);
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
