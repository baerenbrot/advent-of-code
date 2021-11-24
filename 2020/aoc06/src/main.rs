use std::{collections::{HashSet, hash_set::Intersection}, fs};

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

fn union(lines: &[String]) -> usize {
    let mut counter = 0;
    let mut accumulator: HashSet<char> = HashSet::new();
    for line in lines {
	if line.is_empty() {
	    counter += accumulator.len();
	    accumulator.clear();
	} else {
	    line.chars().for_each(|c| {accumulator.insert(c);});
	}
    }
    counter + accumulator.len()
}

fn intersection<'a, F, T>(lines: &[String], operation: F) -> usize
where F: Fn(&'a HashSet<char>, &'a HashSet<char>) -> T,
      T: Iterator<Item=&'a char>
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
	    line.chars().for_each(|c| {answers.insert(c);});
	    // accumulator = operation(&accumulator, &answers)
	    accumulator = HashSet::intersection(&accumulator, &answers)
		.copied()
		.collect();
	} else {
	    line.chars().for_each(|c| {accumulator.insert(c);});
	    group_started = true;
	}
    }
    counter + accumulator.len()
}

fn main() {
    if let Ok(lines) = lines("input.txt") {
	println!("Union: {}", union(&lines));
	println!("Intersection: {}", intersection(&lines, HashSet::intersection));	
    }
}
