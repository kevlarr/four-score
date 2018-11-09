use super::Token;
use ::std::fmt;

type Grid = Vec<Row>;
type Row = Vec<Cell>;
type Cell = Option<Token>;

#[derive(Debug)]
pub struct Board {
    pub columns: u8,
    pub rows: u8,
    grid: Grid,
}

impl Board {
    pub fn new(rows: u8, columns: u8) -> Self {
        let grid = (0..rows)
            .map(|_| (0..columns).map(|_| None).collect())
            .collect();

        Board { rows, columns, grid }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\n  __{}_", "__".repeat(self.columns as usize))?;

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
