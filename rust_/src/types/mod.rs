mod board;

pub use self::board::Board;
use ::std::io;
use ::std::fmt;

pub type Input<T> = Result<T, io::Error>;

#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub token: Token,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Token { X, O }

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Token::X => write!(f, "x"),
            Token::O => write!(f, "o"),
        }
    }
}
