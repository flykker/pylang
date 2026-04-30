pub mod lower;
pub mod emit;

use std::collections::HashMap;
use pylang_front::ast::{Stmt, Type as AstType};
use std::path::Path;

pub struct Compiler;

impl Compiler {
    pub fn new() -> Self {
        Self
    }

    pub fn compile_to_elf(&self, ast: &[Stmt], output: &str) -> Result<(), String> {
        let empty_map = HashMap::new();
        emit::write_simple_elf(Path::new(output), ast, &empty_map)
    }

    pub fn compile_to_elf_with_types(
        &self,
        ast: &[Stmt],
        output: &str,
        fn_var_types: &HashMap<String, HashMap<String, AstType>>,
    ) -> Result<(), String> {
        emit::write_simple_elf(Path::new(output), ast, fn_var_types)
    }
}

impl Default for Compiler {
    fn default() -> Self {
        Self
    }
}
