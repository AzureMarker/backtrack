pub trait Config: Sized {
    fn successors(&self) -> Vec<Self>;
    fn is_valid(&self) -> bool;
    fn is_goal(&self) -> bool;
}
