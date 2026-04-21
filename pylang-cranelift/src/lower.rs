use pylang_ir::{self, Function, Type as IrType, Value, Inst, BinOp as IrBinOp, Imm, Name};
use pylang_front::ast::{Stmt, Expr, Type as AstType, BinOp, Fn as AstFn};
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
        }
        Stmt::Return(r) => {
            let val = r.val.as_ref()
                .map(|e| lower_expr(e, ctx))
                .transpose()?
                .unwrap_or(Value::Imm(Imm::Unit));
            ctx.stmts.push(Inst::Return(val));
        }
        Stmt::Expr(e) => {
            let _ = lower_expr(e, ctx)?;
        }
        _ => {}
    }
    Ok(())
}

fn lower_expr(expr: &Expr, _ctx: &LoweringContext) -> Result<Value, String> {
    match expr {
        Expr::Int(n) => Ok(Value::Imm(Imm::I64(*n))),
        Expr::Float(f) => Ok(Value::Imm(Imm::F64(*f))),
        Expr::Bool(b) => Ok(Value::Imm(Imm::Bool(*b))),
        Expr::Str(s) => Ok(Value::Imm(Imm::Str(s.clone()))),
        Expr::Char(c) => Ok(Value::Imm(Imm::Char(*c))),
        Expr::None => Ok(Value::Imm(Imm::Unit)),
        Expr::Ident(name) => Ok(Value::Arg(Name::new(name))),
        Expr::BinOp { op, lhs, rhs } => {
            let l = lower_expr(lhs, _ctx)?;
            let r = lower_expr(rhs, _ctx)?;
            let ir_op = lower_binop(op);
            Ok(Value::Inst(Box::new(Inst::BinOp { op: ir_op, lhs: l, rhs: r })))
        }
        _ => Ok(Value::Imm(Imm::Unit)),
    }
}

fn lower_binop(op: &BinOp) -> IrBinOp {
    match op {
        BinOp::Add => IrBinOp::Add,
        BinOp::Sub => IrBinOp::Sub,
        BinOp::Mul => IrBinOp::Mul,
        BinOp::Div => IrBinOp::Div,
        BinOp::Rem => IrBinOp::Rem,
        _ => IrBinOp::Add,
    }
}