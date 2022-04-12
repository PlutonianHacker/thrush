use crate::{
    ast::{Ast, AstNode, BinOp, Expr, Lit},
    chunk::Chunk,
    hash::Hash,
    instruction::Instruction,
};

pub struct Compiler {
    chunk: Chunk,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            chunk: Chunk::new(),
        }
    }

    pub fn emit_inst(&mut self, inst: Instruction) {
        self.chunk.instructions.push(inst);
    }

    pub fn emit_return(&mut self) {
        self.emit_inst(Instruction::Halt);
    }

    pub fn run(&mut self, ast: Ast) -> Result<Chunk, String> {
        for node in ast.nodes {
            match &node {
                AstNode::Expr(expr) => self.expression(expr),
            }
        }

        self.emit_return();

        Ok(Chunk {
            instructions: self.chunk.instructions.clone(),
        })
    }

    pub fn expression(&mut self, expr: &Expr) {
        match expr {
            Expr::Literal(lit) => self.literal(lit),
            Expr::BinExpr { op, left, right } => self.binary_expr(op, left, right),
            Expr::UnaryExpr { .. } => todo!(),
        }

        self.emit_inst(Instruction::Pop);
    }

    pub fn binary_expr(&mut self, op: &BinOp, left: &Expr, right: &Expr) {
        self.expression(left);
        self.expression(right);

        let hash = Hash::of(op.into_string());

        // 1 + 2 -> 1.add(2)
        self.emit_inst(Instruction::CallInstance { hash, args: 2 });
    }

    pub fn literal(&mut self, lit: &Lit) {
        match lit {
            Lit::Integer(v) => self.integer(*v),
            Lit::Float(_) => todo!(),
            Lit::Char(_) => todo!(),
            Lit::Nil => todo!(),
            Lit::String(_) => todo!(),
        }
    }

    pub fn integer(&mut self, v: i64) {
        self.emit_inst(Instruction::integer(v));
    }

    pub fn float(&mut self) {}

    pub fn string(&mut self) {}
}

#[cfg(test)]
mod test {
    use crate::{
        instruction::{InstanceValue, Instruction},
        lexer::Lexer,
        parser,
    };

    #[test]
    fn compile_literal() {
        let ast = parser::Parser::new(Lexer::tokenize("1")).parse().unwrap();

        let mut compiler = super::Compiler::new();

        let chunk = compiler.run(ast).unwrap();

        assert_eq!(
            chunk.instructions[0],
            Instruction::Push {
                value: InstanceValue::Integer(1)
            }
        );
    }
}
