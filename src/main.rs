mod backtracker;
mod queen;
mod trunks;

use std::env;
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
        Some(filename) => match Trunk::read_from_file(&filename) {
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
