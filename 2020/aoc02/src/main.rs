use std::io;
use std::io::BufRead;
use std::fs::File;
use std::path::Path;
use regex::Regex;

fn lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug)]
enum Error {
    FileReadError,
    ParsingError
}

#[derive(Debug, Copy, Clone)]
enum PasswordValidationPolicy {
    SledRentalPlace,
    OfficialToboggan
}

struct PasswordEntry {
    min_count: usize,
    max_count: usize,
    character: char,
    password : String,
}

impl PasswordEntry {
    fn new(line: String) -> Option<PasswordEntry> {
        Regex::new("(\\d+)-(\\d+)\\s(.):\\s(.*)")
            .ok()?
            .captures(&line)
            .map(|captures| PasswordEntry {
                min_count: captures[1].parse().unwrap(),
                max_count: captures[2].parse().unwrap(),
                character: captures[3].chars().next().unwrap(),
                password : captures[4].to_string()
            })
    }

    fn char_count(&self) -> usize {
        self.password
            .chars()
            .filter(|&x| x == self.character)
            .count()
    }
 
    fn valid(&self, policy: PasswordValidationPolicy) -> bool {
        match policy {
            PasswordValidationPolicy::OfficialToboggan => {
                (self.password.chars().nth(self.min_count-1).unwrap() == self.character) ^ 
                (self.password.chars().nth(self.max_count-1).unwrap() == self.character)
            },
            PasswordValidationPolicy::SledRentalPlace => {
                let cc = self.char_count();
                self.min_count <= cc && cc <= self.max_count
            }
        }
    }
}

fn get_valid_password_count(path: &str, policy: PasswordValidationPolicy) -> Result<usize, Error> {
    let mut counter: usize = 0;
    for line in lines(path).map_err(|_| Error::FileReadError)? {
        let entry = PasswordEntry::new(line.map_err(|_| Error::FileReadError)?);
        if entry.ok_or(Error::ParsingError)?.valid(policy) {
            counter += 1;
        }
    }
    Ok(counter)
}


fn main() {
    println!("policy 1: {}", get_valid_password_count(
        "input.txt", PasswordValidationPolicy::SledRentalPlace).unwrap());
    println!("policy 2: {}", get_valid_password_count(
        "input.txt", PasswordValidationPolicy::OfficialToboggan).unwrap());
}
