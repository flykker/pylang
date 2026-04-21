use pylang_ir::{self, Function, Type as IrType, Value, Inst, BinOp as IrBinOp, Imm, Name};
use pylang_front::ast::{Stmt, Expr, Type as AstType, BinOp, Fn as AstFn, CmpOp, UnOp, If, While, For, Loop, Match, Try, With, Raise, Assert};
use std::collections::HashMap;

pub struct LoweringContext {
    locals: HashMap<String, Value>,
    stmts: Vec<Inst>,
}

impl Default for LoweringContext {
    fn default() -> Self {
        Self::new()
    }
}

impl LoweringContext {
    pub fn new() -> Self {
        Self {
            locals: HashMap::new(),
            stmts: Vec::new(),
        }
    }
}

pub fn lower_module(stmts: &[Stmt]) -> Result<Vec<Function>, String> {
    let mut functions = Vec::new();
    let mut ctx = LoweringContext::new();
    
    for stmt in stmts {
        if let Stmt::Fn(f) = stmt {
            let ir_fn = lower_fn(f, &mut ctx)?;
            functions.push(ir_fn);
        }
    }
    
    Ok(functions)
}

pub fn lower_fn(f: &AstFn, ctx: &mut LoweringContext) -> Result<Function, String> {
    ctx.locals.clear();
    ctx.stmts.clear();
    
    for param in &f.params {
        let name = Name::new(&param.name);
        ctx.locals.insert(param.name.clone(), Value::Arg(name));
    }
    
    for stmt in &f.body {
        lower_stmt(stmt, ctx)?;
    }
    
    let ret_ty = f.ret.clone()
        .map(|t| lower_type(&t))
        .unwrap_or(IrType::Prim(pylang_ir::PrimType::Unit));
    
    let params: Vec<(Name, IrType)> = f.params.iter()
        .map(|p| {
            let n = Name::new(&p.name);
            let t = lower_type(&p.ty);
            (n, t)
        })
        .collect();
    
    Ok(Function {
        name: Name::new(&f.name),
        params,
        ret: Box::new(ret_ty),
        body: std::mem::take(&mut ctx.stmts),
        res: Value::Undefined,
    })
}

fn lower_type(ty: &AstType) -> IrType {
    match ty {
        AstType::I64 => IrType::Prim(pylang_ir::PrimType::I64),
        AstType::F64 => IrType::Prim(pylang_ir::PrimType::F64),
        AstType::Bool => IrType::Prim(pylang_ir::PrimType::Bool),
        AstType::Char => IrType::Prim(pylang_ir::PrimType::Char),
        AstType::Unit => IrType::Prim(pylang_ir::PrimType::Unit),
        AstType::Named(n) => {
            match n.as_str() {
                "int" => IrType::Prim(pylang_ir::PrimType::I64),
                "float" => IrType::Prim(pylang_ir::PrimType::F64),
                "bool" => IrType::Prim(pylang_ir::PrimType::Bool),
                "str" => IrType::Prim(pylang_ir::PrimType::I64),
                _ => IrType::Prim(pylang_ir::PrimType::I64),
            }
        }
        _ => IrType::Prim(pylang_ir::PrimType::I64),
    }
}

