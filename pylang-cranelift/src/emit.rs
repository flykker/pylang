use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Command;
use cranelift::prelude::types;
use cranelift::prelude::*;
use cranelift_codegen::{settings, ir::UserFuncName};
use cranelift_module::{Module as ClifModule, Linkage};
use cranelift_object::{ObjectModule, ObjectBuilder};
use pylang_front::ast::{Stmt, Type as AstType};

use crate::lower::lower_module;

pub fn write_simple_elf(
    output: &Path,
    ast: &[Stmt],
    fn_var_types: &HashMap<String, HashMap<String, AstType>>,
) -> Result<(), String> {
    let runtime_o = compile_runtime_lib()?;
    
    let mut module = create_module()?;
    
    // Lower all Python functions to CLIF via the new lowering pipeline
    let func_ids = lower_module(&mut module, ast, fn_var_types)?;
    
    // Ensure main exists — if not, create an empty one
    let _main_fn = ast.iter().find(|s| matches!(s, Stmt::Fn(f) if f.name == "main"));
    
    let main_id = *func_ids.get("main").ok_or("main function not found")?;
    
    // Create signature for _start (no returns - entry point)
    let start_sig = module.make_signature();
    let start_id = module.declare_function("_start", Linkage::Export, &start_sig)
        .map_err(|e| e.to_string())?;
    
    let mut ctx = module.make_context();
    ctx.func.signature = start_sig.clone();
    ctx.func.name = UserFuncName::user(0, 0);
    
    let mut fn_ctx = FunctionBuilderContext::new();
    let mut builder = FunctionBuilder::new(&mut ctx.func, &mut fn_ctx);
    let entry = builder.create_block();
    builder.seal_block(entry);
    builder.switch_to_block(entry);
    
    let _dummy_slot = builder.create_sized_stack_slot(StackSlotData::new(StackSlotKind::ExplicitSlot, 32, 16));
    
    // Call _init_module before main if it exists (for decorator desugaring, etc.)
    if let Some(&init_id) = func_ids.get("_init_module") {
        let callee_init = module.declare_func_in_func(init_id, builder.func);
        builder.ins().call(callee_init, &[]);
    }
    
    // Call main without parameters
    let callee_main = module.declare_func_in_func(main_id, builder.func);
    builder.ins().call(callee_main, &[]);
    
    let zero = builder.ins().iconst(types::I64, 0);
    let mut exit_sig = module.make_signature();
    exit_sig.params.push(AbiParam::new(types::I64));
    exit_sig.returns.push(AbiParam::new(types::I64));
    let exit_id = module.declare_function("exit", Linkage::Import, &exit_sig)
        .map_err(|e| e.to_string())?;
    let callee_exit = module.declare_func_in_func(exit_id, builder.func);
    builder.ins().call(callee_exit, &[zero]);
    builder.ins().return_(&[]);
    
    builder.finalize();
    if let Err(e) = module.define_function(start_id, &mut ctx) {
        return Err(format!("Verifier error: {}", e));
    }
    
    // Emit object file
    let product = module.finish();
    let mut obj_data = Vec::new();
    product.object.emit(&mut obj_data)
        .map_err(|e| e.to_string())?;
    
    let obj_path = Path::new(output).with_extension("o");
    std::fs::write(&obj_path, &obj_data)
        .map_err(|e| e.to_string())?;
    
    let status = Command::new("ld")
        .arg("-e").arg("_start")
        .arg("-o").arg(output)
        .arg(&obj_path)
        .arg(&runtime_o)
        .status()
        .map_err(|e| e.to_string())?;
    
    let _ = std::fs::remove_file(&obj_path);
    let _ = std::fs::remove_file(&runtime_o);
    
    if !status.success() {
        return Err("ld failed".to_string());
    }
    
    Ok(())
}

fn compile_runtime_lib() -> Result<PathBuf, String> {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let runtime_src = PathBuf::from(manifest_dir)
        .join("..")
        .join("pylang-runtime")
        .join("src")
        .join("lib.rs");
    
    let tmp_dir = std::env::temp_dir();
    let so_path = tmp_dir.join("pylang_runtime.so");
    let obj_path = tmp_dir.join("pylang_runtime.o");
    
    let status = Command::new("rustc")
        .arg("--emit=obj")
        .arg("--crate-type=cdylib")
        .arg("-C").arg("panic=abort")
        .arg("-O")
        .arg("-o").arg(&so_path)
        .arg(&runtime_src)
        .status()
        .map_err(|e| e.to_string())?;
    
    if !status.success() {
        return Err("rustc failed".to_string());
    }
    
    let status = Command::new("ld")
        .arg("-r")
        .arg("-o").arg(&obj_path)
        .arg(&so_path)
        .status()
        .map_err(|e| e.to_string())?;
    
    if !status.success() {
        return Err("ld -r failed".to_string());
    }
    
    let _ = std::fs::remove_file(&so_path);
    
    Ok(obj_path)
}

fn create_module() -> Result<ObjectModule, String> {
    let isa = cranelift_native::builder()
        .map_err(|e| format!("unsupported target: {}", e))?
        .finish(settings::Flags::new(settings::builder()))
        .map_err(|e| format!("finish error: {}", e))?;

    let builder = ObjectBuilder::new(
        isa,
        "pylang",
        cranelift_module::default_libcall_names(),
    ).map_err(|e| format!("builder error: {}", e))?;

    Ok(ObjectModule::new(builder))
}

#[cfg(test)]
mod tests {
    use super::*;
    use pylang_front::ast::Expr;

    #[test]
    fn test_write_elf() {
        let output = Path::new("/tmp/test_out.elf");
        let ast = vec![Stmt::Fn(pylang_front::ast::Fn {
            name: "main".to_string(),
            params: vec![],
            ret: Some(pylang_front::ast::Type::I64),
            body: vec![
                Stmt::Expr(Expr::Call {
                    func: Box::new(Expr::Ident("print".to_string())),
                    args: vec![Expr::Int(42)],
                }),
                Stmt::Return(pylang_front::ast::Return { val: Some(Expr::Int(0)) }),
            ],
            decorators: vec![],
            captures: vec![],
        })];
        let empty_map = HashMap::new();
        let result = write_simple_elf(output, &ast, &empty_map);
        if let Err(ref e) = result {
            eprintln!("test_write_elf error: {}", e);
        }
        if result.is_ok() {
            let _ = std::fs::remove_file(output);
        }
        assert!(result.is_ok());
    }
}
