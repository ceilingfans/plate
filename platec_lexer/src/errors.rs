#[derive(Debug, PartialEq)]
pub enum LexError {
    InvalidInt,
    InvalidFloat(InvalidFloat),
}

#[derive(Debug, PartialEq)]
pub enum InvalidFloat {
    MultiplePeriods,
    IllegalChars,
}