fn lower_stmt(stmt: &Stmt, ctx: &mut LoweringContext) -> Result<(), String> {
    match stmt {
        Stmt::Let(l) => {
            let val = lower_expr(&l.val, ctx)?;
            ctx.locals.insert(l.name.clone(), val);
            Ok(())
        }
        Stmt::LetMut(l) => {
            let val = lower_expr(&l.val, ctx)?;
            ctx.locals.insert(l.name.clone(), val);
            Ok(())
        }
        Stmt::Assign(a) => {
            let ptr = lower_expr(&a.target, ctx)?;
            let val = lower_expr(&a.val, ctx)?;
            ctx.stmts.push(Inst::Store {
                ptr,
                val,
                offset: Box::new(Value::Imm(Imm::I64(0))),
            });
            Ok(())
        }
        Stmt::AssignOp(a) => {
            let lhs = lower_expr(&a.target, ctx)?;
            let rhs = lower_expr(&a.val, ctx)?;
            let op = lower_binop(&a.op)?;
            ctx.stmts.push(Inst::BinOp { op, lhs, rhs });
            Ok(())
        }
        Stmt::Return(r) => {
            let val = r.val.as_ref()
                .map(|e| lower_expr(e, ctx))
                .transpose()?
                .unwrap_or(Value::Imm(Imm::Unit));
            ctx.stmts.push(Inst::Return(val));
            Ok(())
        }
        Stmt::Expr(e) => {
            let _ = lower_expr(e, ctx)?;
            Ok(())
        }
        Stmt::If(i) => lower_if(i, ctx),
        Stmt::While(w) => lower_while(w, ctx),
        Stmt::For(f) => lower_for(f, ctx),
        Stmt::Loop(l) => lower_loop(l, ctx),
        Stmt::Match(m) => lower_match(m, ctx),
        Stmt::Try(t) => lower_try(t, ctx),
        Stmt::With(w) => lower_with(w, ctx),
        Stmt::Raise(r) => lower_raise(r, ctx),
        Stmt::Assert(a) => lower_assert(a, ctx),
        Stmt::Break => {
            ctx.stmts.push(Inst::Jump(Name::new("__break")));
            Ok(())
        }
        Stmt::Continue => {
            ctx.stmts.push(Inst::Jump(Name::new("__continue")));
            Ok(())
        }
        Stmt::Pass => Ok(()),
        _ => Err(format!("unsupported statement: {:?}", stmt)),
    }
}

fn lower_if(i: &If, ctx: &mut LoweringContext) -> Result<(), String> {
    let cond = lower_expr(&i.cond, ctx)?;
    let then_block = Name::new(&format!("__if_then_{}", ctx.stmts.len()));
    let else_block = Name::new(&format!("__if_else_{}", ctx.stmts.len()));
    let end_block = Name::new(&format!("__if_end_{}", ctx.stmts.len()));
    ctx.stmts.push(Inst::Branch {
        cond,
        then: then_block.clone(),
        else_: else_block.clone(),
    });
    ctx.stmts.push(Inst::Jump(end_block.clone()));
    for stmt in &i.then {
        lower_stmt(stmt, ctx)?;
    }
    ctx.stmts.push(Inst::Jump(end_block.clone()));
    if let Some(ref else_) = i.else_ {
        for stmt in else_ {
            lower_stmt(stmt, ctx)?;
        }
    }
    ctx.stmts.push(Inst::Jump(end_block));
    Ok(())
}

fn lower_while(w: &While, ctx: &mut LoweringContext) -> Result<(), String> {
    let start_block = Name::new(&format!("__while_start_{}", ctx.stmts.len()));
    let body_block = Name::new(&format!("__while_body_{}", ctx.stmts.len()));
    let end_block = Name::new(&format!("__while_end_{}", ctx.stmts.len()));
    ctx.stmts.push(Inst::Jump(start_block.clone()));
    let cond = lower_expr(&w.cond, ctx)?;
    ctx.stmts.push(Inst::Branch {
        cond,
        then: body_block.clone(),
        else_: end_block.clone(),
    });
    for stmt in &w.body {
        lower_stmt(stmt, ctx)?;
    }
    ctx.stmts.push(Inst::Jump(start_block));
    ctx.stmts.push(Inst::Jump(end_block));
    Ok(())
}

fn lower_for(f: &For, ctx: &mut LoweringContext) -> Result<(), String> {
    let _ = lower_expr(&f.iter, ctx)?;
    let body_block = Name::new(&format!("__for_body_{}", ctx.stmts.len()));
    let end_block = Name::new(&format!("__for_end_{}", ctx.stmts.len()));
    for stmt in &f.body {
        lower_stmt(stmt, ctx)?;
    }
    ctx.stmts.push(Inst::Jump(end_block));
    Ok(())
}

fn lower_loop(l: &Loop, ctx: &mut LoweringContext) -> Result<(), String> {
    let start_block = Name::new(&format!("__loop_start_{}", ctx.stmts.len()));
    ctx.stmts.push(Inst::Jump(start_block.clone()));
    for stmt in &l.body {
        lower_stmt(stmt, ctx)?;
    }
    ctx.stmts.push(Inst::Jump(start_block));
    Ok(())
}

fn lower_match(m: &Match, ctx: &mut LoweringContext) -> Result<(), String> {
    let _ = lower_expr(&m.expr, ctx)?;
    for arm in &m.arms {
        for stmt in &arm.body {
            lower_stmt(stmt, ctx)?;
        }
    }
    Ok(())
}

