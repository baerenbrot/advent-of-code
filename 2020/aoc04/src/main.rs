use regex::Regex;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Copy, Clone)]
enum Error {
    FileReadError,
    RegexError,
}

struct PassportData {
    fields: HashMap<String, String>,
}

impl PassportData {
    fn new(data: &str) -> Self {
        PassportData {
            fields: Regex::new(r"(?P<key>[a-z]{3}):(?P<value>[^ \n]+)")
                .map_err(|_| Error::RegexError)
                .unwrap()
                .captures_iter(&data)
                .map(|c| (String::from(&c["key"]), String::from(&c["value"])))
                .collect(),
        }
    }

    fn is_valid_pt1(&self) -> bool {
        let required_fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
        required_fields
            .iter()
            .all(|&key| self.fields.contains_key(key))
    }

    #[allow(dead_code)]
    fn to_string(&self) -> String {
        self.fields.iter().map(|(key, value)| {
            format!("{}: {}", key, value)
        }).collect::<Vec<String>>().join("\n")
    }

    fn is_valid_pt2(&self) -> bool {
        if !self.is_valid_pt1() {
            return false;
        } 

        let is_valid_year = |key: &str, min: usize, max: usize| -> bool {
            if let Ok(number) = self.fields[key].parse::<usize>() {
                if number < min { return false; }
                if number > max { return false; }
                true
            } else {
                false
            }
        };

        if !is_valid_year("byr", 1920, 2002) {
            return false;
        }
        if !is_valid_year("iyr", 2010, 2020) {
            return false;
        }
        if !is_valid_year("eyr", 2020, 2030) {
            return false;
        }

        if let Some(caps) = Regex::new(r"(\d+)(cm|in)").unwrap().captures(&self.fields["hgt"]) {
            let value: usize = caps[1].parse().unwrap();
            let (min, max) = if &caps[2] == "cm" {(150, 193)} else {(59, 76)};
            if value < min {
                return false;
            }
            if value > max {
                return false;
            }
        } else {
            return false;
        }

        if !Regex::new(r"^#[0-9a-f]{6}$").unwrap().is_match(&self.fields["hcl"]) {
            return false;
        }

        if !Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap().is_match(&self.fields["ecl"]) {
            return false;
        }
        
        if !Regex::new(r"^\d{9}$").unwrap().is_match(&self.fields["pid"]) {
            return false;
        }

        true
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

fn main() {
    
    if let Ok(passport_data) = read_passport_data("input.txt") {

        let count_pt1 = passport_data
                .iter()
                .filter(|data| data.is_valid_pt1())
                .count();

        let count_pt2 = passport_data
                .iter()
                .filter(|data| data.is_valid_pt2())
                .count();
    
        println!("valid passport data for part 1: {}", count_pt1);
        println!("valid passport data for part 2: {}", count_pt2);
    }
}
