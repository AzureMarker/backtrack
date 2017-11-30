mod config;
mod backtracker;
#[cfg(test)]
mod test;

use config::QueensConfig;
use backtracker::Backtracker;

fn main() {
    let queen_config = QueensConfig::new();
    println!("{}", match Backtracker::solve(queen_config) {
        Some(solution) => format!("{}", solution),
        None => "No solution found".to_string()
    });
}
