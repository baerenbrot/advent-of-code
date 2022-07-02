use std::borrow::Borrow;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::rc::Rc;
use std::cell::{Ref,RefMut,RefCell};
use std::env;
use std::fmt;
use std::str::Chars;
use std::ops;


#[derive(Debug, Clone)]
enum Error {
    FileNotFound,
    FileReadError,
    ExpectedComma(char),
    ExpectedClosingBracket(char),
    InvalidCharacter(char),
    UnexpectedEndOfLine,
    InputIsEmpty,
    UnexpectedRegularNumber,
    ArgumentMissing,
}

#[derive(Clone)]
enum Edge {
    Leaf(usize),
    Pair(Node),
}

#[derive(PartialEq, Copy, Clone)]
enum Direction { East, West }

impl Direction {
    #[inline]
    fn reverse(self) -> Self {
        match self {
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }
}

type NodeDataRef = Rc<RefCell<NodeData>>;

struct Node(NodeDataRef);

#[derive(Clone)]
struct NodeData {
    west: Edge,
    east: Edge,
}

struct NodeReader<'a> {
    iter: Chars<'a>,
}

#[derive(PartialEq, Copy, Clone, Debug)]
enum ShockWave {
    None,
    Caught,
    Full(usize, usize),
    West(usize),
    East(usize),
}

impl<'a> Node {

    #[inline]
    fn unwrap(&'a self) -> &'a Rc<RefCell<NodeData>> {
        let Node(reference) = self;
        reference
    }

    #[inline]
    fn borrow(&self) -> Ref<'_, NodeData> {
        RefCell::borrow(self.unwrap())
    }

    #[inline]
    fn borrow_mut(&self) -> RefMut<'_, NodeData> {
        self.unwrap().borrow_mut()
    }

    fn new(west: Edge, east: Edge) -> Self {
        Node(Rc::new(RefCell::new(NodeData{west,east})))
    }

    fn read(expression: &str) -> Result<Self, Error> {
        let reader = NodeReader::new(expression);
        match reader.read_node()? {
            Edge::Pair(b) => Ok(b),
            Edge::Leaf(_) => Err(Error::UnexpectedRegularNumber),
        }
    }

    #[inline]
    fn distribute_west(&self, value: usize, out: ShockWave) -> ShockWave {
        self.distribute(Direction::West, value, true);
        out
    }

    #[inline]
    fn distribute_east(&self, value: usize, out: ShockWave) -> ShockWave {
        self.distribute(Direction::East, value, true);
        out
    }

    fn distribute(&self, whence: Direction, value: usize, curve: bool) {
        let mut borrowed = self.borrow_mut();
        match match whence {
            Direction::West => &borrowed.west,
            Direction::East => &borrowed.east,
        } {
            Edge::Pair(p) => p.distribute(
                if curve { whence.reverse() } else { whence }, value, false),
            Edge::Leaf(x) => {
                match whence {
                    Direction::West => {borrowed.west = Edge::Leaf(x + value)}
                    Direction::East => {borrowed.east = Edge::Leaf(x + value)}
                };
            }
        }
    }

    fn magnitude(&self) -> usize {
        let borrowed = self.borrow();
        let a = match &borrowed.west { &Edge::Leaf(x) => x, Edge::Pair(r) => r.magnitude() };
        let b = match &borrowed.east { &Edge::Leaf(x) => x, Edge::Pair(r) => r.magnitude() };
        3*a + 2*b
    }

    fn split(&self) -> bool {
        let mut borrowed = self.borrow_mut();
        let split = |value: usize| {
            let (x, y) = (value / 2, value / 2 + value % 2);
            Edge::Pair(Node::new(Edge::Leaf(x), Edge::Leaf(y)))
        };
        if match &borrowed.west {
            Edge::Pair(w) => w.split(),
            Edge::Leaf(x) => {
                if *x > 9 {
                    borrowed.west = split(*x);
                    true
                } else {
                    false
                }
            }
        } {
            true
        } else { match &borrowed.east {
            Edge::Pair(e) => e.split(),
            Edge::Leaf(y) => {
                if *y > 9 {
                    borrowed.east = split(*y);
                    true
                } else {
                    false
                }
            }
        }}
    }

    fn explode(&self, depth: usize) -> ShockWave {
        let mut origin = Direction::West;
        let mut shockwave: ShockWave;
        {
            let borrowed = self.borrow();
            shockwave = match &borrowed.west {
                Edge::Pair(l) => l.explode(depth + 1),
                Edge::Leaf(x) => {
                    if let Edge::Leaf(y) = &borrowed.east {
                        if depth >= 4 {
                            return ShockWave::Full(*x, *y)
                        }
                    }
                    ShockWave::None
                }
            };
            if shockwave == ShockWave::None {
                if let Edge::Pair(r) = &borrowed.east {
                    origin = Direction::East;
                    shockwave = r.explode(depth + 1);
                }
            }
        }

        if let ShockWave::Full(_, _) = shockwave {
            let mut borrowed = self.borrow_mut();
            match origin {
                Direction::West => { borrowed.west = Edge::Leaf(0) },
                Direction::East => { borrowed.east = Edge::Leaf(0) },
            };
        }

        match shockwave {
            ShockWave::Full(x, y) => match origin {
                Direction::East => self.distribute_west(x, ShockWave::East(y)),
                Direction::West => self.distribute_east(y, ShockWave::West(x)),
            },
            ShockWave::West(x) => match origin {
                Direction::West => shockwave,
                Direction::East => self.distribute_west(x, ShockWave::Caught)
            }
            ShockWave::East(y) => match origin {
                Direction::East => shockwave,
                Direction::West => self.distribute_east(y, ShockWave::Caught)
            }, _ => shockwave
        }
    }

    fn reduce(self) -> Self {
        while self.explode(0) != ShockWave::None || self.split() {}
        self
    }

}

