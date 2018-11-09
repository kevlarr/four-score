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

impl Board {
    pub fn new(height: usize, width: usize) -> Self {
        let rows = (0..height)
            .map(|_| (0..width).map(|_| None).collect())
            .collect();

        Board { height, width, rows }
    }

    /// Attempts to place token in given column, returning false
    /// if no open row was found
    pub fn insert(&mut self, col: usize, token: Token) -> bool {
        match self.rows.iter_mut().rev().find(|row| row[col] == None) {
            Some(row) => {
                row[col] = Some(token);
                return true;
            },
            None => false,
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
