use cranelift::prelude::*;
use cranelift_codegen::ir::{UserFuncName, InstBuilder, BlockArg};
use cranelift_module::{Module, Linkage, FuncId};
use pylang_front::ast::{
    Stmt, Expr, Type as AstType, BinOp, Fn as AstFn, CmpOp, UnOp,
    If, While, For, Loop, Match, Try, With, Raise, Assert, Yield,
    Param,
};
use std::collections::HashMap;

#[derive(Clone)]
struct StructField {
    _name: String,
    offset: i64,
    _ty: Type,
}

#[derive(Clone)]
pub struct StructInfo {
    _name: String,
    fields: Vec<StructField>,
}

#[derive(Clone)]
pub struct ClassInfo {
    _name: String,
    _fields: Vec<StructField>,
    field_defaults: Vec<i64>,
    _methods: HashMap<String, String>,
}

#[derive(Clone)]
pub struct ClosureInfo {
    pub mangled_name: String,
    pub func_id: FuncId,
    pub capture_names: Vec<String>,
}

struct LoopContext {
    exit_block: Block,
    continue_block: Block,
}

pub struct LowerCtx<'a> {
    pub builder: FunctionBuilder<'a>,
    pub module: &'a mut dyn Module,
    pub locals: HashMap<String, Variable>,
    pub func_ids: HashMap<String, FuncId>,
    pub closure_defs: HashMap<String, ClosureInfo>,
    pub block_filled: bool,
    loop_stack: Vec<LoopContext>,
    pub struct_defs: HashMap<String, StructInfo>,
    pub class_defs: HashMap<String, ClassInfo>,
}

impl<'a> LowerCtx<'a> {
    fn switch_to_block(&mut self, block: Block) {
        self.builder.switch_to_block(block);
        self.block_filled = false;
    }

    fn create_block(&mut self) -> Block {
        self.builder.create_block()
    }

    fn push_loop(&mut self, exit_block: Block, continue_block: Block) {
        self.loop_stack.push(LoopContext {
            exit_block,
            continue_block,
        });
    }

    fn pop_loop(&mut self) {
        self.loop_stack.pop();
    }

    fn break_loop(&mut self) {
        if let Some(ctx) = self.loop_stack.last() {
            self.builder.ins().jump(ctx.exit_block, &[]);
            self.block_filled = true;
        }
    }

    fn continue_loop(&mut self) {
        if let Some(ctx) = self.loop_stack.last() {
            self.builder.ins().jump(ctx.continue_block, &[]);
            self.block_filled = true;
        }
    }
}