impl ops::AddAssign for Node {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.clone() + rhs
    }
}

impl ops::Add for &Node {
    type Output = Node;
    fn add(self, rhs: Self) -> Self::Output {
        Node::new(Edge::Pair(self.clone()), Edge::Pair(rhs.clone())).reduce()
    }
}

impl ops::Add for Node {
    type Output = Node;
    fn add(self, rhs: Self) -> Self::Output {
        Node::new(Edge::Pair(self), Edge::Pair(rhs)).reduce()
    }
}

impl Clone for Node {
    fn clone(&self) -> Self {
        let borrowed = self.borrow();
        Node::new(borrowed.west.clone(), borrowed.east.clone())
    }
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        match self {
            &Edge::Leaf(x) => match other { &Edge::Leaf(y) => x == y, _ => false },
             Edge::Pair(p) => match other {  Edge::Pair(q) => p == q, _ => false },
        }
    }
}

impl PartialEq for Node {
    fn eq(&self, them: &Self) -> bool {
        let a = self.borrow();
        let b = them.borrow();
        a.west == b.west && a.east == b.east
    }
}

impl<'a> NodeReader<'a> {
   
    fn new(string: &'a str) -> Self {
        NodeReader { iter: string.chars() }
    }
    
    fn _read_char(mut self) -> Result<(Self, char), Error> {
        self.iter.next().ok_or(Error::UnexpectedEndOfLine).map(|c| (self, c))
    }

    fn _read_pair(self) -> Result<(Self, Node), Error> {
        let s = self;
        let (s, lhs) = s._read_node()?;
        let (s, sep) = s._read_char()?;
        if sep != ',' {
            return Err(Error::ExpectedComma(sep));
        }
        let (s, rhs) = s._read_node()?;
        let (s, end) = s._read_char()?;
        if end != ']' {
            return Err(Error::ExpectedClosingBracket(end));
        }
        Ok((s, Node::new(lhs, rhs)))
    }

    fn _read_node(self) -> Result<(Self, Edge), Error> {
        let s = self;
        let (s, character) = s._read_char()?;
        match character {
            '[' => s._read_pair().map(|(s,p)| (s, Edge::Pair(p))),
            '0'..='9' => Ok((s, Edge::Leaf(character.to_digit(10).unwrap() as usize))),
            _ => Err(Error::InvalidCharacter(character))
        }
    }

    fn read_node(self) -> Result<Edge, Error> {
        Ok(self._read_node()?.1)
    }

}

impl fmt::Debug for Edge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Edge::Pair(node) => node.borrow().fmt(f),
            Edge::Leaf(n) => fmt::Debug::fmt(n, f)
        }
    }
}

impl fmt::Debug for NodeData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let borrowed = self.borrow();
        write!(f, "[{:?},{:?}]", borrowed.west, borrowed.east)
    }
}

fn node_sum(numbers: &Vec<Node>) -> Result<Node, Error> {
    let mut iter = numbers.iter().cloned();
    let mut total = iter.next().ok_or(Error::InputIsEmpty)?;
    for node in iter { total = total + node; }
    Ok(total)
}

