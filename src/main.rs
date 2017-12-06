mod backtracker;
mod queen;
mod trunks;

use std::env;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use trunks::{Trunk, Suitcase};

fn main() {
    let default_config = Trunk::new(3, 3, &[
        Suitcase { width: 1, height: 3, name: 'A' },
        Suitcase { width: 2, height: 1, name: 'B' },
        Suitcase { width: 1, height: 2, name: 'C' },
        Suitcase { width: 1, height: 1, name: 'D' },
        Suitcase { width: 1, height: 1, name: 'E' }
    ]);

    // Read in the config file, or use the default config
    let config = match env::args().nth(1) {
        Some(filename) => match read_from_file(&filename) {
            Ok(trunk) => trunk,
            Err(e) => {
                eprintln!("{}", e);
                return;
            }
        },
        None => default_config
    };

    // Solve the puzzle
    println!(
        "{}",
        match backtracker::solve(config) {
            Some(solution) => format!("{}", solution),
            None => "No solution found".to_string()
        }
    );
}

fn read_from_file(filename: &str) -> io::Result<Trunk> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    let mut lines = contents.lines();
    let header = lines.nth(0).unwrap();

    let mut header_split = header.split_whitespace();
    let width: usize = header_split.nth(0).unwrap().parse().unwrap();
    let height: usize = header_split.nth(0).unwrap().parse().unwrap();

    let suitcases: Vec<Suitcase> = lines
        .map(|line| {
            let split: Vec<&str> = line.split_whitespace().collect();
            let name: char = split[0].parse().unwrap();
            let width: usize = split[1].parse().unwrap();
            let height: usize = split[2].parse().unwrap();

            Suitcase { width, height, name }
        })
        .collect();

    Ok(Trunk::new(width, height, &suitcases))
}
