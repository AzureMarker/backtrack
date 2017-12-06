use std::fmt;
use backtracker::Config;

/// This struct holds the configuration of a step in solving the Trunks problem
#[derive(Debug)]
pub struct Trunk {
    width: usize,
    height: usize,
    grid: Vec<Vec<char>>,
    suitcases_remaining: Vec<Suitcase>
}

/// This struct holds data about a suitcase which will be added to a Trunk
#[derive(Debug, Clone)]
pub struct Suitcase {
    pub width: usize,
    pub height: usize,
    pub name: char
}

impl Trunk {
    pub fn new(width: usize, height: usize, suitcases: &[Suitcase]) -> Trunk {
        Trunk {
            width, height,
            grid: vec![vec!['-'; width]; height],
            suitcases_remaining: suitcases.to_vec()
        }
    }
}

impl Config for Trunk {
    fn successors(&self) -> Vec<Self> {
        vec![]
    }

    fn is_valid(&self) -> bool {
        true
    }

    fn is_goal(&self) -> bool {
        self.suitcases_remaining.is_empty()
    }
}

impl fmt::Display for Trunk {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.grid {
            for character in row {
                write!(f, "{}", character)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}
