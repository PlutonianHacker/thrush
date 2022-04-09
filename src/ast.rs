/// An AST (abstract syntax tree).
#[derive(Debug)]
pub struct Ast {
    pub nodes: Vec<AstNode>,
}

#[derive(Debug, PartialEq)]
pub enum AstNode {
    /// An expression
    Expr(Expr),
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    /// A literal
    Literal(Lit),
    /// A binary expression
    BinExpr {
        left: Box<Expr>,
        right: Box<Expr>,
        op: BinOp,
    },
    /// An unary expression
    UnaryExpr { value: Box<Expr>, op: BinOp },
}

impl Expr {
    pub fn binary_expr(op: BinOp, left: Expr, right: Expr) -> Expr {
        Expr::BinExpr {
            left: Box::new(left),
            right: Box::new(right),
            op,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Lit {
    /// A string literal
    String(String),
    /// An integer literal
    Integer(u64),
    /// A float literal
    Float(f64),
    /// A character literal
    Char(char),
    /// A nil literal
    Nil,
}

/// A binary operator.
#[derive(Debug, PartialEq)]
pub enum BinOp {
    /// +
    Add,
    /// -
    Sub,
    /// *
    Mul,
    /// /
    Div,
    /// %
    Modulo,
    /// !
    Bang,
}

impl BinOp {
    pub fn precedence(&self) -> usize {
        match &self {
            BinOp::Mul | BinOp::Div | BinOp::Modulo => 3,
            BinOp::Sub | BinOp::Add => 2,
            BinOp::Bang => 1,
        }
    }
}
