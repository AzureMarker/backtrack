use std::fmt;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use backtracker::Config;

/// Default cell char which designates an empty cell
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

impl Suitcase {
    /// Get a new suitcase flipped by 90 degrees
    fn flip(&self) -> Suitcase {
        Suitcase { width: self.height, height: self.width, name: self.name }
    }
}

impl Trunk {
    /// Create a starting Trunk config
    ///
    /// # Arguments
    /// * `width` - the width of the trunk
    /// * `height` - the height of the trunk
    /// * `suitcases` - the suitcases to add to the trunk
    pub fn new(width: usize, height: usize, suitcases: &[Suitcase]) -> Trunk {
        let mut suitcases_remaining = suitcases.to_vec();

        // Sort them so that we add them in order of largest to smallest
        suitcases_remaining.sort_by(|a, b|
            (a.width * a.height).cmp(&(b.width * b.height))
        );

        Trunk {
            width, height,
            grid: vec![vec![DEFAULT_CELL; width]; height],
            suitcases_remaining
        }
    }

    /// Read a Trunk config from a file
    ///
    /// # Arguments
    /// * `filename` - the filename of the config to read
    pub fn read_from_file(filename: &str) -> io::Result<Trunk> {
        // Read in the file
        let mut file = File::open(filename)?;
        let mut contents = String::new();

        file.read_to_string(&mut contents)?;

        // Split it into lines
        let mut lines = contents.lines();
        let header = lines.nth(0).unwrap();

        // Parse the header
        let mut header_split = header.split_whitespace();
        let width: usize = header_split.nth(0).unwrap().parse().unwrap();
        let height: usize = header_split.nth(0).unwrap().parse().unwrap();

        // Parse the suitcases
        let suitcases: Vec<Suitcase> = lines
            .map(|line| {
                let split: Vec<&str> = line.split_whitespace().collect();
                let name: char = split[0].parse().unwrap();
                let width: usize = split[1].parse().unwrap();
                let height: usize = split[2].parse().unwrap();

                Suitcase { width, height, name }
            })
            .collect();

        // Make the Trunk
        Ok(Trunk::new(width, height, &suitcases))
    }

    /// Create a Trunk from an existing one. This will copy the existing Trunk's Suitcase stack and
    /// pop off the last suitcase for the new Trunk.
    ///
    /// # Arguments
    /// * `trunk` - the base Trunk
    /// * `grid` - the grid for the new Trunk
    fn from(trunk: &Trunk, grid: Vec<Vec<char>>) -> Trunk {
        let mut suitcases_remaining = trunk.suitcases_remaining.clone();
        suitcases_remaining.pop();

        Trunk {
            width: trunk.width,
            height: trunk.height,
            grid,
            suitcases_remaining
        }
    }

    /// Check if the suitcase will fit in the grid, starting from a certain row and column
    ///
    /// # Arguments
    /// * `grid` - the grid to check against
    /// * `start_row` - the starting row
    /// * `start_col` - the starting column
    /// * `suitcase` - the Suitcase to check with
    fn will_fit(grid: &Vec<Vec<char>>, start_row: usize, start_col: usize, suitcase: &Suitcase)
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

    /// Add a Suitcase to a grid (does not check if it fits, use [will_fit](Trunk::will_fit) for
    /// that). Ownership of `grid` will be returned in the return value. This is used for syntactic
    /// convenience.
    ///
    /// # Arguments
    /// * `grid` - the grid to modify (ownership is returned at the end)
    /// * `start_row` - the starting row
    /// * `start_col` - the starting column
    /// * `suitcase` - the [Suitcase](Suitcase) to add
    fn add_suitcase(mut grid: Vec<Vec<char>>, start_row: usize, start_col: usize, suitcase: &Suitcase)
            -> Vec<Vec<char>>{
        for row in start_row..(start_row + suitcase.height) {
            for col in start_col..(start_col + suitcase.width) {
                grid[row][col] = suitcase.name;
            }
        }

        grid
    }

    /// Return a copy of the grid
    fn copy_grid(&self) -> Vec<Vec<char>> {
        self.grid.clone()
    }
}

impl Config for Trunk {
    fn successors(&self) -> Vec<Self> {
        let suitcase = match self.suitcases_remaining.last() {
            Some(s) => s,
            None => return vec![]
        };

        let mut successors = vec![];

        for row in 0..self.height {
            for col in 0..self.width {
                // Check if the suitcase fits without flipping
                if Trunk::will_fit(&self.grid, row, col, suitcase) {
                    successors.push(Trunk::from(&self, Trunk::add_suitcase(
                        self.copy_grid(),row, col, &suitcase
                    )));
                }

                // Try flipped
                let flipped_suitcase = suitcase.flip();
                if Trunk::will_fit(&self.grid, row, col, &flipped_suitcase) {
                    successors.push(Trunk::from(&self, Trunk::add_suitcase(
                        self.copy_grid(),row, col, &flipped_suitcase
                    )));
                }
            }
        }

        successors
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
