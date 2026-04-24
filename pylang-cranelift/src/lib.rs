pub mod lower;
pub mod emit;

use pylang_front::ast::Stmt;
use std::path::Path;

pub struct Compiler;

impl Compiler {
    pub fn new() -> Self {
        Self
    }

    pub fn compile_to_elf(&self, ast: &[Stmt], output: &str) -> Result<(), String> {
        emit::write_simple_elf(Path::new(output), ast)
    }
}

impl Default for Compiler {
    fn default() -> Self {
        Self
    }
}
