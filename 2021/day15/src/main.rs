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
    IncompleteMap,
    NoPathFound,
    FileMissing,
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

struct Cave {
    map: DiGraph<Node,u32>,
    source: NodeIndex,
    target: NodeIndex,
}

impl Cave {
    fn read(file_name: &str) -> Result<Self,Error> {
        let file = File::open(file_name).or(Err(Error::FileMissing))?;
        let mut map: DiGraph<Node,u32> = DiGraph::new();
        let mut risks: HashMap<Point,NodeIndex> = HashMap::new();

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
                    risks.insert(spot, map.add_node(node));
                }
            );
        }

        let (source, target) = 
            risks.keys().map(|&n| n.x).max().and_then(|x| {
            risks.keys().map(|&n| n.y).max().and_then(|y| {
                let width = x+1;
                let depth = y+1;
                if width * depth != risks.len() { // sanity check
                    None
                } else {
                    risks.get(&Point{x: 0, y: 0}).and_then(|&source| {
                    risks.get(&Point{x: x, y: y}).and_then(|&target| {
                            Some((source, target))
                    })})
                }
            })}).ok_or(Error::IncompleteMap)?;

        risks.iter().for_each(|(&pt, &v)| {
            for (x,y) in [
                (pt.x.wrapping_sub(1), pt.y),
                (pt.x, pt.y.wrapping_sub(1)),
                (pt.x.wrapping_add(1), pt.y),
                (pt.x, pt.y.wrapping_add(1)),
            ] {
                risks.get(&Point{x,y}).map(|&w| {
                    map.add_edge(v, w, map[w].risk);
                    map.add_edge(w, v, map[v].risk);
                });
            }
        });

        Ok(Cave{map,source,target})
    }

    fn navigate(&self) -> Option<(u32,Vec<NodeIndex>)> {
        astar(&self.map, self.source, |v| v == self.target, |e| *e.weight(), |_| 0)
    }
}

fn main_or_error() -> Result<(),Error> {
    let file_name = args().nth(1).ok_or(Error::ArgumentMissing)?;
    let (cost, _) = Cave::read(&file_name)?.navigate().ok_or(Error::NoPathFound)?;
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
