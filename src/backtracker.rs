pub trait Config: Sized {
    fn successors(&self) -> Vec<Self>;
    fn is_valid(&self) -> bool;
    fn is_goal(&self) -> bool;
}

pub fn solve<C: Config>(config: C) -> Option<C> {
    if config.is_goal() {
        return Some(config);
    }

    for child in config.successors() {
        if child.is_valid() {
            match solve(child) {
                Some(solution) => return Some(solution),
                None => continue
            }
        }
    }

    None
}
