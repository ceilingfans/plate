use std::str::Chars;

const EOF_CHAR: char = '\0';

pub(crate) struct Cursor<'a> {
    pub(crate) input: Chars<'a>,
    pub(crate) remaining: usize,
}

/// Heavily inspired by the rustc_lexer package
impl<'a> Cursor<'a> {
    pub(crate) fn new(input: &str) -> Cursor {
        Cursor {
            input: input.chars(),
            remaining: input.len(),
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

    pub(crate) fn advance(&mut self) -> char {
        if self.is_eof() {
            EOF_CHAR
        } else {
            self.remaining -= 1;
            self.input.next().unwrap()
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
