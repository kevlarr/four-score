use super::Token;
use ::std::fmt;

type Cell = Option<Token>;
type Row = Vec<Cell>;

#[derive(Debug)]
pub struct Board {
    pub width: usize,
    pub height: usize,
    rows: Vec<Row>,
}

pub enum MoveResult {
    Inserted,
    ColumnFull,
    GameDraw,
    GameWon,
}

impl Board {
    pub fn new(height: usize, width: usize) -> Self {
        let rows = (0..height)
            .map(|_| (0..width).map(|_| None).collect())
            .collect();

        Board { height, width, rows }
    }

    /// Attempts to place token in given column, returning row number
    /// if successful or None if not
    pub fn insert(&mut self, col: usize, token: Token) -> MoveResult {
        for row in (0..self.height).rev() {
            if let None = self.rows[row][col] {
                self.rows[row][col] = Some(token);

                if self.won(row, col) {
                    return MoveResult::GameWon;
                } else if self.draw(row) {
                    return MoveResult::GameDraw;
                }

                return MoveResult::Inserted;
            }
        }
        MoveResult::ColumnFull
    }

    /// Bottom row is highest index
    ///        0  1  .. w-1
    ///    0 |‾‾‾‾‾‾‾‾‾‾‾‾‾‾
    ///    1 |
    ///    2 |
    ///   .. |
    ///  h-1 |
    ///
    fn won(&self, row: usize, col: usize) -> bool {
        self.won_vertically(row, col)
    }

    fn won_vertically(&self, row: usize, col: usize) -> bool {
        // Only bother checking if it's at least the 4th token played in col
        if (row > self.height - 4) { return false; }

        (row + 1 .. row + 4)
            .map(|i| self.rows[i][col])
            .find(|cell| cell != &self.rows[row][col])
            .is_none()
    }

    fn draw(&self, row: usize) -> bool {
        if row > 0 {
            return false;
        }

        match self.rows[row].iter().find(|cell| *cell == &None) {
            Some(cell) => false,
            None => true,
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\n  __{}_", "__".repeat(self.width))?;

        for row in self.rows.iter() {
            write!(f, "\n  | ")?;

            for cell in row.iter() {
                match cell {
                    Some(token) => write!(f, "{} ", token)?,
                    None => write!(f, "· ")?,
                }
            }

            write!(f, "|")?;
        }

        write!(f, "\n  *‾{}*\n    ", "‾‾".repeat(self.width as usize))?;

        // This is more fun, but way less clear:
        //
        //   write!(f, "  {}", (1..self.width)
        //      .map(|i| i.to_string())
        //      .collect::<Vec<String>>()
        //      .join(" "))?;

        for i in 0..self.width {
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

        assert_eq!(board.height, 4);
        assert_eq!(board.width, 5);
        assert_eq!(board.rows, vec![
            vec![None, None, None, None, None],
            vec![None, None, None, None, None],
            vec![None, None, None, None, None],
            vec![None, None, None, None, None],
        ]);
    }
}
