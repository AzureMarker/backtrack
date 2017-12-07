extern crate backtrack;

use std::env;
use backtrack::queen::QueensConfig;

pub fn main() {
    // Get the starting config
    let (row, col) = if env::args().len() == 3 {
        let args: Vec<usize> = env::args()
            .skip(1)
            .take(2)
            .map(|arg| arg.parse().unwrap_or(0))
            .collect();

        if args[0] >= 8 || args[1] >= 8 {
            eprintln!("Location is out of bounds");
            return;
        }

        (args[0], args[1])
    } else {
        (0, 0)
    };

    let config = QueensConfig::new(row, col);

    println!("Solving for the first Queen at ({}, {})", row, col);

    // Solve the puzzle
    println!(
        "{}",
        match backtrack::solve(config) {
            Some(solution) => format!("{}", solution),
            None => "No solution found".to_string()
        }
    );
}
