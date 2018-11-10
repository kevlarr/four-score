extern crate fourscore;

use std::io;
use std::io::Write;
use fourscore::*;
use fourscore::MoveResult::*;

fn main() {
    println!("Welcome to FourScore!\n");

    let players = new_players().unwrap();
    let mut board = new_board().unwrap();
    let mut p = 0;

    loop {
        println!("{}", board);

        let prompt = format!("\n{}, place your \"{}\"", players[p].name, players[p].token);

        let mut col = get_column(prompt.as_str(), &board);

        loop {
            match board.insert(col, players[p].token) {
                GameWon => {
                    println!("{}\n{} won!", board, players[p].name);
                    return;
                },
                GameDraw => {
                    println!("{}\nDraw", board);
                    return;
                },
                ColumnFull => {
                    col = get_column("Choose an open column", &board);
                },
                Inserted => break,
            }
        }

        p = 1 - p;
    }
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
            let height = input_range("Number of rows (4-14)", 4, 14)?;
            let width = input_range("Number of columns (4-14)", 4, 14)?;

            Ok(Board::new(height, width))
        },
    }
}

fn get_column(prompt: &str, b: &Board) -> usize {
    // Display is 1-based, so subtract 1
    return input_range(prompt, 1, b.width).unwrap() - 1;
}

fn input_range(prompt: &str, x: usize, y: usize) -> Input<usize> {
    let mut i = input(prompt)?;

    loop {
        if let Ok(n) = i.parse::<usize>() {
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
