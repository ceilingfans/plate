use std::str::Chars;

use crate::Location;

pub const EOF_CHAR: char = '\0';

pub(crate) struct Cursor<'a> {
    pub(crate) input: Chars<'a>,
    pub(crate) remaining: usize,
    pub(crate) loc: Location,
}

pub(crate) enum Newline {
    Newline,        // \n
    CarriageReturn, // \r\n
}

/// Heavily inspired by the rustc_lexer package
impl<'a> Cursor<'a> {
    pub(crate) fn new(input: &str) -> Cursor {
        Cursor {
            input: input.chars(),
            remaining: input.len(),
            loc: Location::new(0, 0),
        }
    }

    /* Utility Methods */
    pub(crate) fn peek_first(&self) -> char {
        self.input.clone().next().unwrap_or(EOF_CHAR)
    }

    pub(crate) fn peek_second(&self) -> char {
        let mut iter = self.input.clone();
        iter.next();
        iter.next().unwrap_or(EOF_CHAR)
    }

    pub(crate) fn is_eof(&self) -> bool {
        self.input.as_str().is_empty()
    }

    pub(crate) fn is_newline(&self) -> Option<Newline> {
        if self.peek_first() == '\n' {
            Some(Newline::Newline)
        } else if self.peek_first() == '\r' && self.peek_second() == '\n' {
            Some(Newline::CarriageReturn)
        } else {
            None
        }
    }

    pub(crate) fn advance(&mut self) -> char {
        if self.is_eof() {
            EOF_CHAR
        } else {
            match self.is_newline() {
                Some(t) => {
                    self.loc.row += 1;
                    self.loc.col = 0;

                    match t {
                        Newline::Newline => {
                            self.input.next();
                            self.remaining -= 1;
                        }
                        Newline::CarriageReturn => {
                            self.input.next();
                            self.input.next();
                            self.remaining -= 2;
                        }
                    };

                    return '\n';
                }
                None => {
                    self.loc.col += 1;
                    self.remaining -= 1;

                    return self.input.next().unwrap();
                }
            };
        }
    }

    pub(crate) fn eat_while(&mut self, mut filter: impl FnMut(char) -> bool) -> Vec<char> {
        let mut ret = Vec::new();

        while !self.is_eof() && filter(self.peek_first()) {
            ret.push(self.advance());
        }

        ret
    }
}
