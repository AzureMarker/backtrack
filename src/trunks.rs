use std::fmt;
use backtracker::Config;

static DEFAULT_CELL: char = '-';

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
            grid: vec![vec![DEFAULT_CELL; width]; height],
            suitcases_remaining: suitcases.to_vec()
        }
    }

    fn from(trunk: &Trunk, grid: Vec<Vec<char>>) -> Trunk {
        Trunk {
            width: trunk.width,
            height: trunk.height,
            grid,
            suitcases_remaining: trunk.suitcases_remaining.clone(),
        }
    }

    fn will_fit(grid: &Vec<Vec<char>>, start_row: usize, start_col: usize, suitcase: Suitcase)
            -> bool {
        // Simple bounds check as a heuristic
        if start_row + suitcase.height > grid.len() || start_col + suitcase.width > grid[0].len() {
            return false;
        }

        // Make sure there's no suitcases in the way
        for row in start_row..(start_row + suitcase.height) {
            for col in start_col..(start_col + suitcase.width) {
                if row > grid.len() || col > grid[row].len() || grid[row][col] != DEFAULT_CELL {
                    return false;
                }
            }
        }

        true
    }

    fn add_suitcase<'a>(grid: &'a mut Vec<Vec<char>>, start_row: usize, start_col: usize, suitcase: &Suitcase)
            -> &'a mut Vec<Vec<char>>{
        for row in start_row..(start_row + suitcase.height) {
            for col in start_col..(start_col + suitcase.width) {
                grid[row][col] = suitcase.name;
            }
        }

        grid
    }

    fn copy_grid(&self) -> Vec<Vec<char>> {
        self.grid.clone()
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
