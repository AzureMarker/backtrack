use super::config::Config;

pub struct Backtracker {}

impl Backtracker {
    pub fn solve<C: Config>(config: C) -> Option<C> {
        if config.is_goal() {
            return Some(config);
        }

        for child in config.successors() {
            if child.is_valid() {
                match Backtracker::solve(child) {
                    Some(solution) => return Some(solution),
                    None => continue
                }
            }
        }

        None
    }
}