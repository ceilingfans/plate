use crate::*;

#[test]
fn test_newline() {
    let s = "1\n2";
    let mut c = Cursor::new(s);
    // 1
    assert_eq!(c.advance(), '1');
    assert_eq!(c.loc, Location { row: 0, col: 1 });

    // \n
    assert_eq!(c.advance(), '\n');
    assert_eq!(c.loc, Location { row: 1, col: 0 });

    // 2
    assert_eq!(c.advance(), '2');
    assert_eq!(c.loc, Location { row: 1, col: 1 });
}

#[test]
fn test_carriage_return_newline() {
    let s = "1\r\n2";
    let mut c = Cursor::new(s);
    assert_eq!(c.advance(), '1');
    assert_eq!(c.loc, Location { row: 0, col: 1 });

    // \n
    assert_eq!(c.advance(), '\n');
    assert_eq!(c.loc, Location { row: 1, col: 0 });

    // 2
    assert_eq!(c.advance(), '2');
    assert_eq!(c.loc, Location { row: 1, col: 1 });
}

#[test]
fn test_single_char_tokens() {
    let mut c = Cursor::new("-+/*");
    assert_eq!(
        c.advance_token(),
        Token::new(TokenKind::Minus, String::from('-'), Location::new(0, 0))
    );
    assert_eq!(
        c.advance_token(),
        Token::new(TokenKind::Plus, String::from('+'), Location::new(0, 1))
    );
    assert_eq!(
        c.advance_token(),
        Token::new(TokenKind::Slash, String::from('/'), Location::new(0, 2))
    );
    assert_eq!(
        c.advance_token(),
        Token::new(TokenKind::Star, String::from('*'), Location::new(0, 3))
    );

    assert_eq!(
        c.advance_token(),
        Token::new(TokenKind::Eof, String::from(EOF_CHAR), Location::new(0, 4))
    );
}

#[test]
fn test_comment() {
    let mut c = Cursor::new("// test\n");
    assert_eq!(
        c.advance_token(),
        Token::new(
            TokenKind::Comment,
            String::from("// test"),
            Location::new(0, 0)
        )
    );
    assert_eq!(
        c.advance_token(),
        Token::new(TokenKind::Eof, String::from(EOF_CHAR), Location::new(1, 0))
    );

    let mut c1 = Cursor::new("// test\r\n");
    assert_eq!(
        c1.advance_token(),
        Token::new(
            TokenKind::Comment,
            String::from("// test"),
            Location::new(0, 0)
        )
    );
    assert_eq!(
        c1.advance_token(),
        Token::new(TokenKind::Eof, String::from(EOF_CHAR), Location::new(1, 0))
    );

    let mut c2 = Cursor::new("// test");
    assert_eq!(
        c2.advance_token(),
        Token::new(
            TokenKind::Comment,
            String::from("// test"),
            Location::new(0, 0)
        )
    );
    assert_eq!(
        c2.advance_token(),
        Token::new(TokenKind::Eof, String::from(EOF_CHAR), Location::new(0, 7))
    );
}

#[test]
fn test_slash() {
    let mut c = Cursor::new("/");
    assert_eq!(
        c.advance_token(),
        Token::new(TokenKind::Slash, String::from('/'), Location::new(0, 0))
    );
    assert_eq!(
        c.advance_token(),
        Token::new(TokenKind::Eof, String::from(EOF_CHAR), Location::new(0, 1))
    );
}

#[test]
fn test_number() {
    let mut c = Cursor::new("1237894560");
    assert_eq!(
        c.advance_token(),
        Token::new(
            TokenKind::Number(NumberKind::Int),
            String::from("1237894560"),
            Location::new(0, 0)
        )
    );
    assert_eq!(
        c.advance_token(),
        Token::new(TokenKind::Eof, String::from(EOF_CHAR), Location::new(0, 10))
    );

    let mut c1 = Cursor::new("12345.67890");
    assert_eq!(
        c1.advance_token(),
        Token::new(
            TokenKind::Number(NumberKind::Float),
            String::from("12345.67890"),
            Location::new(0, 0)
        )
    );
    assert_eq!(
        c1.advance_token(),
        Token::new(TokenKind::Eof, String::from(EOF_CHAR), Location::new(0, 11))
    );
}

