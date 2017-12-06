mod backtracker;
mod queen;
mod trunks;

use trunks::{Trunk, Suitcase};

fn main() {
    // Create a starting config for the Trunk problem
    let config = Trunk::new(3, 3, &[
        Suitcase { width: 1, height: 3, name: 'A' },
        Suitcase { width: 2, height: 1, name: 'B' },
        Suitcase { width: 1, height: 2, name: 'C' },
        Suitcase { width: 1, height: 1, name: 'D' },
        Suitcase { width: 1, height: 1, name: 'E' }
    ]);

    // Solve the puzzle
    println!(
        "{}",
        match backtracker::solve(config) {
            Some(solution) => format!("{}", solution),
            None => "No solution found".to_string()
        }
    );
}
