use regex::Regex;
use std::{
    collections::HashMap,
    fs::{self, File},
    io::{BufRead, BufReader, Lines},
};

#[derive(Debug, Copy, Clone)]
enum Error {
    FileReadError,
    RegexError,
}

fn lines(filename: &str) -> Result<Lines<BufReader<File>>, Error> {
    Ok(BufReader::new(File::open(filename).map_err(|_| Error::FileReadError)?).lines())
}

struct PassportData {
    fields: HashMap<String, String>,
}

impl PassportData {
    fn new(data: &str) -> Self {
        PassportData {
            fields: Regex::new(r"(?P<key>[a-z]{3}):(?P<value>[^ \n]+)")
                .map_err(|x| Error::RegexError)
                .unwrap()
                .captures_iter(&data)
                .map(|c| (String::from(&c["key"]), String::from(&c["value"])))
                .collect(),
        }
    }

    fn is_valid(&self) -> bool {
        let required_fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
        required_fields
            .iter()
            .all(|&key| self.fields.contains_key(key))
    }
}

fn read_passport_data(filename: &str) -> Result<Vec<PassportData>, Error> {
    let file_contents = fs::read_to_string(filename).map_err(|_| Error::FileReadError)?;
    Ok(Regex::new(r"\n\s*\n")
        .unwrap()
        .split(&file_contents)
        .map(PassportData::new)
        .collect())
}

fn get_valid_passport_count(filename: &str) -> Result<usize, Error> {
    Ok((read_passport_data(filename)?)
        .iter()
        .filter(|data| data.is_valid())
        .count())
}

fn main() {
    println!(
        "valid passport data: {}",
        get_valid_passport_count("input.txt").unwrap()
    );
}