pub fn lower_module(module: &mut dyn Module, stmts: &[Stmt]) -> Result<HashMap<String, FuncId>, String> {
    let mut struct_defs: HashMap<String, StructInfo> = HashMap::new();
    let mut class_defs: HashMap<String, ClassInfo> = HashMap::new();
    let mut func_ids: HashMap<String, FuncId> = HashMap::new();
    let mut closure_defs: HashMap<String, ClosureInfo> = HashMap::new();

    // First pass: collect all struct and class definitions (without methods)
    for stmt in stmts {
        match stmt {
            Stmt::Struct(s) => {
                let mut offset = 0i64;
                let mut fields: Vec<StructField> = Vec::new();
                for (name, ty) in &s.fields {
                    let ty = ast_type_to_clif(ty)?;
                    fields.push(StructField {
                        _name: name.clone(),
                        offset,
                        _ty: ty,
                    });
                    offset += 8;
                }
                struct_defs.insert(s.name.clone(), StructInfo {
                    _name: s.name.clone(),
                    fields,
                });
            }
            Stmt::Class(c) => {
                let mut offset = 0i64;
                let mut fields: Vec<StructField> = Vec::new();
                let mut field_defaults: Vec<i64> = Vec::new();
                let mut methods: HashMap<String, String> = HashMap::new();
                
                for item in &c.body {
                    match item {
                        Stmt::Fn(f) => {
                            let method_name = if f.name == "__init__" {
                                format!("{}_init", c.name)
                            } else {
                                format!("{}_{}", c.name, f.name)
                            };
                            methods.insert(f.name.clone(), method_name.clone());
                        }
                        Stmt::Let(l) => {
                            fields.push(StructField {
                                _name: l.name.clone(),
                                offset,
                                _ty: types::I64,
                            });
                            let default_val = extract_int_from_expr(&l.val);
                            field_defaults.push(default_val);
                            offset += 8;
                        }
                        Stmt::Assign(a) => {
                            if let Expr::Dot { obj, name } = &*a.target {
                                if let Expr::Ident(s) = &**obj {
                                    if s == "self" {
                                        fields.push(StructField {
                                            _name: name.clone(),
                                            offset,
                                            _ty: types::I64,
                                        });
                                        let default_val = extract_int_from_expr(&a.val);
                                        field_defaults.push(default_val);
                                        offset += 8;
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
                class_defs.insert(c.name.clone(), ClassInfo {
                    _name: c.name.clone(),
                    _fields: fields,
                    field_defaults,
                    _methods: methods,
                });
            }
            _ => {}
        }
    }
    
    // Second pass: lower all functions (including class methods)
    for stmt in stmts {
        match stmt {
            Stmt::Class(c) => {
                for item in &c.body {
                    if let Stmt::Fn(f) = item {
                        let method_name = if f.name == "__init__" {
                            format!("{}_init", c.name)
                        } else {
                            format!("{}_{}", c.name, f.name)
                        };
                        
                        let has_self = f.params.first().map(|p| p.name == "self").unwrap_or(false);
                        let params = if has_self {
                            f.params.clone()
                        } else {
                            let mut p = vec![
                                pylang_front::ast::Param {
                                    name: "self".to_string(),
                                    ty: pylang_front::ast::Type::Named(c.name.clone()),
                                    default: None,
                                }
                            ];
                            p.extend(f.params.iter().cloned());
                            p
                        };
                        
                        let method_fn = pylang_front::ast::Fn {
                            name: method_name.clone(),
                            params,
                            ret: f.ret.clone(),
                            body: f.body.clone(),
                            decorators: vec![],
                            captures: vec![],
                        };
                        let id = lower_fn(module, &method_fn, &struct_defs, &class_defs, &mut func_ids, &mut closure_defs)?;
                        func_ids.insert(method_name, id);
                    }
                }
            }
            Stmt::Fn(f) => {
                let id = lower_fn(module, f, &struct_defs, &class_defs, &mut func_ids, &mut closure_defs)?;
                func_ids.insert(f.name.clone(), id);
            }
            _ => {}
        }
    }
    
    // Third pass: lower module-level non-function statements (decorator desugaring, etc.)
    let module_stmts: Vec<Stmt> = stmts.iter().filter(|s| {
        !matches!(s, Stmt::Fn(_) | Stmt::Class(_) | Stmt::Struct(_))
    }).cloned().collect();
    
    if !module_stmts.is_empty() {
        let init_id = lower_module_init(module, &module_stmts, &struct_defs, &class_defs, &mut func_ids, &mut closure_defs)?;
        func_ids.insert("_init_module".to_string(), init_id);
    }
    
    Ok(func_ids)
}

fn lower_module_init(
    module: &mut dyn Module,
    module_stmts: &[Stmt],
    struct_defs: &HashMap<String, StructInfo>,
    class_defs: &HashMap<String, ClassInfo>,
    func_ids: &mut HashMap<String, FuncId>,
    closure_defs: &mut HashMap<String, ClosureInfo>,
) -> Result<FuncId, String> {
    let init_fn = pylang_front::ast::Fn {
        name: "_init_module".to_string(),
        params: vec![],
        ret: None,
        body: module_stmts.to_vec(),
        decorators: vec![],
        captures: vec![],
    };
    lower_fn(module, &init_fn, struct_defs, class_defs, func_ids, closure_defs)
}



pub fn lower_fn(
    module: &mut dyn Module,
    f: &AstFn,
    struct_defs: &HashMap<String, StructInfo>,
    class_defs: &HashMap<String, ClassInfo>,
    func_ids: &mut HashMap<String, FuncId>,
    closure_defs: &mut HashMap<String, ClosureInfo>,
) -> Result<FuncId, String> {
    // Pre-pass: find nested functions and hoist them to module level
    for stmt in &f.body {
        if let Stmt::Fn(nested) = stmt {
            if nested.captures.is_empty() {
                continue;
            }
            let mangled_name = format!("{}_{}", f.name, nested.name);
            // Hoisted function takes closure_ptr as first param, then actual params
            let mut hoisted_params = vec![Param {
                name: format!("__closure_{}", f.name),
                ty: AstType::I64,
                default: None,
            }];
            hoisted_params.extend(nested.params.clone());
            let hoisted_fn = AstFn {
                name: mangled_name.clone(),
                params: hoisted_params,
                ret: nested.ret.clone(),
                body: nested.body.clone(),
                decorators: vec![],
                captures: vec![],
            };
            let nested_id = lower_fn_closure(
                module, &hoisted_fn, &nested.captures,
                struct_defs, class_defs, func_ids, closure_defs,
            )?;
            func_ids.insert(mangled_name.clone(), nested_id);
            closure_defs.insert(nested.name.clone(), ClosureInfo {
                mangled_name,
                func_id: nested_id,
                capture_names: nested.captures.clone(),
            });
        }
    }

    let mut sig = module.make_signature();
    for param in &f.params {
        let ty = clif_type(&param.ty)?;
        sig.params.push(AbiParam::new(ty));
    }
    let has_return_val = f.body.iter().any(|s| {
        matches!(s, Stmt::Return(r) if r.val.is_some())
    });
    let ret_ty = if f.name == "main" {
        Some(types::I64)
    } else if let Some(ref t) = f.ret {
        Some(clif_type(t)?)
    } else if has_return_val {
        Some(types::I64)
    } else {
        None
    };
    if let Some(ty) = ret_ty {
        sig.returns.push(AbiParam::new(ty));
    }

    let linkage = if f.name.starts_with('_') { Linkage::Local } else { Linkage::Export };
    let func_id = module.declare_function(&f.name, linkage, &sig)
        .map_err(|e| e.to_string())?;

    let mut ctx = module.make_context();
    ctx.func.signature = sig;
    ctx.func.name = UserFuncName::user(0, func_id.as_u32());

    let mut fn_ctx = FunctionBuilderContext::new();
    let mut builder = FunctionBuilder::new(&mut ctx.func, &mut fn_ctx);

    let entry = builder.create_block();
    builder.switch_to_block(entry);

    builder.append_block_params_for_function_params(entry);

    let _dummy_slot = builder.create_sized_stack_slot(StackSlotData::new(StackSlotKind::ExplicitSlot, 8, 4));

    let mut locals = HashMap::new();

    for (i, param) in f.params.iter().enumerate() {
        let ty = clif_type(&param.ty)?;
        let var = builder.declare_var(ty);
        let val = builder.block_params(entry)[i];
        builder.def_var(var, val);
        locals.insert(param.name.clone(), var);
    }

    let mut lctx = LowerCtx {
        builder,
        module,
        locals,
        func_ids: func_ids.clone(),
        closure_defs: closure_defs.clone(),
        block_filled: false,
        loop_stack: Vec::new(),
        struct_defs: struct_defs.clone(),
        class_defs: class_defs.clone(),
    };

    for stmt in &f.body {
        if lctx.block_filled {
            break;
        }
        lower_stmt(stmt, &mut lctx)?;
    }

    if !lctx.block_filled {
        if let Some(ty) = ret_ty {
            let zero = lctx.builder.ins().iconst(ty, 0);
            lctx.builder.ins().return_(&[zero]);
        } else {
            lctx.builder.ins().return_(&[]);
        }
    }

    lctx.builder.seal_all_blocks();
    lctx.builder.finalize();

    module.define_function(func_id, &mut ctx)
        .map_err(|e| e.to_string())?;

    Ok(func_id)
}

fn lower_fn_closure(
    module: &mut dyn Module,
    f: &AstFn,
    capture_names: &[String],
    struct_defs: &HashMap<String, StructInfo>,
    class_defs: &HashMap<String, ClassInfo>,
    func_ids: &mut HashMap<String, FuncId>,
    closure_defs: &mut HashMap<String, ClosureInfo>,
) -> Result<FuncId, String> {
    // Hoisted function signature: (closure_ptr: i64, actual_params...) -> ret
    let mut sig = module.make_signature();
    sig.params.push(AbiParam::new(types::I64)); // closure_ptr as first param
    for param in &f.params {
        let ty = clif_type(&param.ty)?;
        sig.params.push(AbiParam::new(ty));
    }
    let has_return_val = f.body.iter().any(|s| {
        matches!(s, Stmt::Return(r) if r.val.is_some())
    });
    let ret_ty = if let Some(ref t) = f.ret {
        Some(clif_type(t)?)
    } else if has_return_val {
        Some(types::I64)
    } else {
        None
    };
    if let Some(ty) = ret_ty {
        sig.returns.push(AbiParam::new(ty));
    }

    let linkage = Linkage::Export;
    let func_id = module.declare_function(&f.name, linkage, &sig)
        .map_err(|e| e.to_string())?;

    let mut ctx = module.make_context();
    ctx.func.signature = sig;
    ctx.func.name = UserFuncName::user(0, func_id.as_u32());

    let mut fn_ctx = FunctionBuilderContext::new();
    let mut builder = FunctionBuilder::new(&mut ctx.func, &mut fn_ctx);

    let entry = builder.create_block();
    builder.switch_to_block(entry);

    builder.append_block_params_for_function_params(entry);

    let _dummy_slot = builder.create_sized_stack_slot(StackSlotData::new(StackSlotKind::ExplicitSlot, 8, 4));

    let mut locals = HashMap::new();

    // param 0 = closure_ptr
    let closure_ptr_ty = types::I64;
    let closure_var = builder.declare_var(closure_ptr_ty);
    let closure_ptr = builder.block_params(entry)[0];
    builder.def_var(closure_var, closure_ptr);

    // Load captured variables from closure struct (offset 8, 16, ...)
    for (i, cap_name) in capture_names.iter().enumerate() {
        let offset = (8 + i * 8) as i64;
        let offset_val = builder.ins().iconst(types::I64, offset);
        let addr = builder.ins().iadd(closure_ptr, offset_val);
        let cap_val = builder.ins().load(types::I64, MemFlags::trusted(), addr, 0);
        let var = builder.declare_var(types::I64);
        builder.def_var(var, cap_val);
        locals.insert(cap_name.clone(), var);
    }

    // Remaining params (after closure_ptr at index 0)
    let actual_params_start = 1; // skip closure_ptr
    for (i, param) in f.params.iter().enumerate() {
        let ty = clif_type(&param.ty)?;
        let var = builder.declare_var(ty);
        let val = builder.block_params(entry)[actual_params_start + i];
        builder.def_var(var, val);
        locals.insert(param.name.clone(), var);
    }

    let mut lctx = LowerCtx {
        builder,
        module,
        locals,
        func_ids: func_ids.clone(),
        closure_defs: closure_defs.clone(),
        block_filled: false,
        loop_stack: Vec::new(),
        struct_defs: struct_defs.clone(),
        class_defs: class_defs.clone(),
    };

    for stmt in &f.body {
        if lctx.block_filled {
            break;
        }
        lower_stmt(stmt, &mut lctx)?;
    }

    if !lctx.block_filled {
        if let Some(ty) = ret_ty {
            let zero = lctx.builder.ins().iconst(ty, 0);
            lctx.builder.ins().return_(&[zero]);
        } else {
            lctx.builder.ins().return_(&[]);
        }
    }

    lctx.builder.seal_all_blocks();
    lctx.builder.finalize();

    module.define_function(func_id, &mut ctx)
        .map_err(|e| e.to_string())?;

    Ok(func_id)
}

fn extract_int_from_expr(expr: &Expr) -> i64 {
    match expr {
        Expr::Int(n) => *n,
        Expr::Bool(b) => *b as i64,
        _ => 0,
    }
}

fn clif_type(ty: &AstType) -> Result<Type, String> {
    match ty {
        AstType::I64 => Ok(types::I64),
        AstType::F64 => Ok(types::F64),
        AstType::Bool => Ok(types::I8),
        AstType::Char => Ok(types::I64),
        AstType::Unit => Ok(types::I64),
        AstType::Named(n) if n == "int" => Ok(types::I64),
        AstType::Named(n) if n == "float" => Ok(types::F64),
        AstType::Named(n) if n == "bool" => Ok(types::I8),
        AstType::Named(n) if n == "str" => Ok(types::I64),
        AstType::Named(_) => Ok(types::I64),
        _ => Err(format!("unsupported type: {:?}", ty)),
    }
}

fn ast_type_to_clif(ty: &AstType) -> Result<Type, String> {
    clif_type(ty)
}

fn lower_stmt(stmt: &Stmt, lctx: &mut LowerCtx) -> Result<(), String> {
    if lctx.block_filled {
        return Ok(());
    }
    match stmt {
        Stmt::Let(l) => {
            let val = lower_expr(&l.val, lctx)?;
            let var = lctx.builder.declare_var(lctx.builder.func.dfg.value_type(val));
            lctx.builder.def_var(var, val);
            lctx.locals.insert(l.name.clone(), var);
            Ok(())
        }
        Stmt::LetMut(l) => {
            let val = lower_expr(&l.val, lctx)?;
            let var = lctx.builder.declare_var(lctx.builder.func.dfg.value_type(val));
            lctx.builder.def_var(var, val);
            lctx.locals.insert(l.name.clone(), var);
            Ok(())
        }
        Stmt::Assign(a) => {
            if let Expr::Ident(name) = &*a.target {
                let val = lower_expr(&a.val, lctx)?;
                if let Some(&var) = lctx.locals.get(name) {
                    lctx.builder.def_var(var, val);
                } else {
                    let var = lctx.builder.declare_var(lctx.builder.func.dfg.value_type(val));
                    lctx.builder.def_var(var, val);
                    lctx.locals.insert(name.clone(), var);
                }
                Ok(())
            } else if let Expr::Dot { obj, name } = &*a.target {
                let obj_val = lower_expr(obj, lctx)?;
                let val = lower_expr(&a.val, lctx)?;
                let offset = get_field_offset(lctx, name);
                let offset_val = lctx.builder.ins().iconst(types::I64, offset);
                let addr = lctx.builder.ins().iadd(obj_val, offset_val);
                lctx.builder.ins().store(MemFlags::trusted(), val, addr, 0);
                Ok(())
            } else if let Expr::Subscript(elems) = &*a.target {
                if elems.len() == 2 {
                    let obj = lower_expr(&elems[0], lctx)?;
                    let index = lower_expr(&elems[1], lctx)?;
                    let val = lower_expr(&a.val, lctx)?;
                    let eight = lctx.builder.ins().iconst(types::I32, 8);
                    let index_i32 = lctx.builder.ins().ireduce(types::I32, index);
                    let index_times_8 = lctx.builder.ins().imul(eight, index_i32);
                    let offset = lctx.builder.ins().iadd(eight, index_times_8);
                    let offset_i64 = lctx.builder.ins().uextend(types::I64, offset);
                    let addr = lctx.builder.ins().iadd(obj, offset_i64);
                    lctx.builder.ins().store(MemFlags::trusted(), val, addr, 0);
                    Ok(())
                } else {
                    Err("subscript assign requires 2 elements".to_string())
                }
            } else {
                Err("complex assignment not yet supported".to_string())
            }
        }
        Stmt::AssignOp(a) => {
            if let Expr::Ident(name) = &a.target {
                let lhs = if let Some(&var) = lctx.locals.get(name) {
                    lctx.builder.use_var(var)
                } else {
                    return Err(format!("undefined variable: {}", name));
                };
                let rhs = lower_expr(&a.val, lctx)?;
                let result = lower_binop(lctx, &a.op, lhs, rhs)?;
                if let Some(&var) = lctx.locals.get(name) {
                    lctx.builder.def_var(var, result);
                }
                Ok(())
            } else {
                Err("complex assign-op not yet supported".to_string())
            }
        }
        Stmt::Return(r) => {
            let vals: Vec<Value> = r.val.as_ref()
                .map(|e| lower_expr(e, lctx))
                .transpose()?
                .into_iter()
                .collect();
            lctx.builder.ins().return_(&vals);
            lctx.block_filled = true;
            Ok(())
        }
        Stmt::Expr(e) => {
            let _ = lower_expr(e, lctx)?;
            Ok(())
        }
        Stmt::If(i) => lower_if(i, lctx),
        Stmt::While(w) => lower_while(w, lctx),
        Stmt::For(f) => lower_for(f, lctx),
        Stmt::Loop(lo) => lower_loop(lo, lctx),
        Stmt::Match(m) => lower_match(m, lctx),
        Stmt::Try(t) => lower_try(t, lctx),
        Stmt::With(w) => lower_with(w, lctx),
        Stmt::Raise(r) => lower_raise(r, lctx),
        Stmt::Assert(a) => lower_assert(a, lctx),
        Stmt::Yield(y) => lower_yield(y, lctx),
        Stmt::Break => {
            lctx.break_loop();
            Ok(())
        }
        Stmt::Continue => {
            lctx.continue_loop();
            Ok(())
        }
        Stmt::Pass => Ok(()),
        Stmt::Fn(_f) => {
            // Nested functions are hoisted to module level in lower_fn
            // For non-capturing nested fns, store a func_addr local
            // For capturing fns, closure structs are created in Expr::Ident
            Ok(())
        }
        _ => Err(format!("unsupported statement: {:?}", stmt)),
    }
}

fn lower_expr(expr: &Expr, lctx: &mut LowerCtx) -> Result<Value, String> {
    match expr {
        Expr::Int(n) => Ok(lctx.builder.ins().iconst(types::I64, *n)),
        Expr::Bool(b) => Ok(lctx.builder.ins().iconst(types::I8, if *b { 1 } else { 0 })),
        Expr::Float(f) => Ok(lctx.builder.ins().f64const(*f)),
        Expr::Str(s) => {
            let bytes = s.as_bytes();
            alloc_string_literal(lctx, bytes)
        }
        Expr::Char(c) => Ok(lctx.builder.ins().iconst(types::I64, *c as i64)),
        Expr::None => Ok(lctx.builder.ins().iconst(types::I64, 0)),
        Expr::Ident(name) => {
            // Check if ident refers to a nested function (closure)
            if let Some(closure_info) = lctx.closure_defs.get(name) {
                let ci = closure_info.clone();
                let num_captures = ci.capture_names.len();
                let struct_size = 8 + (num_captures * 8);
                let size_val = lctx.builder.ins().iconst(types::I64, struct_size as i64);
                let closure_ptr = call_runtime(lctx, "alloc", &[size_val], types::I64)?;
                
                // Store fn_ptr at offset 0
                let callee_ref = lctx.module.declare_func_in_func(ci.func_id, lctx.builder.func);
                let fn_addr = lctx.builder.ins().func_addr(types::I64, callee_ref);
                lctx.builder.ins().store(MemFlags::trusted(), fn_addr, closure_ptr, 0);
                
                // Store captured values at offsets 8, 16, ...
                for (i, cap_name) in ci.capture_names.iter().enumerate() {
                    let cap_val = if let Some(&var) = lctx.locals.get(cap_name) {
                        lctx.builder.use_var(var)
                    } else {
                        return Err(format!("undefined captured variable: {}", cap_name));
                    };
                    let offset = lctx.builder.ins().iconst(types::I64, (8 + i * 8) as i64);
                    let addr = lctx.builder.ins().iadd(closure_ptr, offset);
                    lctx.builder.ins().store(MemFlags::trusted(), cap_val, addr, 0);
                }
                
                Ok(closure_ptr)
            } else if let Some(&var) = lctx.locals.get(name) {
                // Check locals BEFORE func_ids (reassigned functions from decorators)
                Ok(lctx.builder.use_var(var))
            } else if let Some(&func_id) = lctx.func_ids.get(name) {
                // Module-level function reference - return function address
                let callee_ref = lctx.module.declare_func_in_func(func_id, lctx.builder.func);
                Ok(lctx.builder.ins().func_addr(types::I64, callee_ref))
            } else {
                Err(format!("undefined variable: {}", name))
            }
        }
        Expr::BinOp { op, lhs, rhs } => {
            let l = lower_expr(lhs, lctx)?;
            let r = lower_expr(rhs, lctx)?;
            lower_binop(lctx, op, l, r)
        }
        Expr::UnOp { op, val } => {
            let v = lower_expr(val, lctx)?;
            lower_unop(lctx, op, v)
        }
        Expr::Cmp { op, lhs, rhs } => {
            let l = lower_expr(lhs, lctx)?;
            let r = lower_expr(rhs, lctx)?;
            lower_cmpop(lctx, op, l, r)
        }
        Expr::Call { func, args } => lower_call(func, args, lctx),
        Expr::Method { obj, name, args } => lower_method(obj, name, args, lctx),
        Expr::Dot { obj, name } => {
            let obj_val = lower_expr(obj, lctx)?;
            let offset = get_field_offset(lctx, name);
            let offset_val = lctx.builder.ins().iconst(types::I64, offset);
            let addr = lctx.builder.ins().iadd(obj_val, offset_val);
            Ok(lctx.builder.ins().load(types::I64, MemFlags::trusted(), addr, 0))
        }
        Expr::Index { obj, idx } => {
            let obj_val = lower_expr(obj, lctx)?;
            let idx_val = lower_expr(idx, lctx)?;
            let scale = lctx.builder.ins().iconst(types::I64, 8);
            let offset = lctx.builder.ins().imul(idx_val, scale);
            let addr = lctx.builder.ins().iadd(obj_val, offset);
            Ok(lctx.builder.ins().load(types::I64, MemFlags::trusted(), addr, 0))
        }
        Expr::Slice { .. } => Err("slice lowering not yet supported".to_string()),
        Expr::Tuple(elems) | Expr::List(elems) | Expr::Set(elems) => {
            let vals: Vec<Value> = elems.iter()
                .map(|e| lower_expr(e, lctx))
                .collect::<Result<Vec<_>, _>>()?;
            let data_size = elems.len() * 8;
            let total_size = data_size + 8; // +8 for len
            let size_val = lctx.builder.ins().iconst(types::I64, total_size as i64);
            let ptr = call_runtime(lctx, "alloc", &[size_val], types::I64)?;
            // store len at offset 0
            let len_val = lctx.builder.ins().iconst(types::I64, elems.len() as i64);
            lctx.builder.ins().store(MemFlags::trusted(), len_val, ptr, 0);
            // store elements at offset 8, 16, ...
            for (i, val) in vals.iter().enumerate() {
                let offset = lctx.builder.ins().iconst(types::I64, ((i + 1) * 8) as i64);
                let addr = lctx.builder.ins().iadd(ptr, offset);
                lctx.builder.ins().store(MemFlags::trusted(), *val, addr, 0);
            }
            Ok(ptr)
        }
        Expr::Dict(items) => {
            let size = items.len() * 16;
            let size_val = lctx.builder.ins().iconst(types::I64, size as i64);
            let ptr = call_runtime(lctx, "alloc", &[size_val], types::I64)?;
            for (i, (k, v)) in items.iter().enumerate() {
                let k_val = lower_expr(k, lctx)?;
                let v_val = lower_expr(v, lctx)?;
                let k_offset = lctx.builder.ins().iconst(types::I64, (i * 16) as i64);
                let v_offset = lctx.builder.ins().iconst(types::I64, (i * 16 + 8) as i64);
                let k_addr = lctx.builder.ins().iadd(ptr, k_offset);
                let v_addr = lctx.builder.ins().iadd(ptr, v_offset);
                lctx.builder.ins().store(MemFlags::trusted(), k_val, k_addr, 0);
                lctx.builder.ins().store(MemFlags::trusted(), v_val, v_addr, 0);
            }
            Ok(ptr)
        }
        Expr::ListComp { .. } | Expr::DictComp { .. } => {
            Err("comprehensions not yet supported in CLIF lowering".to_string())
        }
        Expr::Await(inner) => lower_expr(inner, lctx),
        Expr::Async { .. } => Err("async not yet supported".to_string()),
        Expr::YieldFrom(inner) => lower_expr(inner, lctx),
        Expr::Lambda { .. } => Err("lambda not yet supported".to_string()),
        Expr::If { cond, then, else_ } => {
            let cond_val = lower_expr(cond, lctx)?;
            let then_block = lctx.create_block();
            let else_block = lctx.create_block();
            let merge_block = lctx.create_block();

            let _ty = lctx.builder.func.dfg.value_type(cond_val);
            // To get result type, lower both branches first? We need a placeholder type.
            // Use I64 as default.
            lctx.builder.append_block_param(merge_block, types::I64);

            lctx.builder.ins().brif(cond_val, then_block, &[], else_block, &[]);
            lctx.block_filled = true;

            lctx.switch_to_block(then_block);
            let then_val = lower_expr(then, lctx)?;
            if !lctx.block_filled {
                lctx.builder.ins().jump(merge_block, &[BlockArg::Value(then_val)]);
                lctx.block_filled = true;
            }

            lctx.switch_to_block(else_block);
            let else_val = lower_expr(else_, lctx)?;
            if !lctx.block_filled {
                lctx.builder.ins().jump(merge_block, &[BlockArg::Value(else_val)]);
                lctx.block_filled = true;
            }

            lctx.switch_to_block(merge_block);
            let phi = lctx.builder.block_params(merge_block)[0];
            Ok(phi)
        }
        Expr::Match { .. } => Err("match expression not yet supported".to_string()),
        Expr::Subscript(elems) => {
            if elems.len() == 2 {
                let obj = lower_expr(&elems[0], lctx)?;
                let index = lower_expr(&elems[1], lctx)?;
                // assume list: len at 0, data at 8 + index*8
                let eight = lctx.builder.ins().iconst(types::I32, 8);
                let index_i32 = lctx.builder.ins().ireduce(types::I32, index);
                let index_times_8 = lctx.builder.ins().imul(eight, index_i32);
                let offset = lctx.builder.ins().iadd(eight, index_times_8);
                let offset_i64 = lctx.builder.ins().uextend(types::I64, offset);
                let addr = lctx.builder.ins().iadd(obj, offset_i64);
                Ok(lctx.builder.ins().load(types::I64, MemFlags::trusted(), addr, 0))
            } else {
                Err("subscript requires 2 elements".to_string())
            }
        }
        Expr::Bytes(_) => Err("bytes not yet supported".to_string()),
    }
}

fn lower_binop(lctx: &mut LowerCtx, op: &BinOp, l: Value, r: Value) -> Result<Value, String> {
    match op {
        BinOp::Add => Ok(lctx.builder.ins().iadd(l, r)),
        BinOp::Sub => Ok(lctx.builder.ins().isub(l, r)),
        BinOp::Mul => Ok(lctx.builder.ins().imul(l, r)),
        BinOp::Div => Ok(lctx.builder.ins().sdiv(l, r)),
        BinOp::Rem => Ok(lctx.builder.ins().srem(l, r)),
        BinOp::FloorDiv => Ok(lctx.builder.ins().sdiv(l, r)),
        BinOp::Pow => call_runtime(lctx, "pow", &[l, r], types::I64),
        BinOp::BitAnd => Ok(lctx.builder.ins().band(l, r)),
        BinOp::BitOr => Ok(lctx.builder.ins().bor(l, r)),
        BinOp::BitXor => Ok(lctx.builder.ins().bxor(l, r)),
        BinOp::LShift => Ok(lctx.builder.ins().ishl(l, r)),
        BinOp::RShift => Ok(lctx.builder.ins().sshr(l, r)),
    }
}

fn lower_unop(lctx: &mut LowerCtx, op: &UnOp, v: Value) -> Result<Value, String> {
    match op {
        UnOp::Neg => Ok(lctx.builder.ins().ineg(v)),
        UnOp::Not => {
            let ty = lctx.builder.func.dfg.value_type(v);
            let zero = lctx.builder.ins().iconst(ty, 0);
            Ok(lctx.builder.ins().icmp(IntCC::Equal, v, zero))
        }
        UnOp::Pos => Ok(v),
        UnOp::BitNot => Ok(lctx.builder.ins().bnot(v)),
    }
}

fn lower_cmpop(lctx: &mut LowerCtx, op: &CmpOp, l: Value, r: Value) -> Result<Value, String> {
    let cond = match op {
        CmpOp::Eq => IntCC::Equal,
        CmpOp::Ne => IntCC::NotEqual,
        CmpOp::Lt => IntCC::SignedLessThan,
        CmpOp::Le => IntCC::SignedLessThanOrEqual,
        CmpOp::Gt => IntCC::SignedGreaterThan,
        CmpOp::Ge => IntCC::SignedGreaterThanOrEqual,
        CmpOp::Is | CmpOp::IsNot | CmpOp::In | CmpOp::NotIn => {
            return Err(format!("unsupported cmp op: {:?}", op));
        }
    };
    Ok(lctx.builder.ins().icmp(cond, l, r))
}

fn lower_call(func: &Expr, args: &[Expr], lctx: &mut LowerCtx) -> Result<Value, String> {
    // Lower args first (needed for both direct and indirect calls)
    let arg_vals: Vec<Value> = args.iter()
        .map(|a| lower_expr(a, lctx))
        .collect::<Result<Vec<_>, _>>()?;

    if let Expr::Ident(name) = func {
        match name.as_str() {
            "print" => {
                if let Some(&val) = arg_vals.first() {
                    if let Some(Expr::Str(s)) = args.first() {
                        let len = lctx.builder.ins().iconst(types::I64, s.len() as i64);
                        call_runtime(lctx, "print_str", &[val, len], types::I64)?;
                    } else {
                        call_runtime(lctx, "print_int", &[val], types::I64)?;
                    }
                }
                Ok(lctx.builder.ins().iconst(types::I64, 0))
            }
            "len" => {
                if let Some(&val) = arg_vals.first() {
                    Ok(lctx.builder.ins().load(types::I64, MemFlags::trusted(), val, 0))
                } else {
                    Err("len() requires argument".to_string())
                }
            }
            "range" => Err("range() not yet supported in lowering".to_string()),
            "int" => {
                if let Some(&val) = arg_vals.first() {
                    let ty = lctx.builder.func.dfg.value_type(val);
                    if ty == types::F64 {
                        Ok(lctx.builder.ins().fcvt_to_sint(types::I64, val))
                    } else {
                        Ok(val)
                    }
                } else {
                    Err("int() requires argument".to_string())
                }
            }
            "str" => Err("str() not yet supported in lowering".to_string()),
            "bool" => {
                if let Some(&val) = arg_vals.first() {
                    let ty = lctx.builder.func.dfg.value_type(val);
                    let zero = lctx.builder.ins().iconst(ty, 0);
                    Ok(lctx.builder.ins().icmp(IntCC::NotEqual, val, zero))
                } else {
                    Err("bool() requires argument".to_string())
                }
            }
            "float" => {
                if let Some(&val) = arg_vals.first() {
                    let ty = lctx.builder.func.dfg.value_type(val);
                    if ty == types::I64 {
                        Ok(lctx.builder.ins().fcvt_from_sint(types::F64, val))
                    } else {
                        Ok(val)
                    }
                } else {
                    Err("float() requires argument".to_string())
                }
            }
            "input" => Err("input() not yet supported in lowering".to_string()),
            _ => {
                if let Some(struct_info) = lctx.struct_defs.get(name).cloned() {
                    let size = lctx.builder.ins().iconst(types::I64, (struct_info.fields.len() * 8) as i64);
                    let ptr = call_runtime(lctx, "alloc", &[size], types::I64)?;
                    for (i, field) in struct_info.fields.iter().enumerate() {
                        if i < arg_vals.len() {
                            let offset_val = lctx.builder.ins().iconst(types::I64, field.offset);
                            let addr = lctx.builder.ins().iadd(ptr, offset_val);
                            lctx.builder.ins().store(MemFlags::trusted(), arg_vals[i], addr, 0);
                        }
                    }
                    Ok(ptr)
                } else if let Some(class_info) = lctx.class_defs.get(name).cloned() {
                    let size = lctx.builder.ins().iconst(types::I64, (class_info._fields.len() * 8) as i64);
                    let ptr = call_runtime(lctx, "alloc", &[size], types::I64)?;
                    for (i, field) in class_info._fields.iter().enumerate() {
                        let default_val = if i < class_info.field_defaults.len() {
                            class_info.field_defaults[i]
                        } else {
                            0
                        };
                        let offset_val = lctx.builder.ins().iconst(types::I64, field.offset);
                        let addr = lctx.builder.ins().iadd(ptr, offset_val);
                        let val = lctx.builder.ins().iconst(types::I64, default_val);
                        lctx.builder.ins().store(MemFlags::trusted(), val, addr, 0);
                    }
                    
                    if let Some(init_name) = class_info._methods.get("__init__") {
                        let mut init_sig = lctx.module.make_signature();
                        init_sig.params.push(AbiParam::new(types::I64));
                        for _arg in &arg_vals {
                            init_sig.params.push(AbiParam::new(types::I64));
                        }
                        let mut init_args = vec![ptr];
                        init_args.extend(&arg_vals);
                        let init_id = match lctx.module.declare_function(init_name, Linkage::Import, &init_sig) {
                            Ok(id) => id,
                            Err(_) => {
                                let mut sig2 = lctx.module.make_signature();
                                sig2.params.push(AbiParam::new(types::I64));
                                for _arg in &arg_vals {
                                    sig2.params.push(AbiParam::new(types::I64));
                                }
                                sig2.returns.push(AbiParam::new(types::I64));
                                lctx.module.declare_function(init_name, Linkage::Import, &sig2)
                                    .map_err(|e| format!("init: {}", e))?
                            }
                        };
                        let init_ref = lctx.module.declare_func_in_func(init_id, lctx.builder.func);
                        lctx.builder.ins().call(init_ref, &init_args);
                    }
                    
                    Ok(ptr)
                } else if let Some(&callee_id) = lctx.func_ids.get(name) {
                    let callee_ref = lctx.module.declare_func_in_func(callee_id, lctx.builder.func);
                    let call = lctx.builder.ins().call(callee_ref, &arg_vals);
                    let result = lctx.builder.inst_results(call);
                    if result.is_empty() {
                        Ok(lctx.builder.ins().iconst(types::I64, 0))
                    } else {
                        Ok(result[0])
                    }
                } else {
                    let mut sig = lctx.module.make_signature();
                    for arg in &arg_vals {
                        sig.params.push(AbiParam::new(lctx.builder.func.dfg.value_type(*arg)));
                    }
                    sig.returns.push(AbiParam::new(types::I64));

                    let callee_id = lctx.module.declare_function(name, Linkage::Import, &sig)
                        .map_err(|e| e.to_string())?;
                    let callee_ref = lctx.module.declare_func_in_func(callee_id, lctx.builder.func);
                    let call = lctx.builder.ins().call(callee_ref, &arg_vals);
                    let result = lctx.builder.inst_results(call);
                    if result.is_empty() {
                        Ok(lctx.builder.ins().iconst(types::I64, 0))
                    } else {
                        Ok(result[0])
                    }
                }
            }
        }
    } else {
        // Chained / indirect call: lower func expression to get closure pointer
        let closure_ptr = lower_expr(func, lctx)?;
        // Load fn_ptr from offset 0 of closure struct
        let fn_ptr = lctx.builder.ins().load(types::I64, MemFlags::trusted(), closure_ptr, 0);
        
        // Build signature: (closure_ptr: i64, args...) -> i64
        let mut sig = lctx.module.make_signature();
        sig.params.push(AbiParam::new(types::I64)); // closure_ptr
        for arg in &arg_vals {
            sig.params.push(AbiParam::new(lctx.builder.func.dfg.value_type(*arg)));
        }
        sig.returns.push(AbiParam::new(types::I64));
        
        let sig_ref = lctx.builder.import_signature(sig);
        let mut all_args = vec![closure_ptr];
        all_args.extend(&arg_vals);
        let call = lctx.builder.ins().call_indirect(sig_ref, fn_ptr, &all_args);
        let results = lctx.builder.inst_results(call);
        if results.is_empty() {
            Ok(lctx.builder.ins().iconst(types::I64, 0))
        } else {
            Ok(results[0])
        }
    }
}

fn lower_method(obj: &Expr, method_name: &str, args: &[Expr], lctx: &mut LowerCtx) -> Result<Value, String> {
    let obj_val = lower_expr(obj, lctx)?;
    
    let class_type = match obj {
        Expr::Ident(name) => name.clone(),
        _ => String::new(),
    };
    
    let full_method_name = if let Some(class_info) = lctx.class_defs.get(&class_type) {
        class_info._methods.get(method_name).cloned().unwrap_or_else(|| method_name.to_string())
    } else {
        let mut found = None;
        for class_info in lctx.class_defs.values() {
            if let Some(m) = class_info._methods.get(method_name) {
                found = Some(m.clone());
                break;
            }
        }
        found.unwrap_or_else(|| method_name.to_string())
    };
    
    let mut arg_vals = vec![obj_val];
    for arg in args {
        arg_vals.push(lower_expr(arg, lctx)?);
    }

    let mut sig = lctx.module.make_signature();
    for arg in &arg_vals {
        sig.params.push(AbiParam::new(lctx.builder.func.dfg.value_type(*arg)));
    }
    sig.returns.push(AbiParam::new(types::I64));

    let callee_id = match lctx.module.declare_function(&full_method_name, Linkage::Import, &sig) {
        Ok(id) => id,
        Err(_) => {
            // Function declared without return value, try without
            let mut sig2 = lctx.module.make_signature();
            for arg in &arg_vals {
                sig2.params.push(AbiParam::new(lctx.builder.func.dfg.value_type(*arg)));
            }
            let callee_id2 = lctx.module.declare_function(&full_method_name, Linkage::Import, &sig2)
                .map_err(|e| format!("{}: {}", full_method_name, e))?;
            let callee_ref2 = lctx.module.declare_func_in_func(callee_id2, lctx.builder.func);
            lctx.builder.ins().call(callee_ref2, &arg_vals);
            return Ok(lctx.builder.ins().iconst(types::I64, 0));
        }
    };
    
    let callee_ref = lctx.module.declare_func_in_func(callee_id, lctx.builder.func);
    let call = lctx.builder.ins().call(callee_ref, &arg_vals);
    let results = lctx.builder.inst_results(call);
    if results.is_empty() {
        Ok(lctx.builder.ins().iconst(types::I64, 0))
    } else {
        Ok(results[0])
    }
}

fn call_runtime(lctx: &mut LowerCtx, name: &str, args: &[Value], ret_ty: Type) -> Result<Value, String> {
    let mut sig = lctx.module.make_signature();
    for arg in args {
        sig.params.push(AbiParam::new(lctx.builder.func.dfg.value_type(*arg)));
    }
    sig.returns.push(AbiParam::new(ret_ty));

    let callee_id = lctx.module.declare_function(name, Linkage::Import, &sig)
        .map_err(|e| e.to_string())?;
    let callee_ref = lctx.module.declare_func_in_func(callee_id, lctx.builder.func);
    let call = lctx.builder.ins().call(callee_ref, args);
    let results = lctx.builder.inst_results(call);
    if results.is_empty() {
        Ok(lctx.builder.ins().iconst(types::I64, 0))
    } else {
        Ok(results[0])
    }
}

fn alloc_string_literal(lctx: &mut LowerCtx, bytes: &[u8]) -> Result<Value, String> {
    let size = lctx.builder.ins().iconst(types::I64, bytes.len() as i64);
    let ptr = call_runtime(lctx, "alloc", &[size], types::I64)?;
    for (i, &b) in bytes.iter().enumerate() {
        let offset = lctx.builder.ins().iconst(types::I64, i as i64);
        let addr = lctx.builder.ins().iadd(ptr, offset);
        let byte_val = lctx.builder.ins().iconst(types::I8, b as i64);
        lctx.builder.ins().store(MemFlags::trusted(), byte_val, addr, 0);
    }
    Ok(ptr)
}

fn get_field_offset(lctx: &LowerCtx, field_name: &str) -> i64 {
    for struct_info in lctx.struct_defs.values() {
        for field in &struct_info.fields {
            if field._name == field_name {
                return field.offset;
            }
        }
    }
    for class_info in lctx.class_defs.values() {
        for field in &class_info._fields {
            if field._name == field_name {
                return field.offset;
            }
        }
    }
    match field_name {
        "_data" | "_len" | "first" => 0,
        "_cap" | "second" => 8,
        _ => 0,
    }
}

// ---- Control Flow ----

fn lower_if(i: &If, lctx: &mut LowerCtx) -> Result<(), String> {
    let cond = lower_expr(&i.cond, lctx)?;
    let then_block = lctx.create_block();
    let else_block = lctx.create_block();
    let merge_block = lctx.create_block();

    lctx.builder.ins().brif(cond, then_block, &[], else_block, &[]);
    lctx.block_filled = true;

    lctx.switch_to_block(then_block);
    for stmt in &i.then {
        if lctx.block_filled { break; }
        lower_stmt(stmt, lctx)?;
    }
    if !lctx.block_filled {
        lctx.builder.ins().jump(merge_block, &[]);
    }
    lctx.builder.seal_block(then_block);

    lctx.switch_to_block(else_block);
    if let Some(ref else_stmts) = i.else_ {
        for stmt in else_stmts {
            if lctx.block_filled { break; }
            lower_stmt(stmt, lctx)?;
        }
    }
    if !lctx.block_filled {
        lctx.builder.ins().jump(merge_block, &[]);
    }
    lctx.builder.seal_block(else_block);

    lctx.builder.seal_block(merge_block);
    lctx.switch_to_block(merge_block);

    Ok(())
}

fn lower_while(w: &While, lctx: &mut LowerCtx) -> Result<(), String> {
    let header_block = lctx.create_block();
    let body_block = lctx.create_block();
    let exit_block = lctx.create_block();

    lctx.push_loop(exit_block, header_block);

    // Jump from current block into loop header
    lctx.builder.ins().jump(header_block, &[]);
    lctx.block_filled = true;

    // Header block: evaluate condition
    lctx.switch_to_block(header_block);
    let cond = lower_expr(&w.cond, lctx)?;
    lctx.builder.ins().brif(cond, body_block, &[], exit_block, &[]);
    lctx.block_filled = true;

    // Body block: execute loop body
    lctx.switch_to_block(body_block);
    for stmt in &w.body {
        if lctx.block_filled { break; }
        lower_stmt(stmt, lctx)?;
    }
    if !lctx.block_filled {
        // Normal loop completion: jump back to header
        lctx.builder.ins().jump(header_block, &[]);
        lctx.block_filled = true;
    }

    lctx.builder.seal_block(header_block);
    lctx.builder.seal_block(body_block);
    // Do NOT seal exit_block here — subsequent statements may use
    // variables modified in the loop, and Cranelift needs to see
    // all uses before sealing to build correct SSA block parameters.
    // seal_all_blocks() in lower_fn will seal it at the end.
    lctx.pop_loop();

    // Continue with subsequent statements in exit block
    lctx.switch_to_block(exit_block);

    Ok(())
}

fn lower_for(f: &For, lctx: &mut LowerCtx) -> Result<(), String> {
    if let Expr::Call { func, args } = &f.iter {
        if let Expr::Ident(name) = func.as_ref() {
            if name == "range" && args.len() == 1 {
                let end_val = lower_expr(&args[0], lctx)?;

                let header_block = lctx.create_block();
                let body_block = lctx.create_block();
                let exit_block = lctx.create_block();

                let var = lctx.builder.declare_var(types::I64);
                let zero = lctx.builder.ins().iconst(types::I64, 0);
                lctx.builder.def_var(var, zero);
                lctx.locals.insert(f.target.clone(), var);

                lctx.push_loop(exit_block, header_block);

                lctx.builder.ins().jump(header_block, &[]);
                lctx.block_filled = true;

                lctx.switch_to_block(header_block);
                let i = lctx.builder.use_var(var);
                let cond = lctx.builder.ins().icmp(IntCC::SignedLessThan, i, end_val);
                lctx.builder.ins().brif(cond, body_block, &[], exit_block, &[]);
                lctx.block_filled = true;

                lctx.switch_to_block(body_block);
                for stmt in &f.body {
                    if lctx.block_filled { break; }
                    lower_stmt(stmt, lctx)?;
                }
                if !lctx.block_filled {
                    let i = lctx.builder.use_var(var);
                    let one = lctx.builder.ins().iconst(types::I64, 1);
                    let next = lctx.builder.ins().iadd(i, one);
                    lctx.builder.def_var(var, next);
                    lctx.builder.ins().jump(header_block, &[]);
                    lctx.block_filled = true;
                }

                lctx.builder.seal_block(header_block);
                lctx.builder.seal_block(body_block);
                // Do NOT seal exit_block here — subsequent statements may use
                // variables modified in the loop.
                lctx.pop_loop();

                lctx.switch_to_block(exit_block);

                return Ok(());
            }
        }
    }

    // treat as iterable (list for now)
    let _list_val = lower_expr(&f.iter, lctx)?;
    let len_expr = Expr::Call {
        func: Box::new(Expr::Ident("len".to_string())),
        args: vec![f.iter.clone()],
    };
    let end_val = lower_expr(&len_expr, lctx)?;

    let header_block = lctx.create_block();
    let body_block = lctx.create_block();
    let exit_block = lctx.create_block();

    let var = lctx.builder.declare_var(types::I64);
    let zero = lctx.builder.ins().iconst(types::I64, 0);
    lctx.builder.def_var(var, zero);
    lctx.locals.insert("i".to_string(), var);  // for subscript

    let target_var = lctx.builder.declare_var(types::I64);
    lctx.locals.insert(f.target.clone(), target_var);

    lctx.push_loop(exit_block, header_block);

    lctx.builder.ins().jump(header_block, &[]);
    lctx.block_filled = true;

    lctx.switch_to_block(header_block);
    let i = lctx.builder.use_var(var);
    let cond = lctx.builder.ins().icmp(IntCC::SignedLessThan, i, end_val);
    lctx.builder.ins().brif(cond, body_block, &[], exit_block, &[]);
    lctx.block_filled = true;

    lctx.switch_to_block(body_block);
    // assign target = list[i]
    let subscript_expr = Expr::Subscript(vec![f.iter.clone(), Expr::Ident("i".to_string())]);
    let item_val = lower_expr(&subscript_expr, lctx)?;
    lctx.builder.def_var(target_var, item_val);

    // then lower body
    for stmt in &f.body {
        if lctx.block_filled { break; }
        lower_stmt(stmt, lctx)?;
    }
    if !lctx.block_filled {
        let i = lctx.builder.use_var(var);
        let one = lctx.builder.ins().iconst(types::I64, 1);
        let next = lctx.builder.ins().iadd(i, one);
        lctx.builder.def_var(var, next);
        lctx.builder.ins().jump(header_block, &[]);
        lctx.block_filled = true;
    }

    lctx.builder.seal_block(header_block);
    lctx.builder.seal_block(body_block);
    // Do NOT seal exit_block here
    lctx.pop_loop();

    lctx.switch_to_block(exit_block);

    Ok(())
}

fn lower_loop(l: &Loop, lctx: &mut LowerCtx) -> Result<(), String> {
    let body_block = lctx.create_block();
    let exit_block = lctx.create_block();

    lctx.push_loop(exit_block, body_block);

    lctx.builder.ins().jump(body_block, &[]);
    lctx.block_filled = true;

    lctx.switch_to_block(body_block);
    for stmt in &l.body {
        if lctx.block_filled { break; }
        lower_stmt(stmt, lctx)?;
    }
    if !lctx.block_filled {
        lctx.builder.ins().jump(body_block, &[]);
        lctx.block_filled = true;
    }

    lctx.builder.seal_block(body_block);
    // Do NOT seal exit_block here
    lctx.pop_loop();

    lctx.switch_to_block(exit_block);

    Ok(())
}

fn lower_match(_m: &Match, _lctx: &mut LowerCtx) -> Result<(), String> {
    Err("match lowering not yet supported".to_string())
}

fn lower_try(_t: &Try, _lctx: &mut LowerCtx) -> Result<(), String> {
    Err("try/except lowering not yet supported".to_string())
}

fn lower_with(_w: &With, _lctx: &mut LowerCtx) -> Result<(), String> {
    Err("with lowering not yet supported".to_string())
}

fn lower_raise(_r: &Raise, _lctx: &mut LowerCtx) -> Result<(), String> {
    Err("raise lowering not yet supported".to_string())
}

fn lower_assert(a: &Assert, lctx: &mut LowerCtx) -> Result<(), String> {
    let cond = lower_expr(&a.cond, lctx)?;
    let then_block = lctx.create_block();
    let else_block = lctx.create_block();
    lctx.builder.ins().brif(cond, then_block, &[], else_block, &[]);
    lctx.block_filled = true;

    lctx.switch_to_block(else_block);
    let one = lctx.builder.ins().iconst(types::I32, 1);
    call_runtime(lctx, "exit", &[one], types::I64)?;
    if !lctx.block_filled {
        lctx.builder.ins().return_(&[]);
        lctx.block_filled = true;
    }

    lctx.switch_to_block(then_block);

    Ok(())
}

fn lower_yield(_y: &Yield, _lctx: &mut LowerCtx) -> Result<(), String> {
    Err("yield lowering not yet supported".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use cranelift_object::{ObjectModule, ObjectBuilder};
    use cranelift_codegen::settings;

    fn create_test_module() -> ObjectModule {
        let isa = cranelift_native::builder()
            .unwrap()
            .finish(settings::Flags::new(settings::builder()))
            .unwrap();
        let builder = ObjectBuilder::new(
            isa,
            "test",
            cranelift_module::default_libcall_names(),
        ).unwrap();
        ObjectModule::new(builder)
    }

    #[test]
    fn test_lower_simple_fn() {
        let mut module = create_test_module();
        let func = AstFn {
            name: "main".to_string(),
            params: vec![],
            ret: None,
            body: vec![
                Stmt::Expr(Expr::Call {
                    func: Box::new(Expr::Ident("print".to_string())),
                    args: vec![Expr::Int(42)],
                }),
                Stmt::Return(pylang_front::ast::Return { val: Some(Expr::Int(0)) }),
            ],
            decorators: vec![],
            captures: vec![],
        };
        let mut func_ids = HashMap::new();
        let mut closure_defs = HashMap::new();
        let result = lower_fn(&mut module, &func, &HashMap::new(), &HashMap::new(), &mut func_ids, &mut closure_defs);
        assert!(result.is_ok(), "lower_fn failed: {:?}", result.err());
    }

    #[test]
    fn test_lower_if() {
        let mut module = create_test_module();
        let func = AstFn {
            name: "main".to_string(),
            params: vec![],
            ret: None,
            body: vec![
                Stmt::If(If {
                    cond: Expr::Bool(true),
                    then: vec![Stmt::Expr(Expr::Call {
                        func: Box::new(Expr::Ident("print".to_string())),
                        args: vec![Expr::Int(1)],
                    })],
                    elif: vec![],
                    else_: None,
                }),
            ],
            decorators: vec![],
            captures: vec![],
        };
        let mut func_ids = HashMap::new();
        let mut closure_defs = HashMap::new();
        let result = lower_fn(&mut module, &func, &HashMap::new(), &HashMap::new(), &mut func_ids, &mut closure_defs);
        assert!(result.is_ok(), "lower_fn with if failed: {:?}", result.err());
    }

    #[test]
    fn test_lower_while() {
        let mut module = create_test_module();
        let func = AstFn {
            name: "main".to_string(),
            params: vec![],
            ret: None,
            body: vec![
                Stmt::While(While {
                    cond: Expr::Bool(false),
                    body: vec![],
                }),
            ],
            decorators: vec![],
            captures: vec![],
        };
        let mut func_ids = HashMap::new();
        let mut closure_defs = HashMap::new();
        let result = lower_fn(&mut module, &func, &HashMap::new(), &HashMap::new(), &mut func_ids, &mut closure_defs);
        assert!(result.is_ok(), "lower_fn with while failed: {:?}", result.err());
    }

    #[test]
    fn test_lower_binop_add() {
        let mut module = create_test_module();
        let func = AstFn {
            name: "main".to_string(),
            params: vec![],
            ret: None,
            body: vec![
                Stmt::Expr(Expr::BinOp {
                    op: BinOp::Add,
                    lhs: Box::new(Expr::Int(1)),
                    rhs: Box::new(Expr::Int(2)),
                }),
            ],
            decorators: vec![],
            captures: vec![],
        };
        let mut func_ids = HashMap::new();
        let mut closure_defs = HashMap::new();
        let result = lower_fn(&mut module, &func, &HashMap::new(), &HashMap::new(), &mut func_ids, &mut closure_defs);
        assert!(result.is_ok(), "lower_fn with add failed: {:?}", result.err());
    }

    #[test]
    fn test_lower_struct() {
        let mut module = create_test_module();
        let func = AstFn {
            name: "main".to_string(),
            params: vec![],
            ret: None,
            body: vec![
                Stmt::Expr(Expr::Call {
                    func: Box::new(Expr::Ident("Point".to_string())),
                    args: vec![Expr::Int(10), Expr::Int(20)],
                }),
                Stmt::Expr(Expr::Dot {
                    obj: Box::new(Expr::Call {
                        func: Box::new(Expr::Ident("Point".to_string())),
                        args: vec![Expr::Int(10), Expr::Int(20)],
                    }),
                    name: "x".to_string(),
                }),
            ],
            decorators: vec![],
            captures: vec![],
        };
        let mut struct_defs: HashMap<String, StructInfo> = HashMap::new();
        struct_defs.insert("Point".to_string(), StructInfo {
            _name: "Point".to_string(),
            fields: vec![
                StructField { _name: "x".to_string(), offset: 0, _ty: types::I64 },
                StructField { _name: "y".to_string(), offset: 8, _ty: types::I64 },
            ],
        });
        let mut func_ids = HashMap::new();
        let mut closure_defs = HashMap::new();
        let result = lower_fn(&mut module, &func, &struct_defs, &HashMap::new(), &mut func_ids, &mut closure_defs);
        assert!(result.is_ok(), "lower_fn with struct failed: {:?}", result.err());
    }

}