fn lower_try(t: &Try, ctx: &mut LoweringContext) -> Result<(), String> {
    let mut body_insts = Vec::new();
    for stmt in &t.body {
        lower_stmt(stmt, ctx)?;
    }
    let handlers = t.handlers.iter().map(|h| {
        pylang_ir::Handler {
            exc: None,
            binding: h.binding.as_ref().map(|n| Name::new(n)),
            body: vec![],
        }
    }).collect();
    ctx.stmts.push(Inst::Try {
        body: body_insts,
        handlers,
        finally: None,
    });
    Ok(())
}

fn lower_with(w: &With, ctx: &mut LoweringContext) -> Result<(), String> {
    for item in &w.items {
        let _ = lower_expr(&item.expr, ctx)?;
    }
    for stmt in &w.body {
        lower_stmt(stmt, ctx)?;
    }
    Ok(())
}

fn lower_raise(r: &Raise, ctx: &mut LoweringContext) -> Result<(), String> {
    let exc = lower_expr(&r.exc, ctx)?;
    ctx.stmts.push(Inst::Raise(exc));
    Ok(())
}

fn lower_assert(a: &Assert, ctx: &mut LoweringContext) -> Result<(), String> {
    let cond = lower_expr(&a.cond, ctx)?;
    if a.msg.is_some() {
        let _ = lower_expr(a.msg.as_ref().unwrap(), ctx)?;
    }
    ctx.stmts.push(Inst::Nop);
    Ok(())
}

