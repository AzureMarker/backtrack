use std::fmt;
use std::io;
use std::io::prelude::*;
use std::fs::File;
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

impl Suitcase {
    fn flip(&self) -> Suitcase {
        Suitcase { width: self.height, height: self.width, name: self.name }
    }
}

impl Trunk {
    pub fn new(width: usize, height: usize, suitcases: &[Suitcase]) -> Trunk {
        let mut suitcases_remaining = suitcases.to_vec();
        suitcases_remaining.sort_by(|a, b|
            (a.width * a.height).cmp(&(b.width * b.height))
        );

        Trunk {
            width, height,
            grid: vec![vec![DEFAULT_CELL; width]; height],
            suitcases_remaining
        }
    }

    pub fn read_from_file(filename: &str) -> io::Result<Trunk> {
        let mut file = File::open(filename)?;
        let mut contents = String::new();

        file.read_to_string(&mut contents)?;

        let mut lines = contents.lines();
        let header = lines.nth(0).unwrap();

        let mut header_split = header.split_whitespace();
        let width: usize = header_split.nth(0).unwrap().parse().unwrap();
        let height: usize = header_split.nth(0).unwrap().parse().unwrap();

        let suitcases: Vec<Suitcase> = lines
            .map(|line| {
                let split: Vec<&str> = line.split_whitespace().collect();
                let name: char = split[0].parse().unwrap();
                let width: usize = split[1].parse().unwrap();
                let height: usize = split[2].parse().unwrap();

                Suitcase { width, height, name }
            })
            .collect();

        Ok(Trunk::new(width, height, &suitcases))
    }

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

    fn add_suitcase(mut grid: Vec<Vec<char>>, start_row: usize, start_col: usize, suitcase: &Suitcase)
            -> Vec<Vec<char>>{
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
