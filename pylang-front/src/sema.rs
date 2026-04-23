//! Semantic Analyzer — name resolution, type inference, trait resolution.

use crate::ast::{Class, Struct, *};
use std::collections::HashMap;

pub struct Sema {
    pub types: TypeMap,
    pub names: NameMap,
    scopes: Vec<NameMap>,
    errors: Vec<SemaError>,
}

pub type TypeMap = HashMap<String, TypeDef>;
pub type NameMap = HashMap<String, ResolvedName>;

#[derive(Clone, Debug)]
pub enum TypeDef {
    Primitive(PrimDef),
    Struct(StructDef),
    Class(ClassDef),
    Alias(Type),
}

#[derive(Clone, Debug)]
pub struct PrimDef {
    pub name: &'static str,
    pub size: usize,
}

#[derive(Clone, Debug)]
pub struct StructDef {
    pub name: String,
    pub fields: Vec<(String, Type)>,
}

#[derive(Clone, Debug)]
pub struct ClassDef {
    pub name: String,
    pub bases: Vec<Type>,
    pub methods: HashMap<String, Fn>,
}

#[derive(Clone, Debug)]
pub struct ResolvedName {
    pub name: String,
    pub ty: Type,
    pub def: NameDef,
}

#[derive(Clone, Debug)]
pub enum NameDef {
    Param,
    Local,
    Global,
    Function,
    Field,
    Class,
    Struct,
}

#[derive(Clone, Debug)]
pub enum SemaError {
    UndefinedName { name: String },
    DuplicateName { name: String },
    TypeMismatch { expected: Type, found: Type },
    CyclicType { name: String },
    TraitNotSatisfied { ty: Type, trait_: String },
    InvalidMutation { name: String },
    BorrowViolation { name: String },
    UnresolvedReturn,
    InvalidReturn { expected: Type, found: Type },
    CannotAssignTo { name: String },
}

#[allow(clippy::new_without_default)]
impl Sema {
    pub fn new() -> Self {
        let mut s = Self {
            types: HashMap::new(),
            names: HashMap::new(),
            scopes: Vec::new(),
            errors: Vec::new(),
        };
        s.init_builtins();
        s
    }

    #[allow(clippy::inconsistent_struct_constructor)]
    pub fn default() -> Self {
        Self::new()
    }

    fn init_builtins(&mut self) {
        self.types.insert("int".to_string(), TypeDef::Primitive(PrimDef { name: "i64", size: 8 }));
        self.types.insert("float".to_string(), TypeDef::Primitive(PrimDef { name: "f64", size: 8 }));
        self.types.insert("bool".to_string(), TypeDef::Primitive(PrimDef { name: "b1", size: 1 }));
        self.types.insert("str".to_string(), TypeDef::Primitive(PrimDef { name: "str", size: 16 }));
        self.types.insert("i64".to_string(), TypeDef::Primitive(PrimDef { name: "i64", size: 8 }));
        self.types.insert("f64".to_string(), TypeDef::Primitive(PrimDef { name: "f64", size: 8 }));
        self.types.insert("i8".to_string(), TypeDef::Primitive(PrimDef { name: "i8", size: 1 }));
        self.types.insert("i16".to_string(), TypeDef::Primitive(PrimDef { name: "i16", size: 2 }));
        self.types.insert("i32".to_string(), TypeDef::Primitive(PrimDef { name: "i32", size: 4 }));
        self.types.insert("usize".to_string(), TypeDef::Primitive(PrimDef { name: "i64", size: 8 }));
    }

