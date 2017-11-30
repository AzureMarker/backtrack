mod backtracker;
mod queen;

use queen::QueensConfig;

fn main() {
    let queen_config = QueensConfig::new(4, 4);
    println!("{}", match backtracker::solve(queen_config) {
        Some(solution) => format!("{}", solution),
        None => "No solution found".to_string()
    });
}
