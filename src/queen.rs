use std::fmt;
use backtracker::Config;

#[derive(Debug)]
pub struct QueensConfig {
    pub map: [[bool; 8]; 8],
    pub row: usize,
    pub col: usize
}

impl QueensConfig {
    pub fn new(row: usize, col: usize) -> QueensConfig {
        let mut map = [[false; 8]; 8];

        // Start with one queen
        map[row][col] = true;

        QueensConfig { map, row, col }
    }

    pub fn new_from(old_config: &QueensConfig, row: usize, col: usize) -> QueensConfig {
        let mut new_map = old_config.map;
        new_map[row][col] = true;

        QueensConfig { map: new_map, row, col }
    }
}

impl Config for QueensConfig {
    fn successors(&self) -> Vec<QueensConfig> {
        // Get the next column to generate successors in
        let new_col = if self.col == self.map.len() - 1 {
            // If we're at the right side of the map, check if we should wrap to the left side
            for row in self.map.iter() {
                // We only wrap back if there are no queens in the left-most column
                if row[0] {
                    return vec![];
                }
            }

            // If the left-most column is empty, use that column
            0
        }
        else {
            // Keep moving right
            self.col + 1
        };

        let mut successors = Vec::with_capacity(8);

        for row in 0..8 {
            successors.push(QueensConfig::new_from(&self, row, new_col));
        }

        successors
    }

    fn is_valid(&self) -> bool {
        // Check vertical
        for (i, elem) in self.map.iter().enumerate() {
            if i != self.row && elem[self.col] {
                return false;
            }
        }


        // Check horizontal
        for (i, elem) in self.map[self.row].iter().enumerate() {
            if i != self.col && *elem {
                return false;
            }
        }

        // Check diagonals
        for (row_index, row) in self.map.iter().enumerate() {
            // Skip the row with the piece we are checking against
            if row_index == self.row {
                continue;
            }

            // Get the columns of the cells to check
            let diagonal1 = self.col as isize - self.row as isize + row_index as isize;
            let diagonal2 = self.col as isize + self.row as isize - row_index as isize;

            // Check the first diagonal, if it is on the board
            if 0 <= diagonal1 && diagonal1 < row.len() as isize && self.map[row_index][diagonal1 as usize] {
                return false;
            }

            // Check the second diagonal, if it is on the board
            if 0 <= diagonal2 && diagonal2 < row.len() as isize && self.map[row_index][diagonal2 as usize] {
                return false;
            }
        }

        true
    }

    fn is_goal(&self) -> bool {
        if !self.is_valid(){
            return false;
        }

        for row in self.map.iter() {
            if !row.contains(&true) {
                return false;
            }
        }

        return true;
    }
}

impl fmt::Display for QueensConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.map.iter() {
            for elem in row.iter() {
                write!(f, "{} ", if *elem { "Q" } else { "-" })?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_valid() {
        let mut config = QueensConfig::new(1, 0);

        assert!(config.is_valid(), "starting config is valid");

        config.map[0][0] = true;
        assert!(!config.is_valid(), "conflicting queens is invalid");

        config.map[0][0] = false;
        config.map[3][1] = true;
        assert!(config.is_valid(), "non-conflicting queens is valid");
    }
}
