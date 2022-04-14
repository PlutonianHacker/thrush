use std::mem;

use crate::{
    ast::{Ast, BinOp, Expr, Lit, Stmt},
    token::{self, Keyword, Token, TokenKind},
};

/// Defines the precedence of different operators and expressions.
#[repr(u8)]
#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
pub enum Precedence {
    None = 0,
    Sum,  // +, -
    Term, // *, /, %
    Call,
    End,
}

impl Precedence {
    pub fn left(&self) -> Precedence {
        unsafe { mem::transmute(*self as u8 + 1) }
    }
}

/// Parser for the Thrush langauge.
pub struct Parser {
    pub tokens: Vec<Token>,
    pub current: Token,
    pub pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: Token::new(TokenKind::Eof),
            pos: 0,
        }
    }

    // TODO: add error handling.
    /// Consume the current token, and get the next one from the token stream.
    pub fn consume(&mut self) {
        if self.pos + 1 <= self.tokens.len() - 1 {
            self.pos += 1;
            self.current = self.tokens[self.pos].clone();
        }
    }

    /// Get the precedence rule for the current token.
    pub fn prec(&self) -> Precedence {
        match &self.current.kind {
            TokenKind::Plus | TokenKind::Hypen => Precedence::Sum,
            TokenKind::Star | TokenKind::BackSlash | TokenKind::Modulo => Precedence::Term,
            TokenKind::LParen => Precedence::Call,
            TokenKind::Eof | TokenKind::RParen => Precedence::End,
            kind => todo!("No rule implemented for {kind:?}"),
        }
    }

    /// Parse a statement.
    fn statement(&mut self) -> Result<Stmt, String> {
        match &self.current.kind {
            TokenKind::Keyword(keyword) => match keyword {
                Keyword::Class => self.class(),
                _ => todo!(),
            },
            _ => self.expr(),
        }
    }

    /// Parse a class declaration.
    fn class(&mut self) -> Result<Stmt, String> {
        self.consume();

        let name = self.identifier()?;

        let class = Stmt::Class { name };

        // {
        self.consume();
        // }
        self.consume();

        Ok(class)
    }

    /// Parse a expression and a newline.
    pub fn expr(&mut self) -> Result<Stmt, String> {
        let expr = self.expression(Precedence::None)?;

        match &self.current.kind {
            TokenKind::Newline => {
                self.consume();

                Ok(Stmt::Expr(expr))
            }
            TokenKind::Eof | TokenKind::RBrace => Ok(Stmt::Expr(expr)),
            _ => panic!("Unexpected token"),
        }
    }

    /// Parse an expression.
    pub fn expression(&mut self, prec: Precedence) -> Result<Expr, String> {
        let mut left = self.literal()?;

        while self.prec() >= prec && self.prec() != Precedence::End {
            left = self.infix_expr(left)?;
        }

        Ok(left)
    }

    /// Parse a binary expression.
    pub fn infix_expr(&mut self, mut left: Expr) -> Result<Expr, String> {
        match &self.current.kind {
            TokenKind::Plus => {
                self.consume();

                left =
                    Expr::binary_expr(BinOp::Add, left, self.expression(Precedence::Sum.left())?);
            }
            TokenKind::Hypen => {
                self.consume();

                left =
                    Expr::binary_expr(BinOp::Sub, left, self.expression(Precedence::Sum.left())?);
            }
            TokenKind::Star => {
                self.consume();

                left =
                    Expr::binary_expr(BinOp::Mul, left, self.expression(Precedence::Term.left())?);
            }
            TokenKind::BackSlash => {
                self.consume();

                left =
                    Expr::binary_expr(BinOp::Div, left, self.expression(Precedence::Term.left())?);
            }
            TokenKind::Modulo => {
                self.consume();

                left =
                    Expr::binary_expr(BinOp::Rem, left, self.expression(Precedence::Term.left())?);
            }
            TokenKind::LParen => {
                self.consume();

                left = Expr::Call {
                    callee: Box::new(left),
                    args: Vec::new(),
                };

                self.consume();
            }
            _ => {}
        }

        Ok(left)
    }

    /// Parse a literal.
    pub fn literal(&mut self) -> Result<Expr, String> {
        match &self.current.kind.clone() {
            TokenKind::Literal(literal) => match literal {
                token::Lit::Integer(int) => {
                    self.consume();
                    Ok(Expr::Literal(Lit::Integer(*int)))
                }
                token::Lit::String(_) => todo!(),
                token::Lit::Float(_) => todo!(),
            },
            TokenKind::Hypen => {
                self.consume();

                Ok(Expr::UnaryExpr {
                    value: Box::new(self.expression(Precedence::End)?),
                    op: BinOp::Sub,
                })
            }
            TokenKind::Plus => {
                self.consume();

                Ok(Expr::UnaryExpr {
                    value: Box::new(self.expression(Precedence::End)?),
                    op: BinOp::Add,
                })
            }
            TokenKind::Bang => {
                self.consume();

                Ok(Expr::UnaryExpr {
                    value: Box::new(self.expression(Precedence::End)?),
                    op: BinOp::Bang,
                })
            }
            TokenKind::LParen => {
                self.consume();
                let node = self.expression(Precedence::None.left())?;
                self.consume();

                Ok(node)
            }
            TokenKind::Ident(_) => Ok(Expr::Identifier(self.identifier()?)),
            _ => Err("unexpected token".into()),
        }
    }

    /// Constructs an [Ast] from a stream of tokens.
    pub fn parse(&mut self) -> Result<Ast, String> {
        self.current = self.tokens[self.pos].clone();

        let mut nodes = Vec::new();
        while self.current.kind != TokenKind::Eof {
            nodes.push(self.statement()?);
        }

        Ok(Ast { nodes })
    }

    pub fn parse_ast(tokens: Vec<Token>) -> Result<Ast, String> {
        let mut parser = Parser::new(tokens);

        parser.parse()
    }

    fn identifier(&mut self) -> Result<String, String> {
        if let TokenKind::Ident(name) = &self.current.kind {
            let name = name.to_string();
            self.consume();

            Ok(name)
        } else {
            Err("expected a identifier".into())
        }
    }
}

#[cfg(test)]
pub mod test {
    use crate::{
        ast::{BinOp, Expr, Lit, Stmt},
        lexer::Lexer,
    };

    use super::Parser;

    #[test]
    fn test_parser() {
        let mut parser = Parser::new(Lexer::tokenize("4 + 2 * 5"));

        assert_eq!(
            parser.parse().unwrap().nodes[0],
            Stmt::Expr(Expr::BinExpr {
                left: Box::new(Expr::Literal(Lit::Integer(4))),
                right: Box::new(Expr::BinExpr {
                    left: Box::new(Expr::Literal(Lit::Integer(2))),
                    right: Box::new(Expr::Literal(Lit::Integer(5))),
                    op: BinOp::Mul,
                }),
                op: BinOp::Add,
            })
        );
    }
}