fn lower_expr(expr: &Expr, ctx: &mut LoweringContext) -> Result<Value, String> {
    match expr {
        Expr::Int(n) => Ok(Value::Imm(Imm::I64(*n))),
        Expr::Float(f) => Ok(Value::Imm(Imm::F64(*f))),
        Expr::Bool(b) => Ok(Value::Imm(Imm::Bool(*b))),
        Expr::Str(s) => Ok(Value::Imm(Imm::Str(s.clone()))),
        Expr::Char(c) => Ok(Value::Imm(Imm::Char(*c))),
        Expr::None => Ok(Value::Imm(Imm::Unit)),
        Expr::Ident(name) => Ok(Value::Arg(Name::new(name))),
        Expr::BinOp { op, lhs, rhs } => {
            let l = lower_expr(lhs, ctx)?;
            let r = lower_expr(rhs, ctx)?;
            let ir_op = lower_binop(op)?;
            Ok(Value::Inst(Box::new(Inst::BinOp { op: ir_op, lhs: l, rhs: r })))
        }
        Expr::UnOp { op, val } => {
            let v = lower_expr(val, ctx)?;
            let ir_op = lower_unop(op);
            Ok(Value::Inst(Box::new(Inst::UnOp { op: ir_op, val: v })))
        }
        Expr::Cmp { op, lhs, rhs } => {
            let l = lower_expr(lhs, ctx)?;
            let r = lower_expr(rhs, ctx)?;
            let ir_op = lower_cmpop(op);
            Ok(Value::Inst(Box::new(Inst::Cmp { op: ir_op, lhs: l, rhs: r })))
        }
        Expr::Call { func, args } => {
            let func_val = lower_expr(func, ctx)?;
            let args_vals: Result<Vec<_>, _> = args.iter()
                .map(|a| lower_expr(a, ctx))
                .collect();
            match func_val {
                Value::Arg(name) => Ok(Value::Inst(Box::new(Inst::Call {
                    func: name,
                    args: args_vals?,
                }))),
                _ => Err(format!("unsupported call target: {:?}", func_val)),
            }
        }
        Expr::Method { obj, name, args } => {
            let _ = lower_expr(obj, ctx)?;
            for arg in args {
                let _ = lower_expr(arg, ctx)?;
            }
            Err(format!("unsupported expression: Method({})", name))
        }
        Expr::Dot { obj, name } => {
            let _ = lower_expr(obj, ctx)?;
            Err(format!("unsupported expression: Dot({})", name))
        }
        Expr::Index { obj, idx } => {
            let _ = lower_expr(obj, ctx)?;
            let _ = lower_expr(idx, ctx)?;
            Err("unsupported expression: Index".to_string())
        }
        Expr::Slice { obj, start, end, step } => {
            let _ = lower_expr(obj, ctx)?;
            if let Some(s) = start { let _ = lower_expr(s, ctx); }
            if let Some(e) = end { let _ = lower_expr(e, ctx); }
            if let Some(t) = step { let _ = lower_expr(t, ctx); }
            Err("unsupported expression: Slice".to_string())
        }
        Expr::Tuple(elems) => {
            let vals: Result<Vec<_>, _> = elems.iter()
                .map(|e| lower_expr(e, ctx))
                .collect();
            Ok(Value::Inst(Box::new(Inst::Tuple(vals?))))
        }
        Expr::List(elems) => {
            for elem in elems {
                let _ = lower_expr(elem, ctx)?;
            }
            Err("unsupported expression: List".to_string())
        }
        Expr::Dict(items) => {
            for (k, v) in items {
                let _ = lower_expr(k, ctx)?;
                let _ = lower_expr(v, ctx)?;
            }
            Err("unsupported expression: Dict".to_string())
        }
        Expr::Set(elems) => {
            for elem in elems {
                let _ = lower_expr(elem, ctx)?;
            }
            Err("unsupported expression: Set".to_string())
        }
        Expr::ListComp { body, generators } => {
            let _ = lower_expr(body, ctx)?;
            for gen in generators {
                let _ = lower_expr(&gen.iter, ctx)?;
            }
            Err("unsupported expression: ListComp".to_string())
        }
        Expr::DictComp { key, val, generators } => {
            let _ = lower_expr(key, ctx)?;
            let _ = lower_expr(val, ctx)?;
            for gen in generators {
                let _ = lower_expr(&gen.iter, ctx)?;
            }
            Err("unsupported expression: DictComp".to_string())
        }
        Expr::Await(inner) => lower_expr(inner, ctx),
        Expr::Async { params, body } => {
            for p in params {
                let _ = lower_type(&p.ty);
            }
            for stmt in body {
                lower_stmt(stmt, ctx)?;
            }
            Err("unsupported expression: Async".to_string())
        }
        Expr::YieldFrom(inner) => lower_expr(inner, ctx),
        Expr::Lambda { params, body } => {
            for p in params {
                let _ = lower_type(&p.ty);
            }
            let _ = lower_expr(body, ctx)?;
            Err("unsupported expression: Lambda".to_string())
        }
        Expr::If { cond, then, else_ } => {
            let _ = lower_expr(cond, ctx)?;
            let _ = lower_expr(then, ctx)?;
            let _ = lower_expr(else_, ctx)?;
            Err("unsupported expression: If".to_string())
        }
        Expr::Match { expr, arms } => {
            let _ = lower_expr(expr, ctx)?;
            for arm in arms {
                let mut tmp_ctx = LoweringContext::new();
                for stmt in &arm.body {
                    lower_stmt(stmt, &mut tmp_ctx)?;
                }
            }
            Err("unsupported expression: Match".to_string())
        }
        Expr::Subscript(elems) => {
            for e in elems {
                let _ = lower_expr(e, ctx)?;
            }
            Err("unsupported expression: Subscript".to_string())
        }
        Expr::Bytes(_) => Err("unsupported expression: Bytes".to_string()),
    }
}

fn lower_binop(op: &BinOp) -> Result<IrBinOp, String> {
    match op {
        BinOp::Add => Ok(IrBinOp::Add),
        BinOp::Sub => Ok(IrBinOp::Sub),
        BinOp::Mul => Ok(IrBinOp::Mul),
        BinOp::Div => Ok(IrBinOp::Div),
        BinOp::Rem => Ok(IrBinOp::Rem),
        BinOp::FloorDiv => Ok(IrBinOp::Add),
        BinOp::Pow => Ok(IrBinOp::Mul),
        BinOp::BitAnd => Ok(IrBinOp::And),
        BinOp::BitOr => Ok(IrBinOp::Or),
        BinOp::BitXor => Ok(IrBinOp::Xor),
        BinOp::LShift => Ok(IrBinOp::Shl),
        BinOp::RShift => Ok(IrBinOp::Shr),
    }
}

