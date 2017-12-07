extern crate backtrack;

use backtrack::queen::QueensConfig;

pub fn main() {
    // Create a starting config for the 8 Queens Puzzle
    let queen_config = QueensConfig::new(4, 4);

    // Solve the puzzle
    println!(
        "{}",
        match backtrack::solve(queen_config) {
            Some(solution) => format!("{}", solution),
            None => "No solution found".to_string()
        }
    );
}
