mod cursor;
mod errors;

use cursor::Cursor;
use errors::*;

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub literal: String,
    pub loc: Location,
}

#[derive(Debug)]
pub enum TokenKind {
    Int,
}

#[derive(Debug)]
pub struct Location {
    pub row: u32,
    pub col: u32,
}

impl Token {
    pub fn new(kind: TokenKind, literal: String, loc: Location) -> Token {
        Token { kind, literal, loc }
    }
}

impl Location {
    pub fn new(row: u32, col: u32) -> Location {
        Location { row, col }
    }
}

impl<'a> Cursor<'a> {
    /// Returns the int literal in a Result or a LexError
    pub(crate) fn eat_int(&mut self) -> Result<String, LexError> {
        let int_range = '0'..'9';

        if !int_range.contains(&self.peek_first()) {
            Err(LexError::InvalidInt) // should only happen if there is no check made before calling
        } else {
            let literal = self.eat_while(|c| int_range.contains(&c));
            Ok(literal.into_iter().collect::<String>())
        }
    }
}

#[test]
fn test_eat_int() {
    let mut c = Cursor::new("123");
    assert_eq!(c.eat_int(), Ok(String::from("123")));
}