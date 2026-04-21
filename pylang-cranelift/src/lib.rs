pub mod lower;

use pylang_front::ast::Stmt;
use pylang_ir::Function;

pub struct Compiler;

impl Compiler {
    pub fn new() -> Self {
        Self
    }

    pub fn compile(&self, ast: &[Stmt]) -> Result<Vec<Function>, String> {
        lower::lower_module(ast)
    }
}