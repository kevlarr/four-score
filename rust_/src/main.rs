use std::io;
use std::io::Write;

struct Players(pub String, pub String);

fn main() {
    println!("Welcome to FourScore!\n");

    match new_players() {
        Ok(players) => println!("{} and {}, sitting in a tree...", players.0, players.1),
        Err(e) => println!("Error: {:?}", e),
    }
}

fn new_players() -> Result<Players, io::Error> {
    let first = input("First player")?;
    let mut second = input("Second player")?;

    while first == second {
        second = input("Please use a unique name")?;
    }

    Ok(Players(first, second))
}

fn input(prompt: &str) -> Result<String, io::Error> {
    print!("{}: ", prompt);

    // Need to print and then flush to keep read_line on same line
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(input.trim().to_string())
}
