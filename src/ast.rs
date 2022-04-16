/// An AST (abstract syntax tree).
#[derive(Debug)]
pub struct Ast {
    pub nodes: Vec<Stmt>,
}

#[derive(Debug, PartialEq)]
pub enum AstNode {
    /// A statement.
    Stmt(Stmt),
    /// An expression
    Expr(Expr),
}

#[derive(Debug, PartialEq)]
pub enum Stmt {
    Class {
        name: String,
    },
    Expr(Expr),
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    /// An identifier
    Identifier(String),
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
    Call { callee: Box<Expr>, args: Vec<Expr> },
    /// A dot expression. 
    Dot { object: Box<Expr>, property: Box<Expr> },
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
    Integer(i64),
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
    Rem,
    /// !
    Bang,
}

impl BinOp {
    pub fn into_string(&self) -> &str {
        match self {
            BinOp::Add => "+",
            BinOp::Sub => "-",
            BinOp::Mul => "*",
            BinOp::Div => "/",
            BinOp::Rem => "%",
            BinOp::Bang => "!",
        }
    }
}
