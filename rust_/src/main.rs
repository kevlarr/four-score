use std::io;
use std::io::Write;

fn main() {
    println!("Welcome to FourScore!\n");

    match get_player("First") {
        Ok(name) => println!("Hello, {}!", name),
        Err(e) => println!("Error: {:?}", e),
    }
}

fn get_player(nth: &str) -> Result<String, io::Error> {
    print!("{} player: ", nth);

    // Need to print and then flush to keep read_line on same line
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(input.trim().to_string())
}
