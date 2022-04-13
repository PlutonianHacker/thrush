#[derive(Debug, PartialEq, Clone)]
pub enum Lit {
    Integer(i64),
    String(String),
    Float(f64),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Keyword {
    Class,
    Fun,
    Var,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Symbol {}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    /// +
    Plus,
    /// -
    Hypen,
    /// *
    Star,
    /// /
    BackSlash,
    /// %
    Modulo,
    /// .
    Dot,
    /// ~
    Tilde,
    /// !
    Bang,
    /// ,
    Comma,
    /// =
    Assign,

    /// (
    LParen,
    /// )
    RParen,
    /// [
    LBracket,
    /// ]
    RBracket,
    /// {
    LBrace,
    /// }
    RBrace,

    Literal(Lit),
    Ident(Symbol),
    Keyword(Keyword),

    /// <eof>
    Eof,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub kind: TokenKind,
}

impl Token {
    pub fn new(kind: TokenKind) -> Self {
        Self { kind }
    }

    pub fn literal(lit: Lit) -> Self {
        Self {
            kind: TokenKind::Literal(lit),
        }
    }
}
