mod cursor;

use cursor::Cursor;

#[cfg(test)]
mod tests;

pub struct Token {
    pub literal: String,
    pub len: usize,
    pub pos: Location,
}

pub struct Location {
    pub row: u32,
    pub col: u32,
}