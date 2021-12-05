use std::fs;
use std::env;
use std::ops::Index;
use regex::Regex;

#[derive(Debug, Clone)]
enum Error {
    FileReadError(String),
    MissingArgument,
    EmptyInput,
    NotASquare(usize),
    NoWinner
}

#[derive(Copy, Clone, Debug)]
struct Square(usize, bool);

#[derive(Clone, Debug)]
struct Board {
    columns: usize,
    squares: Vec<Square>,
    won: bool,
}

#[derive(Clone, Debug)]
struct Game {
    boards: Vec<Board>,
    _input: Vec<usize>,
    _round: usize,
}

impl Square {
    fn new(value: usize) -> Self {
        Square(value, false)
    }
    fn mark(&mut self, value: usize) -> () {
        if value == self.0 { self.1 = true; }
    }
    #[inline]
    fn marked(&self) -> bool { self.1 }
}

impl From<&Square> for usize {
    fn from(item: &Square) -> usize {item.0} 
}

impl Index<(usize, usize)> for Board {
    type Output = Square;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (row, col) = index;
        if row >= self.columns || col >= self.columns { panic!() }
        &self.squares[row * self.columns + col]
    }
}

impl Board {
    fn new(grid: &str) -> Result<Self, Error> {
        let number_pattern = Regex::new(r"\b\d+\b").unwrap();
        let lines: Vec<&str> = grid.split('\n')
            .map(|s| s.trim())
            .filter(|s| s.len() > 0)
            .collect();
        let columns = lines.len();
        if columns == 0 {
            return Err(Error::EmptyInput);
        }
        let grid: Vec<Vec<usize>> = lines.iter().map(|line|
            number_pattern.find_iter(line).map(|nr| nr.as_str().parse().unwrap()).collect::<Vec<_>>()
        ).collect();
        if grid.iter().any(|v| v.len() != columns) {
            return Err(Error::NotASquare(columns));
        }
        Ok(Board {
            columns: columns,
            squares: grid.into_iter().flatten().map(|v| Square::new(v)).collect(),
            won: false
        })
    }

    fn winning(&mut self) -> bool {
        if self.won {
            false 
        } else {
            self.won = (0..self.columns).any(
                |anchor| {
                    (0..self.columns).map(|k| self[(anchor, k)]).all(|square| square.marked()) ||
                    (0..self.columns).map(|k| self[(k, anchor)]).all(|square| square.marked())
                }
            );
            self.won
        }
    }

    fn play(&mut self, value: usize) -> &mut Self {
        for square in &mut self.squares { square.mark(value) }
        self
    }
}

struct Win<'a> {
    input: usize,
    board: &'a Board
}

impl<'a> Win<'a> {
    fn score(&'a self) -> usize {
        self.input * self.board.squares.iter().fold(0, |a, x| if x.marked() {a} else {a + usize::from(x)})
    }
}


impl Game {
    fn new(input: String) -> Result<Self, Error> {
        let paragraph_separator = Regex::new(r"\n\s*\n").unwrap();
        let digits = Regex::new(r"\b\d+\b").unwrap();
        let mut paragraphs = paragraph_separator.split(&input);
        let input_values_string = paragraphs.next().ok_or(Error::EmptyInput)?;
        Ok(Game {
            _input: digits.find_iter(input_values_string).map(|nr| nr.as_str().parse().unwrap()).collect(),
            _round: 0,
            boards: paragraphs.map(|spec| Board::new(spec)).collect::<Result<_,_>>()?
        })
    }

    fn play_round(&mut self) -> Option<Win> {
        let n = self.boards.len();
        let m = self._input.len();
        while self._round < m {
            let input = self._input[self._round];
            for i in 0..n {
                if !self.boards[i].won {
                    self.boards[i].play(input);
                }
            }
            for i in 0..n {
                if self.boards[i].winning() {
                    return Some(Win{board: &self.boards[i], input: input});
                }
            }
            self._round += 1;
        }
        None
    }
}

impl Iterator for Game {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        self.play_round().map(|win| win.score())
    }
}


fn fread(filename: &str) -> Result<String, Error> {
    fs::read_to_string(filename)
        .map_err(|_| Error::FileReadError(String::from(filename)))
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
    let mut game = Game::new(fread(&filename)?)?;
    if let Some(score) = game.next() {
        println!("First Score: {}", score);
    } else {
        return Err(Error::NoWinner)
    }
    if let Some(score) = game.last() {
        println!("Final Score: {}", score);
    }
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
        Err(Error::EmptyInput) => {
            println!("Error: Empty input.");
        },
        Err(Error::NotASquare(c)) => {
            println!("Board was not a {}×{} square.", c, c);
        },
        Err(Error::NoWinner) => {
            println!("Noone won!");
        }
    }
}