    pub fn check_module(&mut self, stmts: &[Stmt]) -> Result<(), Vec<SemaError>> {
        for stmt in stmts {
            self.check_stmt(stmt)?;
        }
        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(std::mem::take(&mut self.errors))
        }
    }

    fn check_stmt(&mut self, stmt: &Stmt) -> Result<(), Vec<SemaError>> {
        match stmt {
            Stmt::Fn(f) => self.check_fn(f),
            Stmt::Class(c) => self.check_class(c),
            Stmt::Struct(s) => self.check_struct(s),
            Stmt::Let(l) => self.check_let(l),
            Stmt::Return(r) => self.check_return(r),
            Stmt::Expr(e) => {
                self.check_expr(e)?;
                Ok(())
            }
            Stmt::If(i) => self.check_if(i),
            Stmt::While(w) => self.check_while(w),
            Stmt::For(f) => self.check_for(f),
            Stmt::Try(t) => self.check_try(t),
            Stmt::Raise(r) => self.check_raise(r),
            Stmt::Loop(l) => self.check_loop(l),
            Stmt::Match(m) => self.check_match(m),
            Stmt::With(w) => self.check_with(w),
            Stmt::Yield(y) => self.check_yield(y),
            Stmt::Assert(a) => self.check_assert(a),
            Stmt::Break => Ok(()),
            Stmt::Continue => Ok(()),
            Stmt::Pass => Ok(()),
            Stmt::LetMut(l) => self.check_letmut(l),
            Stmt::Assign(a) => self.check_assign(a),
            Stmt::AssignOp(a) => self.check_assignop(a),
        }
    }

    fn check_fn(&mut self, f: &Fn) -> Result<(), Vec<SemaError>> {
        self.enter_scope();
        
        for param in &f.params {
            self.define_name(
                param.name.clone(),
                param.ty.clone(),
                NameDef::Param,
            );
        }
        
        for stmt in &f.body {
            self.check_stmt(stmt)?;
        }
        
        self.exit_scope();
        Ok(())
    }
    
    fn check_class(&mut self, c: &Class) -> Result<(), Vec<SemaError>> {
        let class_def = ClassDef {
            name: c.name.clone(),
            bases: c.bases.clone(),
            methods: HashMap::new(),
        };
        self.types.insert(c.name.clone(), TypeDef::Class(class_def));
        
        let ty = Type::Class(c.name.clone());
        self.define_name(c.name.clone(), ty, NameDef::Class);
        
        self.enter_scope();
        for stmt in &c.body {
            self.check_stmt(stmt)?;
        }
        self.exit_scope();
        
        Ok(())
    }

    fn check_struct(&mut self, s: &Struct) -> Result<(), Vec<SemaError>> {
        let struct_def = StructDef {
            name: s.name.clone(),
            fields: s.fields.clone(),
        };
        self.types.insert(s.name.clone(), TypeDef::Struct(struct_def));
        
        let ty = Type::Named(s.name.clone());
        self.define_name(s.name.clone(), ty, NameDef::Struct);
        
        Ok(())
    }

    fn define_name(&mut self, name: String, ty: Type, def: NameDef) {
        let name_for_map = name.clone();
        let resolved = ResolvedName { name, ty, def };
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name_for_map, resolved);
        } else {
            self.names.insert(name_for_map, resolved);
        }
    }

    fn check_let(&mut self, l: &Let) -> Result<(), Vec<SemaError>> {
        let val_ty = self.check_expr(&l.val)?;
        
        if let Some(ref ty) = l.ty {
            if !self.types_equal(ty, &val_ty) {
                self.errors.push(SemaError::TypeMismatch {
                    expected: ty.clone(),
                    found: val_ty.clone(),
                });
            }
        }
        
        let ty = l.ty.clone().unwrap_or_else(|| val_ty.clone());
        self.define_name(l.name.clone(), ty, NameDef::Local);
        
        Ok(())
    }

    fn check_return(&mut self, r: &Return) -> Result<(), Vec<SemaError>> {
        if let Some(ref val) = r.val {
            self.check_expr(val)?;
        }
        Ok(())
    }

    fn check_try(&mut self, t: &Try) -> Result<(), Vec<SemaError>> {
        for stmt in &t.body {
            self.check_stmt(stmt)?;
        }
        
        for handler in &t.handlers {
            self.enter_scope();
            for stmt in &handler.body {
                self.check_stmt(stmt)?;
            }
            self.exit_scope();
        }
        
        if let Some(ref finally_body) = t.finally {
            for stmt in finally_body {
                self.check_stmt(stmt)?;
            }
        }
        
        Ok(())
    }
    
    fn check_raise(&mut self, r: &Raise) -> Result<(), Vec<SemaError>> {
        self.check_expr(&r.exc)?;
        Ok(())
    }
    
    fn check_if(&mut self, i: &If) -> Result<(), Vec<SemaError>> {
        self.check_expr(&i.cond)?;
        
        self.enter_scope();
        for stmt in &i.then {
            self.check_stmt(stmt)?;
        }
        self.exit_scope();
        
        for elif in &i.elif {
            self.check_expr(&elif.cond)?;
            self.enter_scope();
            for stmt in &elif.body {
                self.check_stmt(stmt)?;
            }
            self.exit_scope();
        }
        
        if let Some(ref else_) = i.else_ {
            self.enter_scope();
            for stmt in else_ {
                self.check_stmt(stmt)?;
            }
            self.exit_scope();
        }
        Ok(())
    }

    fn check_while(&mut self, w: &While) -> Result<(), Vec<SemaError>> {
        self.check_expr(&w.cond)?;
        
        self.enter_scope();
        for stmt in &w.body {
            self.check_stmt(stmt)?;
        }
        self.exit_scope();
        Ok(())
    }

    fn check_for(&mut self, f: &For) -> Result<(), Vec<SemaError>> {
        self.check_expr(&f.iter)?;
        
        self.enter_scope();
        self.define_name(f.target.clone(), Type::I64, NameDef::Local);
        
        for stmt in &f.body {
            self.check_stmt(stmt)?;
        }
        self.exit_scope();
        Ok(())
    }

    fn check_loop(&mut self, l: &Loop) -> Result<(), Vec<SemaError>> {
        self.enter_scope();
        for stmt in &l.body {
            self.check_stmt(stmt)?;
        }
        self.exit_scope();
        Ok(())
    }

    fn check_match(&mut self, m: &Match) -> Result<(), Vec<SemaError>> {
        let _ = self.check_expr(&m.expr)?;
        for arm in &m.arms {
            self.enter_scope();
            if let Some(ref guard) = arm.guard {
                let _ = self.check_expr(guard)?;
            }
            for stmt in &arm.body {
                self.check_stmt(stmt)?;
            }
            self.exit_scope();
        }
        Ok(())
    }

    fn check_with(&mut self, w: &With) -> Result<(), Vec<SemaError>> {
        for item in &w.items {
            self.check_expr(&item.expr)?;
            if let Some(ref name) = item.as_ {
                self.define_name(name.clone(), Type::I64, NameDef::Local);
            }
        }
        self.enter_scope();
        for stmt in &w.body {
            self.check_stmt(stmt)?;
        }
        self.exit_scope();
        Ok(())
    }

    fn check_yield(&mut self, y: &Yield) -> Result<(), Vec<SemaError>> {
        if let Some(ref val) = y.val {
            self.check_expr(val)?;
        }
        Ok(())
    }

    fn check_assert(&mut self, a: &Assert) -> Result<(), Vec<SemaError>> {
        let cond_ty = self.check_expr(&a.cond)?;
        if cond_ty != Type::Bool {
            self.errors.push(SemaError::TypeMismatch {
                expected: Type::Bool,
                found: cond_ty,
            });
        }
        if let Some(ref msg) = a.msg {
            self.check_expr(msg)?;
        }
        Ok(())
    }

    fn check_letmut(&mut self, l: &LetMut) -> Result<(), Vec<SemaError>> {
        let val_ty = self.check_expr(&l.val)?;
        if let Some(ref ty) = l.ty {
            if !self.types_equal(ty, &val_ty) {
                self.errors.push(SemaError::TypeMismatch {
                    expected: ty.clone(),
                    found: val_ty.clone(),
                });
            }
        }
        let ty = l.ty.clone().unwrap_or(val_ty);
        self.define_name(l.name.clone(), ty, NameDef::Local);
        Ok(())
    }

    fn check_assign(&mut self, a: &Assign) -> Result<(), Vec<SemaError>> {
        let _ = self.check_expr(&a.target)?;
        let _ = self.check_expr(&a.val)?;
        Ok(())
    }

    fn check_assignop(&mut self, a: &AssignOp) -> Result<(), Vec<SemaError>> {
        let _ = self.check_expr(&a.target)?;
        let _ = self.check_expr(&a.val)?;
        Ok(())
    }

    fn check_expr(&mut self, expr: &Expr) -> Result<Type, Vec<SemaError>> {
        match expr {
            Expr::Int(_) => Ok(Type::I64),
            Expr::Float(_) => Ok(Type::F64),
            Expr::Bool(_) => Ok(Type::Bool),
            Expr::Str(_) => Ok(Type::Named("str".to_string())),
            Expr::Char(_) => Ok(Type::Char),
            Expr::None => Ok(Type::Unit),
            Expr::Ident(name) => {
                if let Some(resolved) = self.lookup_name(name) {
                    Ok(resolved.ty)
                } else {
                    self.errors.push(SemaError::UndefinedName { name: name.clone() });
                    Ok(Type::Unit)
                }
            }
            Expr::BinOp { op, lhs, rhs } => {
                let _ = self.check_expr(lhs)?;
                let _ = self.check_expr(rhs)?;
                Ok(self.binop_type(op))
            }
            Expr::UnOp { op, val } => {
                let _ = self.check_expr(val)?;
                Ok(self.unop_type(op))
            }
            Expr::Cmp { op: _, lhs, rhs } => {
                let _ = self.check_expr(lhs)?;
                let _ = self.check_expr(rhs)?;
                Ok(Type::Bool)
            }
            Expr::Call { func, args } => {
                let func_ty = self.check_expr(func)?;
                for arg in args {
                    let _ = self.check_expr(arg)?;
                }
                Ok(self.call_return_type(&func_ty))
            }
            Expr::Method { obj, name: _, args } => {
                let _ = self.check_expr(obj)?;
                for arg in args {
                    let _ = self.check_expr(arg)?;
                }
                Ok(Type::Unit)
            }
            Expr::Dot { obj, name } => {
                let obj_ty = self.check_expr(obj)?;
                Ok(self.field_type(&obj_ty, name))
            }
            Expr::Index { obj, idx } => {
                let obj_ty = self.check_expr(obj)?;
                let _ = self.check_expr(idx)?;
                Ok(self.element_type(&obj_ty))
            }
            Expr::Slice { obj, start, end, step } => {
                let _ = self.check_expr(obj)?;
                if let Some(s) = start { let _ = self.check_expr(s); }
                if let Some(e) = end { let _ = self.check_expr(e); }
                if let Some(t) = step { let _ = self.check_expr(t); }
                Ok(Type::Slice(Box::new(Type::I64)))
            }
            Expr::Tuple(elems) => {
                let types: Vec<Type> = elems
                    .iter()
                    .filter_map(|e| self.check_expr(e).ok())
                    .collect();
                Ok(Type::Tuple(types))
            }
            Expr::List(elems) => {
                let elem_ty = elems.first()
                    .and_then(|e| self.check_expr(e).ok())
                    .unwrap_or(Type::Unit);
                for elem in elems {
                    let _ = self.check_expr(elem)?;
                }
                Ok(Type::Array(Box::new(elem_ty)))
            }
            Expr::Dict(items) => {
                for (k, v) in items {
                    let _ = self.check_expr(k)?;
                    let _ = self.check_expr(v)?;
                }
                Ok(Type::Named("dict".to_string()))
            }
            Expr::Set(elems) => {
                for elem in elems {
                    let _ = self.check_expr(elem)?;
                }
                Ok(Type::Named("set".to_string()))
            }
            Expr::Lambda { params, body } => {
                self.enter_scope();
                let param_tys: Vec<Type> = params.iter().map(|p| {
                    self.define_name(p.name.clone(), p.ty.clone(), NameDef::Param);
                    p.ty.clone()
                }).collect();
                let ret_ty = self.check_expr(body).unwrap_or(Type::Unit);
                self.exit_scope();
                Ok(Type::Fn {
                    params: param_tys,
                    ret: Box::new(ret_ty),
                })
            }
            Expr::If { cond, then, else_ } => {
                let cond_ty = self.check_expr(cond)?;
                if cond_ty != Type::Bool {
                    self.errors.push(SemaError::TypeMismatch {
                        expected: Type::Bool,
                        found: cond_ty,
                    });
                }
                let then_ty = self.check_expr(then)?;
                let else_ty = self.check_expr(else_)?;
                Ok(self.common_type(&then_ty, &else_ty))
            }
            Expr::Match { expr, arms } => {
                let _ = self.check_expr(expr)?;
                let mut result_ty = Type::Unit;
                for arm in arms {
                    if let Some(ref guard) = arm.guard {
                        let _ = self.check_expr(guard)?;
                    }
                    for stmt in &arm.body {
                        self.check_stmt(stmt)?;
                    }
                    let arm_ty = self.last_stmt_expr_type(&arm.body);
                    result_ty = self.common_type(&result_ty, &arm_ty);
                }
                Ok(result_ty)
            }
            Expr::ListComp { body, generators } => {
                for gen in generators {
                    self.enter_scope();
                    self.define_name(gen.target.clone(), Type::I64, NameDef::Local);
                    let _ = self.check_expr(&gen.iter)?;
                    if let Some(ref cond) = gen.cond {
                        let _ = self.check_expr(cond)?;
                    }
                    self.exit_scope();
                }
                let elem_ty = self.check_expr(body)?;
                Ok(Type::Array(Box::new(elem_ty)))
            }
            Expr::DictComp { key, val, generators } => {
                for gen in generators {
                    self.enter_scope();
                    self.define_name(gen.target.clone(), Type::I64, NameDef::Local);
                    let _ = self.check_expr(&gen.iter)?;
                    if let Some(ref cond) = gen.cond {
                        let _ = self.check_expr(cond)?;
                    }
                    self.exit_scope();
                }
                let _ = self.check_expr(key)?;
                let _ = self.check_expr(val)?;
                Ok(Type::Named("dict".to_string()))
            }
            Expr::Await(inner) => {
                self.check_expr(inner)
            }
            Expr::Async { params, body } => {
                self.enter_scope();
                for param in params {
                    self.define_name(param.name.clone(), param.ty.clone(), NameDef::Param);
                }
                for stmt in body {
                    self.check_stmt(stmt)?;
                }
                self.exit_scope();
                Ok(Type::Fn {
                    params: params.iter().map(|p| p.ty.clone()).collect(),
                    ret: Box::new(Type::Unit),
                })
            }
            Expr::YieldFrom(inner) => {
                self.check_expr(inner)
            }
            Expr::Subscript(elems) => {
                for e in elems {
                    let _ = self.check_expr(e).ok();
                }
                Ok(Type::Array(Box::new(Type::I64)))
            }
            Expr::Bytes(_) => Ok(Type::Array(Box::new(Type::I64))),
        }
    }

    fn binop_type(&self, op: &BinOp) -> Type {
        match op {
            BinOp::Add | BinOp::Sub | BinOp::Mul | BinOp::Div | BinOp::Rem => Type::I64,
            BinOp::FloorDiv => Type::I64,
            BinOp::Pow => Type::I64,
            BinOp::BitAnd | BinOp::BitOr | BinOp::BitXor => Type::I64,
            BinOp::LShift | BinOp::RShift => Type::I64,
        }
    }

    fn unop_type(&self, op: &UnOp) -> Type {
        match op {
            UnOp::Pos | UnOp::Neg => Type::I64,
            UnOp::Not => Type::Bool,
            UnOp::BitNot => Type::I64,
        }
    }

    fn lookup_name(&self, name: &str) -> Option<ResolvedName> {
        for scope in self.scopes.iter().rev() {
            if let Some(n) = scope.get(name) {
                return Some(n.clone());
            }
        }
        self.names.get(name).cloned()
    }

    fn enter_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn exit_scope(&mut self) {
        self.scopes.pop();
    }

    fn types_equal(&self, a: &Type, b: &Type) -> bool {
        match (a, b) {
            (Type::I64, Type::I64) => true,
            (Type::F64, Type::F64) => true,
            (Type::Bool, Type::Bool) => true,
            (Type::Char, Type::Char) => true,
            (Type::Unit, Type::Unit) => true,
            (Type::Named(a), Type::Named(b)) => a == b,
            (Type::Class(a), Type::Class(b)) => a == b,
            (Type::Array(a), Type::Array(b)) => self.types_equal(a, b),
            (Type::Slice(a), Type::Slice(b)) => self.types_equal(a, b),
            (Type::Tuple(a), Type::Tuple(b)) => a.len() == b.len() && a.iter().zip(b.iter()).all(|(x,y)| self.types_equal(x, y)),
            (Type::Fn { params: pa, ret: ra }, Type::Fn { params: pb, ret: rb }) =>
                pa.len() == pb.len() && **ra == **rb && pa.iter().zip(pb.iter()).all(|(x,y)| self.types_equal(x, y)),
            _ => false,
        }
    }

    fn call_return_type(&self, func_ty: &Type) -> Type {
        match func_ty {
            Type::Fn { ret, .. } => (**ret).clone(),
            _ => Type::Unit,
        }
    }

    fn field_type(&self, obj_ty: &Type, _name: &str) -> Type {
        if let Type::Class(name) = obj_ty {
            if let Some(TypeDef::Class(c)) = self.types.get(name) {
                if let Some(method) = c.methods.get(_name) {
                    return method.ret.clone().unwrap_or(Type::Unit);
                }
            }
        }
        Type::Unit
    }

    fn element_type(&self, ty: &Type) -> Type {
        match ty {
            Type::Array(elem) => (**elem).clone(),
            Type::Slice(elem) => (**elem).clone(),
            Type::Tuple(elems) => elems.first().cloned().unwrap_or(Type::Unit),
            _ => Type::Unit,
        }
    }

    fn common_type(&self, a: &Type, b: &Type) -> Type {
        if self.types_equal(a, b) {
            return a.clone();
        }
        match (a, b) {
            (Type::I64, Type::F64) | (Type::F64, Type::I64) => Type::F64,
            (Type::I64, _) | (_, Type::I64) => Type::I64,
            (Type::F64, _) | (_, Type::F64) => Type::F64,
            _ => Type::Unit,
        }
    }

    fn last_stmt_expr_type(&mut self, stmts: &[Stmt]) -> Type {
        stmts.last().map(|s| match s {
            Stmt::Expr(e) => self.check_expr(e).unwrap_or(Type::Unit),
            Stmt::Return(r) => r.val.as_ref()
                .and_then(|e| self.check_expr(e).ok())
                .unwrap_or(Type::Unit),
            _ => Type::Unit,
        }).unwrap_or(Type::Unit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{Stmt as AstStmt, Let as AstLet, Return as AstReturn, Expr as AstExpr, Type as AstType};

    #[test]
    fn test_builtins() {
        let sema = Sema::new();
        assert!(sema.types.contains_key("int"));
        assert!(sema.types.contains_key("float"));
        assert!(sema.types.contains_key("bool"));
        assert!(sema.types.contains_key("str"));
    }

    #[test]
    fn test_check_let() {
        let mut sema = Sema::new();
        let let_stmt = AstStmt::Let(AstLet {
            name: "x".to_string(),
            ty: Some(AstType::I64),
            val: AstExpr::Int(42),
        });
        let result = sema.check_stmt(&let_stmt);
        assert!(result.is_ok());
    }

    #[test]
    fn test_check_return() {
        let mut sema = Sema::new();
        let ret_stmt = AstStmt::Return(AstReturn {
            val: Some(AstExpr::Int(0)),
        });
        let result = sema.check_stmt(&ret_stmt);
        assert!(result.is_ok());
    }

    #[test]
    fn test_check_expr_int() {
        let mut sema = Sema::new();
        let ty = sema.check_expr(&AstExpr::Int(42)).unwrap();
        assert_eq!(ty, AstType::I64);
    }

    #[test]
    fn test_check_expr_bool() {
        let mut sema = Sema::new();
        let ty = sema.check_expr(&AstExpr::Bool(true)).unwrap();
        assert_eq!(ty, AstType::Bool);
    }

    #[test]
    fn test_check_expr_str() {
        let mut sema = Sema::new();
        let ty = sema.check_expr(&AstExpr::Str("hello".to_string())).unwrap();
        assert_eq!(ty, AstType::Named("str".to_string()));
    }

    #[test]
    fn test_check_expr_lambda() {
        let mut sema = Sema::new();
        let lambda_expr = AstExpr::Lambda {
            params: vec![crate::ast::Param {
                name: "x".to_string(),
                ty: AstType::I64,
                default: None,
            }],
            body: Box::new(AstExpr::Int(42)),
        };
        let ty = sema.check_expr(&lambda_expr).unwrap();
        assert_eq!(ty, AstType::Fn {
            params: vec![AstType::I64],
            ret: Box::new(AstType::I64),
        });
    }

    #[test]
    fn test_check_expr_binop() {
        let mut sema = Sema::new();
        let binop_expr = AstExpr::BinOp {
            op: crate::ast::BinOp::Add,
            lhs: Box::new(AstExpr::Int(1)),
            rhs: Box::new(AstExpr::Int(2)),
        };
        let ty = sema.check_expr(&binop_expr).unwrap();
        assert_eq!(ty, AstType::I64);
    }

    #[test]
    fn test_check_expr_if() {
        let mut sema = Sema::new();
        let if_expr = AstExpr::If {
            cond: Box::new(AstExpr::Bool(true)),
            then: Box::new(AstExpr::Int(1)),
            else_: Box::new(AstExpr::Int(0)),
        };
        let ty = sema.check_expr(&if_expr).unwrap();
        assert_eq!(ty, AstType::I64);
    }

    #[test]
    fn test_check_loop() {
        let mut sema = Sema::new();
        let loop_stmt = AstStmt::Loop(crate::ast::Loop {
            body: vec![],
        });
        let result = sema.check_stmt(&loop_stmt);
        assert!(result.is_ok());
    }

    #[test]
    fn test_check_while() {
        let mut sema = Sema::new();
        let while_stmt = AstStmt::While(crate::ast::While {
            cond: AstExpr::Bool(true),
            body: vec![],
        });
        let result = sema.check_stmt(&while_stmt);
        assert!(result.is_ok());
    }

    #[test]
    fn test_check_for() {
        let mut sema = Sema::new();
        let for_stmt = AstStmt::For(crate::ast::For {
            target: "i".to_string(),
            iter: AstExpr::Int(10),
            body: vec![],
        });
        let result = sema.check_stmt(&for_stmt);
        assert!(result.is_ok());
    }

    #[test]
    fn test_check_match() {
        let mut sema = Sema::new();
        let match_stmt = AstStmt::Match(crate::ast::Match {
            expr: AstExpr::Int(1),
            arms: vec![
                crate::ast::MatchArm {
                    pat: crate::ast::Pattern::Literal(crate::ast::Literal::Int(1)),
                    guard: None,
                    body: vec![],
                },
            ],
        });
        let result = sema.check_stmt(&match_stmt);
        assert!(result.is_ok());
    }

    #[test]
    fn test_check_try() {
        let mut sema = Sema::new();
        let try_stmt = AstStmt::Try(crate::ast::Try {
            body: vec![],
            handlers: vec![],
            finally: None,
        });
        let result = sema.check_stmt(&try_stmt);
        assert!(result.is_ok());
    }

    #[test]
    fn test_check_with() {
        let mut sema = Sema::new();
        let with_stmt = AstStmt::With(crate::ast::With {
            items: vec![crate::ast::WithItem {
                expr: AstExpr::None,
                as_: Some("f".to_string()),
            }],
            body: vec![],
        });
        let result = sema.check_stmt(&with_stmt);
        assert!(result.is_ok());
    }

    #[test]
    fn test_check_raise() {
        let mut sema = Sema::new();
        let raise_stmt = AstStmt::Raise(crate::ast::Raise {
            exc: AstExpr::Str("error".to_string()),
        });
        let result = sema.check_stmt(&raise_stmt);
        assert!(result.is_ok());
    }

    #[test]
    fn test_check_assert() {
        let mut sema = Sema::new();
        let assert_stmt = AstStmt::Assert(crate::ast::Assert {
            cond: AstExpr::Bool(true),
            msg: None,
        });
        let result = sema.check_stmt(&assert_stmt);
        assert!(result.is_ok());
    }

    #[test]
    fn test_check_break() {
        let mut sema = Sema::new();
        let break_stmt = AstStmt::Break;
        let result = sema.check_stmt(&break_stmt);
        assert!(result.is_ok());
    }

    #[test]
    fn test_check_continue() {
        let mut sema = Sema::new();
        let continue_stmt = AstStmt::Continue;
        let result = sema.check_stmt(&continue_stmt);
        assert!(result.is_ok());
    }

    #[test]
    fn test_check_pass() {
        let mut sema = Sema::new();
        let pass_stmt = AstStmt::Pass;
        let result = sema.check_stmt(&pass_stmt);
        assert!(result.is_ok());
    }
}