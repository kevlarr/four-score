use std::io;
use std::io::Write;

type Input<T> = Result<T, io::Error>;

#[derive(Debug)]
struct Players(pub String, pub String);

#[derive(Debug)]
struct Board(pub u8);

fn main() {
    println!("Welcome to FourScore!\n");

    let players = new_players().unwrap();
    let board = new_board().unwrap();

    println!("Players {:?}, playing with board {:?}", players, board);
}

fn new_players() -> Input<Players> {
    let first = input("First player")?;
    let mut second = input("Second player")?;

    while first == second {
        second = input("Please use a unique name")?;
    }

    Ok(Players(first, second))
}

fn new_board() -> Input<Board> {
    let choice = input_range(
        "Play with a (1) standard or (2) custom board",
        1, 2)?;

    Ok(Board(choice))
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
