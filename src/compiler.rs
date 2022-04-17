use crate::{
    ast::{Ast, BinOp, Expr, Lit, Stmt},
    chunk::Chunk,
    instruction::Instruction,
    scope::State,
};

pub struct Class {
    pub fields: usize,
}

impl Class {
    fn _new() -> Option<Class> {
        Some(Self { fields: 0 })
    }
}

pub struct Compiler<'a> {
    _state: &'a mut State,
    chunk: Chunk,
}

impl<'a> Compiler<'a> {
    pub fn new(state: &'a mut State) -> Self {
        Self {
            _state: state,
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
                Stmt::Class { name } => self.class(name),
                Stmt::VarDecl { id, init } => self.var_declartion(id, init),
                Stmt::Expr(expr) => self.expression(expr),
            }
        }

        self.emit_return();

        Ok(Chunk {
            instructions: self.chunk.instructions.clone(),
            variables: self.chunk.variables.clone(),
        })
    }

    fn class(&mut self, name: &str) {
        let index = self.chunk.add_variable(name);

        self.emit_inst(Instruction::Class { index });
        self.emit_inst(Instruction::DefineGlobal { index });

        //self.class = Class::new();

        //self.state.add_class(name);
    }

    fn var_declartion(&mut self, id: &str, init: &Expr) {
        self.expr(init);
        
        let index = self.chunk.add_variable(id);
        self.emit_inst(Instruction::DefineGlobal { index });
    }

    fn expression(&mut self, expr: &Expr) {
        self.expr(expr);

        self.emit_inst(Instruction::Pop);
    }

    fn expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Dot { object, property } => self.dot_expr(object, property),
            Expr::Literal(lit) => self.literal(lit),
            Expr::BinExpr { op, left, right } => self.binary_expr(op, left, right),
            Expr::Identifier(ident) => self.identifier(ident),
            Expr::Call { callee, .. } => self.call(callee),
            Expr::UnaryExpr { .. } => todo!(),
        }
    }

    fn binary_expr(&mut self, _op: &BinOp, left: &Expr, right: &Expr) {
        // PUSH 2
        self.expr(left);
        // PUSH 1
        self.expr(right);

        // add
        //let hash = Hash::of(op.into_string());

        // GET_PROP
        //self.emit_inst(Instruction::GetProperty { name: hash });

        // CALL
        self.emit_inst(Instruction::Call);

        // 1 + 2 -> 1.add(2)
        // PUSH 2
        // PUSH 1
        // OP_GET add
        // OP_CALL
        //
        //self.emit_inst(Instruction::CallInstance { hash, args: 2 });
    }

    fn dot_expr(&mut self, object: &Expr, property: &Expr) {
        self.expr(object);

        if let Expr::Identifier(name) = property {
            let index = self.chunk.add_variable(name.to_string());
            self.emit_inst(Instruction::GetProperty { index });
        }
    }

    fn call(&mut self, expr: &Expr) {
        self.expr(expr);

        self.emit_inst(Instruction::Call);
    }

    fn identifier(&mut self, name: &str) {
        let index = self.chunk.add_variable(name);
        self.emit_inst(Instruction::GetGlobal { index });
    }

    fn literal(&mut self, lit: &Lit) {
        match lit {
            Lit::Integer(v) => self.integer(*v),
            Lit::Float(_) => todo!(),
            Lit::Char(_) => todo!(),
            Lit::Nil => self.nil(),
            Lit::String(_) => todo!(),
        }
    }

    fn integer(&mut self, v: i64) {
        self.emit_inst(Instruction::integer(v));
    }

    fn _float(&mut self) {}

    fn _string(&mut self) {}

    fn nil(&mut self) {
        self.emit_inst(Instruction::LoadNil);
    }
}

#[cfg(test)]
mod test {
    use crate::{
        instruction::{InstanceValue, Instruction},
        lexer::Lexer,
        parser,
        scope::State,
    };

    #[test]
    fn compile_literal() {
        let ast = parser::Parser::new(Lexer::tokenize("1")).parse().unwrap();
        let scope = &mut State::new();

        let mut compiler = super::Compiler::new(scope);

        let chunk = compiler.run(ast).unwrap();

        assert_eq!(
            chunk.instructions[0],
            Instruction::Push {
                value: InstanceValue::Integer(1)
            }
        );
    }
}
