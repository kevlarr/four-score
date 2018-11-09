use super::Token;
use ::std::fmt;

type Cell = Option<Token>;
type Row = Vec<Cell>;

#[derive(Debug)]
pub struct Board {
    pub columns: usize,
    pub rows: usize,
    grid: Vec<Row>,
}

impl Board {
    pub fn new(rows: usize, columns: usize) -> Self {
        let grid = (0..rows)
            .map(|_| (0..columns).map(|_| None).collect())
            .collect();

        Board { rows, columns, grid }
    }

    /// Attempts to place token in given column, returning
    /// Some(row) if successful or None if already full
    pub fn insert(&mut self, col: usize, token: Token) -> Option<usize> {
        // match self.find_available_cell(col) {
        //    Some(row) => {
        //        self.grid[row][col] = token.clone();
        //        return Some(row);
        //    },
        //    None => false,
        // }

        if let Some(row) = self.find_available_cell(col) {
            self.grid[row][col] = Some(token);
            return Some(row);
        }

        None
    }

    /// Locates the lowest cell vertically (ie. highest row index),
    /// returning Some(index) or None if all are full.
    fn find_available_cell(&self, col: usize) -> Option<usize> {
        //for row in self.grid.iter().enumerate().rev() {
        //}

        for row in (0..self.rows).rev() {
            if let None = self.grid[row][col] {
                return Some(row);
            }
        }

        None
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\n  __{}_", "__".repeat(self.columns))?;

        for row in self.grid.iter() {
            write!(f, "\n  | ")?;

            for cell in row.iter() {
                match cell {
                    Some(token) => write!(f, "{} ", token)?,
                    None => write!(f, "· ")?,
                }
            }

            write!(f, "|")?;
        }

        write!(f, "\n  *‾{}*\n    ", "‾‾".repeat(self.columns as usize))?;

        // This is more fun, but way less clear:
        //
        //   write!(f, "  {}", (1..self.columns)
        //      .map(|i| i.to_string())
        //      .collect::<Vec<String>>()
        //      .join(" "))?;

        for i in 0..self.columns {
            write!(f, "{} ", (i + 1).to_string())?;
        }

        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let board = Board::new(4, 5);

        assert_eq!(board.rows, 4);
        assert_eq!(board.columns, 5);
        assert_eq!(board.grid, vec![
            vec![None, None, None, None, None],
            vec![None, None, None, None, None],
            vec![None, None, None, None, None],
            vec![None, None, None, None, None],
        ]);
    }
}
