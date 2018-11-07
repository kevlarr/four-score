mod board;

pub use self::board::Board;
use ::std::io;

pub type Input<T> = Result<T, io::Error>;

#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub token: Token,
}

#[derive(Debug)]
pub enum Token { X, O }
