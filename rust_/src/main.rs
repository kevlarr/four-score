use std::io;
use std::io::Write;

type Input<T> = Result<T, io::Error>;
type Row = Vec<Option<Token>>;
type Grid = Vec<Row>;

#[derive(Debug)]
struct Player {
    pub name: String,
    pub token: Token,
}

#[derive(Debug)]
enum Token { X, O }

#[derive(Debug)]
struct Board {
    pub columns: u8,
    pub rows: u8,
    grid: Grid,
}

impl Board {
    fn new(rows: u8, columns: u8) -> Self {
        let grid = (0..rows)
            .map(|_| (0..columns).map(|_| None).collect())
            .collect();

        Board { rows, columns, grid }
    }
}

fn main() {
    println!("Welcome to FourScore!\n");

    let players = new_players().unwrap();
    let board = new_board().unwrap();

    println!("{:?}, playing with {:?}", players, board);
}

fn new_players() -> Input<[Player; 2]> {
    let first = input("First player")?;
    let mut second = input("Second player")?;

    while first == second {
        second = input("Please use a unique name")?;
    }

    Ok([
       Player { name: first,  token: Token::X },
       Player { name: second, token: Token::O },
    ])
}

fn new_board() -> Input<Board> {
    let choice = input_range("Play with a (1) standard or (2) custom board", 1, 2)?;

    match choice {
        1 => Ok(Board::new(7, 7)),
        _ => {
            let rows = input_range("Number of rows (4-14)", 4, 14)?;
            let columns = input_range("Number of columns (4-14)", 4, 14)?;

            Ok(Board::new(rows, columns))
        },
    }
}

fn input_range(prompt: &str, x: u8, y: u8) -> Input<u8> {
    let mut i = input(prompt)?;

    loop {
        if let Ok(n) = i.parse::<u8>() {
            if n >= x && n <= y {
                return Ok(n);
            }
        }

        i = input("Please enter a valid choice")?;
    }
}

fn input(prompt: &str) -> Input<String> {
    print!("{}: ", prompt);

    // Need to print and then flush to keep read_line on same line
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(input.trim().to_string())
}
