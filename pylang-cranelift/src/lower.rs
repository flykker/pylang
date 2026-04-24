use cranelift::prelude::*;
use cranelift_codegen::ir::{UserFuncName, InstBuilder, BlockArg};
use cranelift_module::{Module, Linkage, FuncId};
use pylang_front::ast::{
    Stmt, Expr, Type as AstType, BinOp, Fn as AstFn, CmpOp, UnOp,
    If, While, For, Loop, Match, Try, With, Raise, Assert, Yield,
};
use std::collections::HashMap;

struct LoopContext {
    exit_block: Block,
    continue_block: Block,
}

pub struct LowerCtx<'a> {
    pub builder: FunctionBuilder<'a>,
    pub module: &'a mut dyn Module,
    pub locals: HashMap<String, Variable>,
    pub block_filled: bool,
    loop_stack: Vec<LoopContext>,
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

pub fn lower_module(module: &mut dyn Module, stmts: &[Stmt]) -> Result<(), String> {
    for stmt in stmts {
        if let Stmt::Fn(f) = stmt {
            lower_fn(module, f)?;
        }
    }
    Ok(())
}

pub fn lower_fn(module: &mut dyn Module, f: &AstFn) -> Result<FuncId, String> {
    let mut sig = module.make_signature();
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

    let func_id = module.declare_function(&f.name, Linkage::Export, &sig)
        .map_err(|e| e.to_string())?;

    let mut ctx = module.make_context();
    ctx.func.signature = sig;
    ctx.func.name = UserFuncName::user(0, func_id.as_u32());

    let mut fn_ctx = FunctionBuilderContext::new();
    let mut builder = FunctionBuilder::new(&mut ctx.func, &mut fn_ctx);

    let entry = builder.create_block();
    builder.switch_to_block(entry);

    // Force stack allocation to ensure 16-byte alignment before calls
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
        block_filled: false,
        loop_stack: Vec::new(),
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
        AstType::Named(n) => Err(format!("unknown type: {}", n)),
        _ => Err(format!("unsupported type: {:?}", ty)),
    }
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
            let var = lctx.locals.get(name)
                .ok_or_else(|| format!("undefined variable: {}", name))?;
            Ok(lctx.builder.use_var(*var))
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
            let offset = get_field_offset(name);
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
        Expr::Slice { obj, start, end, step } => {
            let obj_val = lower_expr(obj, lctx)?;
            let _start = start.as_ref().map(|s| lower_expr(s, lctx)).transpose()?;
            let _end = end.as_ref().map(|e| lower_expr(e, lctx)).transpose()?;
            let _step = step.as_ref().map(|s| lower_expr(s, lctx)).transpose()?;
            Ok(obj_val)
        }
        Expr::Tuple(elems) | Expr::List(elems) | Expr::Set(elems) => {
            let vals: Vec<Value> = elems.iter()
                .map(|e| lower_expr(e, lctx))
                .collect::<Result<Vec<_>, _>>()?;
            let size = elems.len() * 8;
            let size_val = lctx.builder.ins().iconst(types::I64, size as i64);
            let ptr = call_runtime(lctx, "alloc", &[size_val], types::I64)?;
            for (i, val) in vals.iter().enumerate() {
                let offset = lctx.builder.ins().iconst(types::I64, (i * 8) as i64);
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
            let vals: Vec<Value> = elems.iter()
                .map(|e| lower_expr(e, lctx))
                .collect::<Result<Vec<_>, _>>()?;
            if vals.is_empty() {
                Ok(lctx.builder.ins().iconst(types::I64, 0))
            } else {
                Ok(vals[0])
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
    if let Expr::Ident(name) = func {
        let arg_vals: Vec<Value> = args.iter()
            .map(|a| lower_expr(a, lctx))
            .collect::<Result<Vec<_>, _>>()?;

        match name.as_str() {
            "print" => {
                if let Some(&val) = arg_vals.first() {
                    call_runtime(lctx, "print_int", &[val], types::I64)?;
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
            "range" => {
                Ok(arg_vals.first().copied().unwrap_or_else(|| lctx.builder.ins().iconst(types::I64, 0)))
            }
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
            "str" => {
                Ok(arg_vals.first().copied().unwrap_or_else(|| lctx.builder.ins().iconst(types::I64, 0)))
            }
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
            "input" => Ok(lctx.builder.ins().iconst(types::I64, 0)),
            _ => {
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
    } else {
        Err("unsupported call target".to_string())
    }
}

fn lower_method(obj: &Expr, name: &str, args: &[Expr], lctx: &mut LowerCtx) -> Result<Value, String> {
    let obj_val = lower_expr(obj, lctx)?;
    let mut arg_vals = vec![obj_val];
    for arg in args {
        arg_vals.push(lower_expr(arg, lctx)?);
    }

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

fn get_field_offset(name: &str) -> i64 {
    match name {
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

    Err("for loop only supports range() for now".to_string())
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
        };
        let result = lower_fn(&mut module, &func);
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
        };
        let result = lower_fn(&mut module, &func);
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
        };
        let result = lower_fn(&mut module, &func);
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
        };
        let result = lower_fn(&mut module, &func);
        assert!(result.is_ok(), "lower_fn with add failed: {:?}", result.err());
    }
}