#[test]
#[should_panic] // todo: error message
fn test_invalid_float() {
    let mut c = Cursor::new("123.456.7890");
    c.advance_token();
}

#[test]
fn test_whitespace() {
    let mut c = Cursor::new("123 / 123");
    assert_eq!(
        c.advance_token(),
        Token::new(
            TokenKind::Number(NumberKind::Int),
            String::from("123"),
            Location::new(0, 0)
        )
    );
    assert_eq!(
        c.advance_token(),
        Token::new(TokenKind::Slash, String::from('/'), Location::new(0, 4))
    );
    assert_eq!(
        c.advance_token(),
        Token::new(
            TokenKind::Number(NumberKind::Int),
            String::from("123"),
            Location::new(0, 6)
        )
    );
    assert_eq!(
        c.advance_token(),
        Token::new(TokenKind::Eof, String::from(EOF_CHAR), Location::new(0, 9))
    );
}

#[test]
fn test_simple_ident() {
    let mut c = Cursor::new("test t1_a aAb___123");
    assert_eq!(
        c.advance_token(),
        Token::new(TokenKind::Ident, String::from("test"), Location::new(0, 0))
    );
    assert_eq!(
        c.advance_token(),
        Token::new(TokenKind::Ident, String::from("t1_a"), Location::new(0, 5))
    );
    assert_eq!(
        c.advance_token(),
        Token::new(
            TokenKind::Ident,
            String::from("aAb___123"),
            Location::new(0, 10)
        )
    );
    assert_eq!(
        c.advance_token(),
        Token::new(TokenKind::Eof, String::from(EOF_CHAR), Location::new(0, 19))
    );
}

#[test]
fn test_ident_keyword() {
    let mut c = Cursor::new("let fn loop break include extern Let");
    assert_eq!(
        c.advance_token(),
        Token::new(
            TokenKind::Keyword(KeywordKind::Let),
            String::from("let"),
            Location::new(0, 0)
        )
    );
    assert_eq!(
        c.advance_token(),
        Token::new(
            TokenKind::Keyword(KeywordKind::Fn),
            String::from("fn"),
            Location::new(0, 4)
        )
    );
    assert_eq!(
        c.advance_token(),
        Token::new(
            TokenKind::Keyword(KeywordKind::Loop),
            String::from("loop"),
            Location::new(0, 7)
        )
    );
    assert_eq!(
        c.advance_token(),
        Token::new(
            TokenKind::Keyword(KeywordKind::Break),
            String::from("break"),
            Location::new(0, 12)
        )
    );
    assert_eq!(
        c.advance_token(),
        Token::new(
            TokenKind::Keyword(KeywordKind::Include),
            String::from("include"),
            Location::new(0, 18)
        )
    );
    assert_eq!(
        c.advance_token(),
        Token::new(
            TokenKind::Keyword(KeywordKind::Extern),
            String::from("extern"),
            Location::new(0, 26)
        )
    );
    assert_eq!(
        c.advance_token(),
        Token::new(TokenKind::Ident, String::from("Let"), Location::new(0, 33))
    );
    assert_eq!(
        c.advance_token(),
        Token::new(TokenKind::Eof, String::from(EOF_CHAR), Location::new(0, 36))
    );
}

#[test]
fn test_string() {
    let mut c = Cursor::new(r#""hello world""#);
    assert_eq!(
        c.advance_token(),
        Token::new(TokenKind::String, String::from("hello world"), Location::new(0, 0))
    );

    let mut c1 = Cursor::new(r#""hi \"bob\"""#);
    assert_eq!(
        c1.advance_token(),
        // should expect {hi \"bob\"} as the string content 
        Token::new(TokenKind::String, String::from("hi \\\"bob\\\""), Location::new(0, 0))
    );
}