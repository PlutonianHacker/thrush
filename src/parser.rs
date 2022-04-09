use std::mem;

use crate::{
    ast::{Ast, AstNode, BinOp, Expr, Lit},
    token::{self, Token, TokenKind},
};

/// Defines the precedence of different operators and expressions.
#[repr(u8)]
#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
pub enum Precedence {
    None = 0,
    Sum,  // +, -
    Term, // *, /, %
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
            TokenKind::Eof | TokenKind::RParen => Precedence::End,
            kind => todo!("No rule implemented for {kind:?}"),
        }
    }

    /// Parse a statement.
    pub fn statement(&mut self) -> Result<AstNode, String> {
        Ok(AstNode::Expr(self.expression(Precedence::None)?))
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

                left = Expr::binary_expr(
                    BinOp::Modulo,
                    left,
                    self.expression(Precedence::Term.left())?,
                );
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
}

#[cfg(test)]
pub mod test {
    use crate::{
        ast::{AstNode, BinOp, Expr, Lit},
        lexer::Lexer,
    };

    use super::Parser;

    #[test]
    fn test_parser() {
        let mut parser = Parser::new(Lexer::tokenize("4 + 2 * 5"));

        assert_eq!(
            parser.parse().unwrap().nodes[0],
            AstNode::Expr(Expr::BinExpr {
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
