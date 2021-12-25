use std::fs::File;
use std::io::{BufReader, BufRead};
use std::env;
use std::cmp::{Eq, PartialEq};

#[derive(Debug, Clone)]
enum Error {
    FileNotFound,
    FileReadError,
    ArgumentMissing,
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
enum ChunkType {
    Round = 1,
    Square = 2,
    Curly = 3,
    Pointy = 4,
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
enum Chunk { Open(ChunkType), Close(ChunkType) }

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
enum SyntaxCheckResult {
    Corrupt(ChunkType),
    Incomplete(Vec<ChunkType>),
    Valid
}

struct Line(Vec<Chunk>);

struct Input {
    lines: Vec<Line>
}

impl SyntaxCheckResult {
    fn score_errors(&self) -> usize {
        match self {
            SyntaxCheckResult::Corrupt(ChunkType::Round)  => 3,
            SyntaxCheckResult::Corrupt(ChunkType::Square) => 57,
            SyntaxCheckResult::Corrupt(ChunkType::Curly)  => 1197,
            SyntaxCheckResult::Corrupt(ChunkType::Pointy) => 25137,
            _ => 0
        }
    }
    fn score_syntax(&self) -> usize {
        if let SyntaxCheckResult::Incomplete(v) = self {
            v.iter().fold(0, |score, &t| score * 5 + (t as usize))
        } else { 0 }
    }
}

impl Line {
    fn new(input: &str) -> Self {
        Line(input.chars().filter_map(|c| match c {
            '(' => Some(Chunk::Open(ChunkType::Round)),
            '[' => Some(Chunk::Open(ChunkType::Square)),
            '{' => Some(Chunk::Open(ChunkType::Curly)),
            '<' => Some(Chunk::Open(ChunkType::Pointy)),
            ')' => Some(Chunk::Close(ChunkType::Round)),
            ']' => Some(Chunk::Close(ChunkType::Square)),
            '}' => Some(Chunk::Close(ChunkType::Curly)),
            '>' => Some(Chunk::Close(ChunkType::Pointy)),
             _  => None
        }).collect())
    }

    fn check(&self) -> SyntaxCheckResult {
        let mut stack: Vec<ChunkType> = Vec::new();
        let Line(chunks) = self;
        for chunk in chunks.iter().copied() {
            match chunk {
                Chunk::Open(t) => {
                    stack.push(t);
                }
                Chunk::Close(t) => {
                    if stack.pop() != Some(t) {
                        return SyntaxCheckResult::Corrupt(t)
                    }
                }
            }
        }
        if stack.is_empty() {
            SyntaxCheckResult::Valid
        } else {
            stack.reverse();
            SyntaxCheckResult::Incomplete(stack)
        }
    }
}

impl Input {
    fn read_from(filename: &str) -> Result<Self,Error> {
        let file = File::open(filename).map_err(|_| Error::FileNotFound)?;
        let mut lines: Vec<Line> = Vec::new();
        for line in BufReader::new(file).lines() {
            let line = line.map_err(|_| Error::FileReadError)?;
            lines.push(Line::new(&line));
        }
        Ok(Input{lines})
    }
}

fn main_or_error() -> Result<(), Error> {
    let file_name = env::args().nth(1).ok_or(Error::ArgumentMissing)?;
    let input = Input::read_from(&file_name)?;   
    println!("Score for Errors: {}",
        input.lines.iter().map(|l| l.check().score_errors()).sum::<usize>());
    let mut scores: Vec<_> = input.lines.iter()
        .map(|l| l.check().score_syntax()).filter(|&t| t > 0).collect();
    scores.sort();
    println!("Score for Syntax: {}", scores[scores.len() / 2]);
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