fn maximum_sum(numbers: &Vec<Node>) -> Result<Node, Error> {
    let mut best_num: Option<Node> = None;
    let mut best_mag: usize = 0;
    for a in numbers.iter() {
        for b in numbers.iter() {
            if a == b {
                continue;
            }
            let sum = a + b;
            let mag = sum.magnitude();
            if mag > best_mag {
                best_num = Some(sum);
                best_mag = mag;
            }
        }
    }
    best_num.ok_or(Error::InputIsEmpty)
}


#[test]
fn test_simple_sums() {
    let test = (1..=4).map(|k| Node::new(Edge::Leaf(k), Edge::Leaf(k))).collect();
    let test = node_sum(&test);
    let goal = Node::read("[[[[1,1],[2,2]],[3,3]],[4,4]]");
    assert!(test.is_ok() && goal.is_ok() && test.unwrap() == goal.unwrap());

    let test = (1..=5).map(|k| Node::new(Edge::Leaf(k), Edge::Leaf(k))).collect();
    let test = node_sum(&test);
    let goal = Node::read("[[[[3,0],[5,3]],[4,4]],[5,5]]");
    assert!(test.is_ok() && goal.is_ok() && test.unwrap() == goal.unwrap());

    let test = (1..=6).map(|k| Node::new(Edge::Leaf(k), Edge::Leaf(k))).collect();
    let test = node_sum(&test);
    let goal = Node::read("[[[[5,0],[7,4]],[5,5]],[6,6]]");
    assert!(test.is_ok() && goal.is_ok() && test.unwrap() == goal.unwrap());
}

#[test]
fn test_magnitude() {
    assert!(Node::read("[[1,2],[[3,4],5]]").unwrap().magnitude() == 143);
    assert!(Node::read("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").unwrap().magnitude() == 1384);
    assert!(Node::read("[[[[1,1],[2,2]],[3,3]],[4,4]]").unwrap().magnitude() == 445);
    assert!(Node::read("[[[[3,0],[5,3]],[4,4]],[5,5]]").unwrap().magnitude() == 791);
    assert!(Node::read("[[[[5,0],[7,4]],[5,5]],[6,6]]").unwrap().magnitude() == 1137);
    assert!(Node::read("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]").unwrap().magnitude() == 3488);
}

#[test]
fn test_slightly_larger() {
    fn _read() -> Result<Node, Error> {
        let mut x: Node;
        x  = Node::read("[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]")?;
        x += Node::read("[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]")?;
        x += Node::read("[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]")?;
        x += Node::read("[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]")?;
        x += Node::read("[7,[5,[[3,8],[1,4]]]]")?;
        x += Node::read("[[2,[2,2]],[8,[8,1]]]")?;
        x += Node::read("[2,9]")?;
        x += Node::read("[1,[[[9,3],9],[[9,0],[0,7]]]]")?;
        x += Node::read("[[[5,[7,4]],7],1]")?;
        x += Node::read("[[[[4,2],2],6],[8,7]]")?;
        Ok(x)
    }
    let x = _read();
    let y = Node::read("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]");
    assert!(x.is_ok() && y.is_ok() && x.unwrap() == y.unwrap());
}

#[test]
fn test_example_homework_assignment() {
    fn _read() -> Result<Node, Error> {
        let mut x: Node;
        x  = Node::read("[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]")?;
        x += Node::read("[[[5,[2,8]],4],[5,[[9,9],0]]]")?;
        x += Node::read("[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]")?;
        x += Node::read("[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]")?;
        x += Node::read("[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]")?;
        x += Node::read("[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]")?;
        x += Node::read("[[[[5,4],[7,7]],8],[[8,3],8]]")?;
        x += Node::read("[[9,3],[[9,9],[6,[4,9]]]]")?;
        x += Node::read("[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]")?;
        x += Node::read("[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]")?;
        Ok(x)
    }
    let x = _read();
    let y = Node::read("[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]");
    assert!(x.is_ok() && y.is_ok());
    let x = x.unwrap();
    let y = y.unwrap();
    assert!(x == y);
    assert!(x.magnitude() == 4140);
}


fn main_or_error() -> Result<(), Error> {

    let path = env::args().nth(1).ok_or(Error::ArgumentMissing)?;
    let file = File::open(path).map_err(|_| Error::FileNotFound)?;

    let lines: Result<Vec<String>, _> = BufReader::new(file).lines().collect();
    let lines = lines.map_err(|_| Error::FileReadError)?;

    let pairs: Result<Vec<Node>, _> = lines.iter().map(|s| Node::read(s)).collect();
    let pairs = pairs?;
    let total = node_sum(&pairs)?;
    let best = maximum_sum(&pairs)?;

    println!("sum all : {:?}", total.borrow());
    println!("best num: {:?}", best.borrow());
    println!("best mag: {:?}", best.magnitude());
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
