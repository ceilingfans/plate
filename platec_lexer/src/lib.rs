use cursor::{Cursor, Newline, EOF_CHAR};

mod cursor;
mod errors;

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub literal: String,
    /// start location of the token
    pub loc: Location,
}

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    /// -
    Minus,
    /// +
    Plus,
    /// /
    Slash,
    /// *
    Star,
    /// :
    Colon,
    /// ;
    SemiColon,
    /// (
    OpenParen,
    /// )
    CloseParen,
    /// [
    OpenBracket,
    /// ]
    CloseBracket,
    /// {
    OpenBrace,
    /// }
    CloseBrace,
    /// =
    Eq,
    /// !
    Bang,
    /// <
    Lt,
    /// >
    Gt,
    /// &
    Ampersand,
    /// |
    Pipe,
    /// .
    Dot,

    Ident,
    Keyword(KeywordKind),
    Number(NumberKind),
    Comment,

    String,
    Char,
    
    Eof,
}

#[derive(Debug, PartialEq)]
pub enum QuoteKind {
    Single,
    Double,
}

#[derive(Debug, PartialEq)]
pub enum NumberKind {
    Int,
    Float,
}

#[derive(Debug, PartialEq)]
pub enum KeywordKind {
    /// let
    Let,
    /// fn
    Fn,
    /// loop
    Loop, // TODO: while
    /// break
    Break,
    /// include
    Include,
    /// extern
    Extern,
}

#[derive(Debug, Clone, PartialEq)]
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

pub fn tokenize(input: &str) -> impl Iterator<Item = Token> + '_ {
    let mut c = Cursor::new(input);
    std::iter::from_fn(move || {
        let tok = c.advance_token();
        if tok.kind != TokenKind::Eof { Some(tok) } else { None }
    })
}

impl<'a> Cursor<'a> {
    fn eat_comment(&mut self, loc: Location) -> Token {
        let mut literal = String::new();

        while !self.is_eof() {
            match self.is_newline() {
                Some(_) => {
                    self.advance();
                    break;
                }
                None => literal.push(self.advance()),
            }
        }

        Token::new(TokenKind::Comment, literal, loc)
    }

    fn eat_number(&mut self, loc: Location) -> Token {
        // TODO: implement bin hex and oct
        let mut literal = String::new();
        let mut dot_flag = false;

        while !self.is_eof() {  
            match self.peek_first() {
                '0'..='9' => literal.push(self.advance()),
                '_' => {
                    self.advance();
                }
                '.' => {
                    if dot_flag {
                        panic!("multiple dots in number"); // TODO: actual error message
                    }

                    dot_flag = true;
                    literal.push(self.advance());
                }
                _ => break,
            }
        }

        let num_kind = if dot_flag {
            NumberKind::Float
        } else {
            NumberKind::Int
        };
        Token::new(TokenKind::Number(num_kind), literal, loc)
    }

    fn eat_string(&mut self, loc: Location) -> Token {
        let mut literal = String::new();
        let mut closed = false;
        self.advance();

        while !self.is_eof() && self.is_newline().is_none() {
            if self.peek_first() == '\\' && self.peek_second() == '"' {
                literal.push(self.advance());
                literal.push(self.advance());
            }

            if self.peek_first() == '"' {
                self.advance();
                closed = true;
                break;
            }

            literal.push(self.advance());
        }

        if !closed {
            panic!("string was never closed");
        }

        Token::new(TokenKind::String, literal, loc)
    }

    pub fn advance_token(&mut self) -> Token {
        let current_loc = self.loc.clone();
        let current_char = self.peek_first();

        let token_kind = match current_char {
            // single char tokens
            '-' => TokenKind::Minus,
            '+' => TokenKind::Plus,
            '*' => TokenKind::Star,
            ':' => TokenKind::Colon,
            ';' => TokenKind::SemiColon,
            '(' => TokenKind::OpenParen,
            ')' => TokenKind::CloseParen,
            '[' => TokenKind::OpenBracket,
            ']' => TokenKind::CloseBracket,
            '{' => TokenKind::OpenBrace,
            '}' => TokenKind::CloseBrace,
            '=' => TokenKind::Eq,
            '!' => TokenKind::Bang,
            '<' => TokenKind::Lt,
            '>' => TokenKind::Gt,
            '&' => TokenKind::Ampersand,
            '|' => TokenKind::Pipe,
            '.' => TokenKind::Dot,
            EOF_CHAR => TokenKind::Eof,

            // comments or slash
            '/' => {
                if self.peek_second() == '/' {
                    return self.eat_comment(current_loc);
                } else {
                    TokenKind::Slash
                }
            }

            // numbers
            '0'..='9' => {
                return self.eat_number(current_loc);
            }

            // char
            '\'' => panic!("chars are not implemented"),

            // string
            '"' => {
                return self.eat_string(current_loc);
            }

            _ => {
                if is_whitespace(current_char) {
                    self.advance();
                    return self.advance_token();
                }

                if is_ident(current_char) {
                    let ident = self.eat_while(|x| is_ident(x)).iter().collect::<String>();
                    let ty = match ident.as_str() {
                        "let" => TokenKind::Keyword(KeywordKind::Let),
                        "fn" => TokenKind::Keyword(KeywordKind::Fn),
                        "loop" => TokenKind::Keyword(KeywordKind::Loop),
                        "break" => TokenKind::Keyword(KeywordKind::Break),
                        "include" => TokenKind::Keyword(KeywordKind::Include),
                        "extern" => TokenKind::Keyword(KeywordKind::Extern),
                        reserved @ ("true" | "false" | "while" | "for" | "ret") => {
                            panic!("{} unimplemented but reserved", reserved);
                        }
                        _ => TokenKind::Ident,
                    };

                    return Token::new(ty, ident, current_loc);
                }

                panic!("not implemented: {:?}", current_char);
            }
        };

        self.advance();
        Token::new(token_kind, current_char.to_string(), current_loc)
    }
}

fn is_whitespace(c: char) -> bool {
    matches!(
        c,
        // taken from rustc_lexer
        // Usual ASCII suspects
        '\u{0009}'   // \t
        | '\u{000A}' // \n
        | '\u{000B}' // vertical tab
        | '\u{000C}' // form feed
        | '\u{000D}' // \r
        | '\u{0020}' // space

        // NEXT LINE from latin1
        | '\u{0085}'

        // Bidi markers
        | '\u{200E}' // LEFT-TO-RIGHT MARK
        | '\u{200F}' // RIGHT-TO-LEFT MARK

        // Dedicated whitespace characters from Unicode
        | '\u{2028}' // LINE SEPARATOR
        | '\u{2029}' // PARAGRAPH SEPARATOR
    )
}

fn is_ident(c: char) -> bool {
    matches!(
        c,
        'a'..='z'
        | 'A'..='Z'
        | '0'..='9'
        | '_'
    )
}
