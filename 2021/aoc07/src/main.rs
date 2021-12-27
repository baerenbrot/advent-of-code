use std::fs;
use std::env;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
enum Error {
    ArgumentMissing,
    FileNotFound,
    ParsingError,
}

fn file_name() -> Result<String, Error> {
    Ok(env::args().nth(1).ok_or(Error::ArgumentMissing)?)
}

fn read_file(path: &String) -> Result<String, Error> {
    fs::read_to_string(path).map_err(|_| Error::FileNotFound)
}

struct Crabs(HashMap<isize, isize>);

impl Crabs {
    fn new(data: &String) -> Result<Self, Error> {
        let mut result = HashMap::new();
        for value in data.split(',').map(|t| t.trim().parse::<isize>().map_err(|_| Error::ParsingError)) {
            let fuel = value?;
            result.insert(fuel, result.get(&fuel).unwrap_or(&0) + 1);
        }
        Ok(Crabs(result))
    }

    fn fuel_cost(&self, to: isize, computation: fn(isize) -> isize) -> isize {
        self.0.iter().map(|(&position, count)| computation((position - to).abs()) * count).sum()
    }

    fn minimum_fuel_cost(&self, computation: fn(isize) -> isize) -> Option<isize> {
        let lower_bound = *self.0.keys().min()?;
        let upper_bound = *self.0.keys().max()?;
        (lower_bound..=upper_bound).map(|to| self.fuel_cost(to, computation)).min()
    }
}

fn main_or_error() -> Result<(), Error> {
    let file_name = file_name()?;
    let file_data = read_file(&file_name)?;
    let crabs = Crabs::new(&file_data)?;
    println!("Linear Minimum Fuel Cost: {}", crabs.minimum_fuel_cost(|t| t).unwrap());
    println!("Actual Minimum Fuel Cost: {}", crabs.minimum_fuel_cost(|t| t * (t+1) / 2).unwrap());
    Ok(())
}

fn main() {
    match main_or_error() {
        Ok(()) => {},
        Err(_) => {
            println!("An error occurred.");
        }
    }
}
