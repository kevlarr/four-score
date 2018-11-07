use super::Token;

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
