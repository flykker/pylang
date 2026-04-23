use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use cranelift::prelude::types;
use cranelift::prelude::*;
use cranelift_codegen::{Context, settings, ir::UserFuncName};
use cranelift_module::{Module as ClifModule, ModuleResult, Linkage};
use cranelift_object::{ObjectModule, ObjectBuilder};
use pylang_front::ast::{Stmt, Expr};

pub fn write_simple_elf(output: &Path, ast: &[Stmt]) -> Result<(), String> {
    let runtime_o = compile_runtime_lib()?;
    
    let mut module = create_module()?;
    
    compile_all_functions(&mut module, ast).map_err(|e| format!("compile error: {:?}", e))?;
    
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

fn compile_all_functions(module: &mut ObjectModule, ast: &[Stmt]) -> ModuleResult<()> {
    let void_sig = module.make_signature();
    
    let mut print_int_sig = module.make_signature();
    print_int_sig.params.push(AbiParam::new(types::I64));
    let print_int_id = module.declare_function("print_int", Linkage::Import, &print_int_sig)?;
    
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
    
    let print_calls = extract_print_calls(ast);
    for arg in &print_calls {
        let val = match arg {
            PrintArg::Int(i) => bldr_main.ins().iconst(types::I64, *i),
            PrintArg::Str(s) => {
                let str_ptr = bldr_main.ins().iconst(types::I64, *s as i64);
                str_ptr
            }
        };
        let callee_print = module.declare_func_in_func(print_int_id, bldr_main.func);
        bldr_main.ins().call(callee_print, &[val]);
    }
    
    bldr_main.ins().return_(&[]);
    
    let id_start = module.declare_function("_start", Linkage::Export, &void_sig)?;
    module.define_function(id_start, &mut ctx_start)?;
    
    let id_main = module.declare_function("main", Linkage::Export, &void_sig)?;
    module.define_function(id_main, &mut ctx_main)?;
    
    Ok(())
}

enum PrintArg {
    Int(i64),
    Str(i64),
}

fn extract_print_calls(ast: &[Stmt]) -> Vec<PrintArg> {
    let mut calls = Vec::new();
    for stmt in ast {
        extract_print_from_stmt(stmt, &mut calls);
    }
    calls
}

fn extract_print_from_stmt(stmt: &Stmt, calls: &mut Vec<PrintArg>) {
    match stmt {
        Stmt::Fn(f) => {
            for s in &f.body {
                extract_print_from_stmt(s, calls);
            }
        }
        Stmt::Expr(e) => {
            extract_print_from_expr(e, calls);
        }
        Stmt::If(i) => {
            for s in &i.then {
                extract_print_from_stmt(s, calls);
            }
            for elif in &i.elif {
                for s in &elif.body {
                    extract_print_from_stmt(s, calls);
                }
            }
            if let Some(else_body) = &i.else_ {
                for s in else_body {
                    extract_print_from_stmt(s, calls);
                }
            }
        }
        Stmt::While(w) => {
            for s in &w.body {
                extract_print_from_stmt(s, calls);
            }
        }
        Stmt::For(f) => {
            for s in &f.body {
                extract_print_from_stmt(s, calls);
            }
        }
        Stmt::Try(t) => {
            for s in &t.body {
                extract_print_from_stmt(s, calls);
            }
            for handler in &t.handlers {
                for s in &handler.body {
                    extract_print_from_stmt(s, calls);
                }
            }
            if let Some(finally_body) = &t.finally {
                for s in finally_body {
                    extract_print_from_stmt(s, calls);
                }
            }
        }
        Stmt::Loop(l) => {
            for s in &l.body {
                extract_print_from_stmt(s, calls);
            }
        }
        _ => {}
    }
}

fn extract_print_from_expr(expr: &Expr, calls: &mut Vec<PrintArg>) {
    if let Expr::Call { func, args } = expr {
        if let Expr::Ident(name) = func.as_ref() {
            if name == "print" {
                for arg in args {
                    match arg {
                        Expr::Int(i) => calls.push(PrintArg::Int(*i)),
                        Expr::Str(s) => calls.push(PrintArg::Str(s.as_ptr() as i64)),
                        Expr::Bool(b) => calls.push(PrintArg::Int(if *b { 1 } else { 0 })),
                        Expr::Float(f) => calls.push(PrintArg::Int(*f as i64)),
                        Expr::Char(c) => calls.push(PrintArg::Int(*c as i64)),
                        _ => {}
                    }
                }
            }
        }
    }
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
            let s: &str = match x {
                0 => "0",
                1 => "1",
                2 => "2",
                3 => "3",
                4 => "4",
                5 => "5",
                6 => "6",
                7 => "7",
                8 => "8",
                9 => "9",
                10 => "10",
                11 => "11",
                12 => "12",
                13 => "13",
                14 => "14",
                15 => "15",
                16 => "16",
                17 => "17",
                18 => "18",
                19 => "19",
                20 => "20",
                21 => "21",
                22 => "22",
                23 => "23",
                24 => "24",
                25 => "25",
                26 => "26",
                27 => "27",
                28 => "28",
                29 => "29",
                30 => "30",
                31 => "31",
                32 => "32",
                33 => "33",
                34 => "34",
                35 => "35",
                36 => "36",
                37 => "37",
                38 => "38",
                39 => "39",
                40 => "40",
                41 => "41",
                42 => "42",
                43 => "43",
                44 => "44",
                45 => "45",
                46 => "46",
                47 => "47",
                48 => "48",
                49 => "49",
                50 => "50",
                51 => "51",
                52 => "52",
                53 => "53",
                54 => "54",
                55 => "55",
                56 => "56",
                57 => "57",
                58 => "58",
                59 => "59",
                60 => "60",
                61 => "61",
                62 => "62",
                63 => "63",
                64 => "64",
                65 => "65",
                66 => "66",
                67 => "67",
                68 => "68",
                69 => "69",
                70 => "70",
                71 => "71",
                72 => "72",
                73 => "73",
                74 => "74",
                75 => "75",
                76 => "76",
                77 => "77",
                78 => "78",
                79 => "79",
                80 => "80",
                81 => "81",
                82 => "82",
                83 => "83",
                84 => "84",
                85 => "85",
                86 => "86",
                87 => "87",
                88 => "88",
                89 => "89",
                90 => "90",
                91 => "91",
                92 => "92",
                93 => "93",
                94 => "94",
                95 => "95",
                96 => "96",
                97 => "97",
                98 => "98",
                99 => "99",
                100 => "100",
                101 => "101",
                102 => "102",
                103 => "103",
                104 => "104",
                105 => "105",
                106 => "106",
                107 => "107",
                108 => "108",
                109 => "109",
                110 => "110",
                111 => "111",
                112 => "112",
                113 => "113",
                114 => "114",
                115 => "115",
                116 => "116",
                117 => "117",
                118 => "118",
                119 => "119",
                120 => "120",
                _ => "?",
            };
            let ptr = s.as_ptr();
            let len = s.len();
            unsafe {
                core::arch::asm!(
                    "syscall",
                    in("rax") 1i64,
                    in("rdi") 1i64,
                    in("rsi") ptr as i64,
                    in("rdx") len as i64,
                    options(nostack)
                );
            }
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
        .arg("-C")
        .arg("link-arg=-lc")
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_elf() {
        let output = Path::new("/tmp/test_out.elf");
        let result = write_simple_elf(output, &[]);
        if result.is_ok() {
            let _ = std::fs::remove_file(output);
        }
        assert!(result.is_ok());
    }
}