use std::io;
use std::io::BufRead;
use std::fs::File;
use std::path::Path;


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

fn read_expense_report(path: &str) -> Result<Vec<u32>, Error> {
    lines(path)
        .map_err(|_| Error::FileReadError)?
        .map(|line| line
            .map_err(|_| Error::FileReadError)?.parse()
            .map_err(|_| Error::ParsingError))
        .collect()
}



fn main() {
    let expense_report = read_expense_report("input.txt").unwrap();

    'part1: for &a in &expense_report {
        for &b in &expense_report {
            if a + b == 2020 {
                println!("{}", a * b);
                break 'part1;
            }
        }
    }

    'part2: for &a in &expense_report {
        for &b in &expense_report {
            for &c in &expense_report {
                if a + b + c == 2020 {
                    println!("{}", a * b * c);
                    break 'part2;
                }
            }
        }
    }
}