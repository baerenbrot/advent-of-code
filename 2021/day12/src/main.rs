use std::env::args;
use petgraph::graph::{NodeIndex, UnGraph};
use std::collections::{HashMap};
use std::fs::File;
use std::io::{BufReader,BufRead};
use itertools::Itertools;
use itertools::FoldWhile;

#[derive(Clone,Debug)]
enum Error {
    ArgumentMissing,
    InvalidArgument,
    FormatError,
    ReadError,
    NodeMissing,
    InfiniteLoopDetected,
    FileMissing
}

struct Node {
    name: String,
    large: bool,
}

struct Cave {
    map: UnGraph<Node, ()>,
    source: NodeIndex,
    target: NodeIndex,
}

impl Node {
    fn new(name: &str) -> Self {
        let name = name.to_string();
        let large = name.chars().all(|c| c.is_ascii_uppercase() || !c.is_ascii());
        Node{name,large}
    }
}

impl Cave {
    fn read(file_name: &str) -> Result<Self,Error> {
        let file = File::open(file_name).map_err(|_| Error::FileMissing)?;
        let map: UnGraph<Node,()> = UnGraph::new_undirected();
        let mut who: HashMap<String, NodeIndex> = HashMap::new();
        let mut add_node = |name: String, mut map: UnGraph<Node,()>| {
            let index = match who.get(&name) {
                Some(&index) => index,
                None => map.add_node(Node::new(&name))
            };
            who.insert(name, index);
            (index, map)
        };
        let lines: Result<Vec<_>,_> = BufReader::new(file).lines().map(|line| {
            let line = line.map_err(|_| Error::ReadError)?;
            let edge: Vec<&str> = line.trim().split('-').collect();
            let a = edge.get(0).ok_or(Error::FormatError)?.to_string();
            let b = edge.get(1).ok_or(Error::FormatError)?.to_string();
            Ok((a,b))
        }).collect();
        if let FoldWhile::Continue(map) = lines?.into_iter().fold_while(map, |map, (a,b)| {
            let (a, map) = add_node(a, map);
            let (b, map) = add_node(b, map);
            let mut map = map;
            map.add_edge(a, b, ());
            if map[a].large && map[b].large {
                FoldWhile::Done(map)
            } else {
                FoldWhile::Continue(map)
            }
        }) {
            if let (Some(&source), Some(&target)) = (who.get("start"), who.get("end")) {
                Ok(Cave{map,source,target})
            } else {
                Err(Error::NodeMissing)
            }
        } else {
            Err(Error::InfiniteLoopDetected)
        }
    }

    fn count_paths(&mut self, revisit_count: usize, print: bool) -> usize {
        let mut pending: Vec<(usize,Vec<NodeIndex>)> = vec![(0,vec![self.source])];
        let mut count: usize = 0;
        while pending.len() > 0 {
            let (revisits, path) = pending.pop().unwrap();
            for next in self.map.neighbors_undirected(path[path.len() - 1]) {
                if next == self.source {
                    continue;
                }
                let revisits = if !self.map[next].large && path.contains(&next) {
                    revisits + 1
                } else {
                    revisits
                };
                if revisits <= revisit_count {
                    let mut path = path.clone();
                    path.push(next);
                    if next == self.target {
                        count += 1;
                        if print {
                            println!("{}", path.iter().map(|&k| self.map[k].name.clone()).join("-"));
                        }
                    } else {
                        pending.push((revisits, path));
                    }
                }
            }
        }
        return count;
    }
}

fn main_or_error() -> Result<(),Error> {
    let file_name = args().nth(1).ok_or(Error::ArgumentMissing)?;
    let revisits: usize = args().nth(2).map(|p| p.parse()).unwrap_or(Ok(0)).map_err(|_| Error::InvalidArgument)?;
    let mut cave = Cave::read(&file_name)?;
    println!("Path Count: {}", cave.count_paths(revisits, false));
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
