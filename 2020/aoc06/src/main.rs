use std::collections::HashSet;
use std::fs;
use std::env;

#[derive(Debug, Copy, Clone)]
enum Error {
    FileReadError,
}

fn lines(filename: &str) -> Result<Vec<String>, Error> {
    Ok(fs::read_to_string(filename)
        .map_err(|_| Error::FileReadError)?
        .split('\n')
        .map(String::from)
        .collect::<Vec<_>>())
}

trait IteratorCallback<'a>: Copy {
    type Output: Iterator<Item = &'a char> + 'a;
    fn call(self, a: &'a HashSet<char>, b: &'a HashSet<char>) -> Self::Output;
}

impl<'a, F, T> IteratorCallback<'a> for F
where
    F: Fn(&'a HashSet<char>, &'a HashSet<char>) -> T,
    F: Copy,
    T: 'a,
    T: Iterator<Item = &'a char>,
{
    type Output = T;
    fn call(self, a: &'a HashSet<char>, b: &'a HashSet<char>) -> T {
        self(a, b)
    }
}

fn apply<O>(lines: &[String], operation: O) -> usize
where
    O: for<'a> IteratorCallback<'a>,
{
    let mut counter = 0;
    let mut accumulator: HashSet<char> = HashSet::new();
    let mut group_started = false;
    for line in lines {
        if line.is_empty() {
            counter += accumulator.len();
            accumulator.clear();
            group_started = false;
        } else if group_started {
            let mut answers = HashSet::new();
            line.chars().for_each(|c| {
                answers.insert(c);
            });
            accumulator = operation.call(&accumulator, &answers).copied().collect();
        } else {
            line.chars().for_each(|c| {
                accumulator.insert(c);
            });
            group_started = true;
        }
    }
    counter + accumulator.len()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please specify input file.");
    } else if let Ok(lines) = lines(&args[1]) {
        println!("Union: {}", apply(&lines, HashSet::union));
        println!("Intersection: {}", apply(&lines, HashSet::intersection));
    } else {
        println!("Failed to read from file: {}", args[1]);
    }
}
