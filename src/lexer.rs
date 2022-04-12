use crate::token::{Lit, Token, TokenKind};

/// Helper struct for reading a string.
pub struct StringReader<'a> {
    pub current: usize,
    pub previous: usize,
    pub src: &'a str,
}

impl<'a> StringReader<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            src,
            current: 0,
            previous: 0,
        }
    }

    pub fn remaining(&self) -> &str {
        &self.src[self.current..]
    }

    pub fn peek(&self) -> Option<&str> {
        if self.remaining().is_empty() {
            None
        } else {
            let source = &self.src[self.current..];
            let mut end = 1;
            while !source.is_char_boundary(end) {
                end += 1;
            }

            Some(&source[0..end])
        }
    }

    pub fn advance(&mut self) -> Option<&str> {
        if self.remaining().is_empty() {
            None
        } else {
            let source = &self.src[self.current..];
            let mut end = 1;
            while !source.is_char_boundary(end) {
                end += 1;
            }

            self.current += end;

            Some(&source[0..end])
        }
    }

    pub fn next_token(&mut self) -> &str {
        let token = &self.src[self.previous..self.current];
        self.previous = self.current;

        token
    }
}

pub trait IntoStringReader {
    fn into_string_reader(&self) -> StringReader;
}

impl IntoStringReader for &str {
    fn into_string_reader(&self) -> StringReader {
        StringReader::new(&self)
    }
}

/// Generates a stream of [Token]s from input.
pub struct Lexer<'a> {
    pub reader: StringReader<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            reader: StringReader::new(src),
        }
    }

    pub fn tokenize(src: &'a str) -> Vec<Token> {
        let mut lexer = Lexer::new(src);
        let mut tokens = Vec::new();

        loop {
            if lexer.reader.peek().is_none() {
                tokens.push(lexer.next_token());
                break;
            } else {
                tokens.push(lexer.next_token());
            }
        }

        tokens
    }

    fn make_token(&mut self, kind: TokenKind) -> Token {
        let token = Token::new(kind);

        self.reader.previous = self.reader.current;

        token
    }

    fn skip_whitespace(&mut self) {
        while self.reader.peek().is_some() && is_whitespace(self.reader.peek().unwrap()) {
            self.reader.advance();
        }

        self.reader.previous = self.reader.current;
    }

    fn number(&mut self) -> Token {
        while self.reader.peek().is_some() && is_numeric(self.reader.peek().unwrap()) {
            self.reader.advance();
        }

        let num = &self.reader.next_token();

        Token::new(TokenKind::Literal(Lit::Integer(num.parse::<i64>().unwrap())))
    }

    pub fn next_token(&mut self) -> Token {
        let c = &self.reader.advance();

        match c {
            Some("+") => self.make_token(TokenKind::Plus),
            Some("-") => self.make_token(TokenKind::Hypen),
            Some("*") => self.make_token(TokenKind::Star),
            Some("/") => self.make_token(TokenKind::BackSlash),
            Some("%") => self.make_token(TokenKind::Modulo),
            Some("!") => self.make_token(TokenKind::Bang),
            Some("~") => self.make_token(TokenKind::Tilde),
            Some("(") => self.make_token(TokenKind::LParen),
            Some(")") => self.make_token(TokenKind::RParen),
            Some("[") => self.make_token(TokenKind::LBracket),
            Some("]") => self.make_token(TokenKind::RBracket),
            Some(c) => {
                if is_numeric(c) {
                    self.number()
                } else if is_whitespace(c) {
                    self.skip_whitespace();

                    self.next_token()
                } else {
                    todo!()
                }
            }
            None => Token::new(TokenKind::Eof),
        }
    }
}

fn is_numeric(c: &str) -> bool {
    c.bytes().all(|c| c.is_ascii_digit())
}

fn is_whitespace(c: &str) -> bool {
    c.bytes().all(|c| c.is_ascii_whitespace())
}

#[cfg(test)]
mod test {
    use crate::token::{Lit, Token, TokenKind};

    use super::Lexer;

    #[test]
    fn test_token() {
        let tokens = [
            ("1", Token::literal(Lit::Integer(1))),
            ("123", Token::literal(Lit::Integer(123))),
            ("+", Token::new(TokenKind::Plus)),
            ("-", Token::new(TokenKind::Hypen)),
            ("/", Token::new(TokenKind::BackSlash)),
            ("*", Token::new(TokenKind::Star)),
            ("~", Token::new(TokenKind::Tilde)),
            ("!", Token::new(TokenKind::Bang)),
            ("(", Token::new(TokenKind::LParen)),
            (")", Token::new(TokenKind::RParen)),
            (" ", Token::new(TokenKind::Eof)),
        ];

        for (string, token) in tokens {
            let lexer = &mut Lexer::new(string);

            assert_eq!(lexer.next_token(), token);
        }
    }
}
