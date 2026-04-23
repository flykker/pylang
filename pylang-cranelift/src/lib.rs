pub mod lower;
pub mod codegen;
pub mod emit;

use pylang_front::ast::Stmt;
use pylang_ir::Function;
use std::path::Path;

pub struct Compiler;

impl Compiler {
    pub fn new() -> Self {
        Self
    }

    pub fn compile(&self, ast: &[Stmt]) -> Result<Vec<Function>, String> {
        lower::lower_module(ast)
    }

    pub fn compile_to_elf(&self, ast: &[Stmt], output: &str) -> Result<(), String> {
        emit::write_simple_elf(Path::new(output), ast)?;
        Ok(())
    }
}

impl Default for Compiler {
    fn default() -> Self {
        Self
    }
}