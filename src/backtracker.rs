/// Used to solve a backtracking problem
pub trait Config: Sized {
    /// Get all the successive configs from the current config
    fn successors(&self) -> Vec<Self>;

    /// Check if the config is valid
    fn is_valid(&self) -> bool;

    /// Check if the config is the final/goal config. It must also be valid.
    fn is_goal(&self) -> bool;
}

/// Solve the backtracking problem using the specified config (recursive)
pub fn solve<C: Config>(config: C) -> Option<C> {
    // Return once we found the goal
    if config.is_goal() {
        return Some(config);
    }

    // Search the successors for the goal
    for child in config.successors() {
        // The config must be valid for it to be the goal
        if child.is_valid() {
            // Check if the config leads to a solution
            match solve(child) {
                Some(solution) => return Some(solution),
                None => continue
            }
        }
    }

    // No solution found, prune this tree
    None
}
