use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use cranelift::prelude::types;
use cranelift::prelude::*;
use cranelift_codegen::{Context, settings, ir::UserFuncName};
use cranelift_module::{Module as ClifModule, ModuleResult, Linkage};
use cranelift_object::{ObjectModule, ObjectBuilder};

pub fn write_simple_elf(output: &Path) -> Result<(), String> {
    let runtime_o = compile_runtime_lib()?;
    
    let mut module = create_module()?;
    
    compile_all_functions(&mut module).map_err(|e| format!("compile error: {:?}", e))?;
    
    let product = module.finish();
    
    let mut obj_data = Vec::new();
    product.object
        .emit(&mut obj_data)
        .map_err(|e| format!("emit error: {}", e))?;
    
    let obj_path = Path::new(output).with_extension("o");
    let mut file = File::create(&obj_path)
        .map_err(|e| format!("create file error: {}", e))?;
    file.write_all(&obj_data)
        .map_err(|e| format!("write error: {}", e))?;
    drop(file);
    
    let status = Command::new("ld")
        .arg("-e")
        .arg("_start")
        .arg("-o")
        .arg(output)
        .arg(&obj_path)
        .arg(&runtime_o)
        .arg("-dynamic-linker")
        .arg("/lib64/ld-linux-x86-64.so.2")
        .status()
        .map_err(|e| format!("ld error: {}", e))?;
    
    let _ = std::fs::remove_file(&obj_path);
    let _ = std::fs::remove_file(&runtime_o);
    
    if !status.success() {
        return Err("ld failed".to_string());
    }
    
    Ok(())
}

fn compile_runtime_lib() -> Result<PathBuf, String> {
    let runtime_src = r#"
        #![crate_type = "cdylib"]
        #![allow(unused)]
        
        #[no_mangle]
        pub extern "C" fn exit(code: i32) -> ! {
            unsafe {
                core::arch::asm!("mov rax, 60; syscall", in("rax") code);
                core::hint::unreachable_unchecked();
            }
        }
        
        #[no_mangle]
        pub extern "C" fn print_int(x: i64) {
            let _ = x;
        }
        
        #[no_mangle]
        pub extern "C" fn alloc(size: usize) -> *mut u8 {
            static mut HEAP: [u8; 65536] = [0; 65536];
            static mut POS: usize = 0;
            unsafe {
                if POS + size > 65536 {
                    core::ptr::null_mut()
                } else {
                    let ptr = core::ptr::addr_of_mut!(HEAP).cast::<u8>();
                    let current_pos = POS;
                    POS += size;
                    ptr.add(current_pos)
                }
            }
        }
    "#;
    
    let tmp_dir = std::env::temp_dir();
    let src_path = tmp_dir.join("pylang_runtime.rs");
    let obj_path = tmp_dir.join("libpylang_runtime.so");
    
    std::fs::write(&src_path, runtime_src)
        .map_err(|e| format!("write runtime src error: {}", e))?;
    
    let status = Command::new("rustc")
        .arg("--emit=obj")
        .arg("--target=x86_64-unknown-linux-gnu")
        .arg("-C")
        .arg("panic=abort")
        .arg("-O")
        .arg("-o")
        .arg(&obj_path)
        .arg(&src_path)
        .status()
        .map_err(|e| format!("rustc error: {}", e))?;
    
    let _ = std::fs::remove_file(&src_path);
    
    if !status.success() {
        return Err("rustc failed".to_string());
    }
    
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

fn compile_all_functions(module: &mut ObjectModule) -> ModuleResult<()> {
    let void_sig = module.make_signature();
    
    let mut exit_sig = module.make_signature();
    exit_sig.params.push(AbiParam::new(types::I32));
    
    let main_id = module.declare_function("main", Linkage::Import, &void_sig)?;
    let exit_id = module.declare_function("exit", Linkage::Import, &exit_sig)?;
    
    let mut ctx_start = Context::new();
    let mut fb_ctx_start = FunctionBuilderContext::new();
    ctx_start.func.signature = void_sig.clone();
    ctx_start.func.name = UserFuncName::user(0, 0);
    
    let mut bldr_start = FunctionBuilder::new(&mut ctx_start.func, &mut fb_ctx_start);
    let entry_start = bldr_start.create_block();
    bldr_start.switch_to_block(entry_start);
    bldr_start.seal_block(entry_start);
    
    let callee_main = module.declare_func_in_func(main_id, bldr_start.func);
    bldr_start.ins().call(callee_main, &[]);
    
    let zero = bldr_start.ins().iconst(types::I32, 0);
    let callee_exit = module.declare_func_in_func(exit_id, bldr_start.func);
    bldr_start.ins().call(callee_exit, &[zero]);
    
    bldr_start.ins().return_(&[]);
    
    let mut ctx_main = Context::new();
    let mut fb_ctx_main = FunctionBuilderContext::new();
    ctx_main.func.signature = void_sig.clone();
    ctx_main.func.name = UserFuncName::user(0, 1);
    
    let mut bldr_main = FunctionBuilder::new(&mut ctx_main.func, &mut fb_ctx_main);
    let entry_main = bldr_main.create_block();
    bldr_main.switch_to_block(entry_main);
    bldr_main.seal_block(entry_main);
    
    bldr_main.ins().return_(&[]);
    
    let id_start = module.declare_function("_start", Linkage::Export, &void_sig)?;
    module.define_function(id_start, &mut ctx_start)?;
    
    let id_main = module.declare_function("main", Linkage::Export, &void_sig)?;
    module.define_function(id_main, &mut ctx_main)?;
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_elf() {
        let output = Path::new("/tmp/test_out.elf");
        let result = write_simple_elf(output);
        if result.is_ok() {
            let _ = std::fs::remove_file(output);
        }
        assert!(result.is_ok());
    }
}