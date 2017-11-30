mod config;
mod backtracker;
mod queen;

use queen::QueensConfig;
use backtracker::Backtracker;

fn main() {
    let queen_config = QueensConfig::new();
    println!("{}", match Backtracker::solve(queen_config) {
        Some(solution) => format!("{}", solution),
        None => "No solution found".to_string()
    });
}