fn lower_unop(op: &UnOp) -> pylang_ir::UnOp {
    match op {
        UnOp::Not => pylang_ir::UnOp::Not,
        UnOp::Pos => pylang_ir::UnOp::Neg,
        UnOp::Neg => pylang_ir::UnOp::Neg,
        UnOp::BitNot => pylang_ir::UnOp::Not,
    }
}

fn lower_cmpop(op: &CmpOp) -> pylang_ir::CmpOp {
    match op {
        CmpOp::Eq => pylang_ir::CmpOp::Eq,
        CmpOp::Ne => pylang_ir::CmpOp::Ne,
        CmpOp::Lt => pylang_ir::CmpOp::Lt,
        CmpOp::Le => pylang_ir::CmpOp::Le,
        CmpOp::Gt => pylang_ir::CmpOp::Gt,
        CmpOp::Ge => pylang_ir::CmpOp::Ge,
        CmpOp::Is => pylang_ir::CmpOp::Eq,
        CmpOp::IsNot => pylang_ir::CmpOp::Ne,
        CmpOp::In => pylang_ir::CmpOp::Eq,
        CmpOp::NotIn => pylang_ir::CmpOp::Ne,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lower_int() {
        let ctx = &mut LoweringContext::new();
        let result = lower_expr(&Expr::Int(42), ctx);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Value::Imm(Imm::I64(42)));
    }

    #[test]
    fn test_lower_bool() {
        let ctx = &mut LoweringContext::new();
        let result = lower_expr(&Expr::Bool(true), ctx);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Value::Imm(Imm::Bool(true)));
    }

    #[test]
    fn test_lower_ident() {
        let ctx = &mut LoweringContext::new();
        let result = lower_expr(&Expr::Ident("x".to_string()), ctx);
        assert!(result.is_ok());
    }

    #[test]
    fn test_lower_binop_add() {
        let ctx = &mut LoweringContext::new();
        let result = lower_expr(&Expr::BinOp {
            op: BinOp::Add,
            lhs: Box::new(Expr::Int(1)),
            rhs: Box::new(Expr::Int(2)),
        }, ctx);
        assert!(result.is_ok());
    }

    #[test]
    fn test_lower_binop_sub() {
        let ctx = &mut LoweringContext::new();
        let result = lower_expr(&Expr::BinOp {
            op: BinOp::Sub,
            lhs: Box::new(Expr::Int(5)),
            rhs: Box::new(Expr::Int(3)),
        }, ctx);
        assert!(result.is_ok());
    }

    #[test]
    fn test_lower_binop_mul() {
        let ctx = &mut LoweringContext::new();
        let result = lower_expr(&Expr::BinOp {
            op: BinOp::Mul,
            lhs: Box::new(Expr::Int(2)),
            rhs: Box::new(Expr::Int(3)),
        }, ctx);
        assert!(result.is_ok());
    }

    #[test]
    fn test_lower_binop_div() {
        let ctx = &mut LoweringContext::new();
        let result = lower_expr(&Expr::BinOp {
            op: BinOp::Div,
            lhs: Box::new(Expr::Int(6)),
            rhs: Box::new(Expr::Int(2)),
        }, ctx);
        assert!(result.is_ok());
    }

    #[test]
    fn test_lower_call() {
        let ctx = &mut LoweringContext::new();
        let result = lower_expr(&Expr::Call {
            func: Box::new(Expr::Ident("print".to_string())),
            args: vec![Expr::Int(42)],
        }, ctx);
        assert!(result.is_ok());
    }

    #[test]
    fn test_lower_stmt_let() {
        let ctx = &mut LoweringContext::new();
        let stmt = Stmt::Let(pylang_front::ast::Let {
            name: "x".to_string(),
            ty: None,
            val: Expr::Int(42),
        });
        let result = lower_stmt(&stmt, ctx);
        assert!(result.is_ok());
    }

    #[test]
    fn test_lower_stmt_return() {
        let ctx = &mut LoweringContext::new();
        let stmt = Stmt::Return(pylang_front::ast::Return {
            val: Some(Expr::Int(0)),
        });
        let result = lower_stmt(&stmt, ctx);
        assert!(result.is_ok());
    }

    #[test]
    fn test_lower_stmt_if() {
        let ctx = &mut LoweringContext::new();
        let stmt = Stmt::If(If {
            cond: Expr::Bool(true),
            then: vec![],
            elif: vec![],
            else_: None,
        });
        let result = lower_stmt(&stmt, ctx);
        assert!(result.is_ok());
    }

    #[test]
    fn test_lower_stmt_while() {
        let ctx = &mut LoweringContext::new();
        let stmt = Stmt::While(While {
            cond: Expr::Bool(true),
            body: vec![],
        });
        let result = lower_stmt(&stmt, ctx);
        assert!(result.is_ok());
    }

    #[test]
    fn test_lower_stmt_for() {
        let ctx = &mut LoweringContext::new();
        let stmt = Stmt::For(For {
            target: "i".to_string(),
            iter: Expr::Int(10),
            body: vec![],
        });
        let result = lower_stmt(&stmt, ctx);
        assert!(result.is_ok());
    }

    #[test]
    fn test_lower_stmt_loop() {
        let ctx = &mut LoweringContext::new();
        let stmt = Stmt::Loop(Loop {
            body: vec![],
        });
        let result = lower_stmt(&stmt, ctx);
        assert!(result.is_ok());
    }

    #[test]
    fn test_lower_stmt_try() {
        let ctx = &mut LoweringContext::new();
        let stmt = Stmt::Try(Try {
            body: vec![],
            handlers: vec![],
            finally: None,
        });
        let result = lower_stmt(&stmt, ctx);
        assert!(result.is_ok());
    }

    #[test]
    fn test_lower_stmt_raise() {
        let ctx = &mut LoweringContext::new();
        let stmt = Stmt::Raise(Raise {
            exc: Expr::Str("error".to_string()),
        });
        let result = lower_stmt(&stmt, ctx);
        assert!(result.is_ok());
    }

    #[test]
    fn test_lower_stmt_assert() {
        let ctx = &mut LoweringContext::new();
        let stmt = Stmt::Assert(Assert {
            cond: Expr::Bool(true),
            msg: None,
        });
        let result = lower_stmt(&stmt, ctx);
        assert!(result.is_ok());
    }

    #[test]
    fn test_lower_stmt_break() {
        let ctx = &mut LoweringContext::new();
        let stmt = Stmt::Break;
        let result = lower_stmt(&stmt, ctx);
        assert!(result.is_ok());
    }

    #[test]
    fn test_lower_stmt_continue() {
        let ctx = &mut LoweringContext::new();
        let stmt = Stmt::Continue;
        let result = lower_stmt(&stmt, ctx);
        assert!(result.is_ok());
    }

    #[test]
    fn test_lower_stmt_pass() {
        let ctx = &mut LoweringContext::new();
        let stmt = Stmt::Pass;
        let result = lower_stmt(&stmt, ctx);
        assert!(result.is_ok());
    }

    #[test]
    fn test_lower_binop_all() {
        let ops = vec![
            BinOp::Add, BinOp::Sub, BinOp::Mul, BinOp::Div, BinOp::Rem,
            BinOp::FloorDiv, BinOp::Pow,
            BinOp::BitAnd, BinOp::BitOr, BinOp::BitXor,
            BinOp::LShift, BinOp::RShift,
        ];
        for op in ops {
            let result = lower_binop(&op);
            assert!(result.is_ok(), "failed for {:?}", op);
        }
    }

    #[test]
    fn test_lower_cmpop_all() {
        let ops = vec![
            CmpOp::Eq, CmpOp::Ne, CmpOp::Lt, CmpOp::Le,
            CmpOp::Gt, CmpOp::Ge,
            CmpOp::Is, CmpOp::IsNot,
            CmpOp::In, CmpOp::NotIn,
        ];
        for op in ops {
            let _result = lower_cmpop(&op);
        }
    }

    #[test]
    fn test_lower_unop() {
        let ctx = &mut LoweringContext::new();
        assert!(lower_expr(&Expr::UnOp { op: UnOp::Not, val: Box::new(Expr::Bool(true)) }, ctx).is_ok());
        assert!(lower_expr(&Expr::UnOp { op: UnOp::Neg, val: Box::new(Expr::Int(1)) }, ctx).is_ok());
        assert!(lower_expr(&Expr::UnOp { op: UnOp::Pos, val: Box::new(Expr::Int(1)) }, ctx).is_ok());
        assert!(lower_expr(&Expr::UnOp { op: UnOp::BitNot, val: Box::new(Expr::Int(1)) }, ctx).is_ok());
    }
}