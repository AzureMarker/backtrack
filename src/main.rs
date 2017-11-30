mod backtracker;
mod queen;

use queen::QueensConfig;

fn main() {
    // Create a starting config for the 8 Queens Puzzle
    let queen_config = QueensConfig::new(4, 4);

    // Solve the puzzle
    println!(
        "{}",
        match backtracker::solve(queen_config) {
            Some(solution) => format!("{}", solution),
            None => "No solution found".to_string()
        }
    );
}
