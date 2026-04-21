//! Semantic Analyzer — name resolution, type inference, trait resolution.

use crate::ast::*;
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

    fn init_builtins(&mut self) {
        for (name, def) in [
            ("int", TypeDef::Primitive(PrimDef { name: "i64", size: 8 })),
            ("float", TypeDef::Primitive(PrimDef { name: "f64", size: 8 })),
            ("bool", TypeDef::Primitive(PrimDef { name: "b1", size: 1 })),
            ("str", TypeDef::Primitive(PrimDef { name: "str", size: 16 })),
            ("char", TypeDef::Primitive(PrimDef { name: "i32", size: 4 })),
            ("i8", TypeDef::Primitive(PrimDef { name: "i8", size: 1 })),
            ("i16", TypeDef::Primitive(PrimDef { name: "i16", size: 2 })),
            ("i32", TypeDef::Primitive(PrimDef { name: "i32", size: 4 })),
            ("i64", TypeDef::Primitive(PrimDef { name: "i64", size: 8 })),
            ("f32", TypeDef::Primitive(PrimDef { name: "f32", size: 4 })),
            ("usize", TypeDef::Primitive(PrimDef { name: "usize", size: 8 })),
        ] {
            self.types.insert(name.to_string(), def);
        }
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
            _ => Ok(()),
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
                    Ok(Type::I64)
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
                let _ = self.check_expr(func)?;
                for arg in args {
                    let _ = self.check_expr(arg)?;
                }
                Ok(Type::I64)
            }
            Expr::Tuple(elems) => {
                let types: Vec<Type> = elems
                    .iter()
                    .filter_map(|e| self.check_expr(e).ok())
                    .collect();
                Ok(Type::Tuple(types))
            }
            Expr::List(elems) => {
                for elem in elems {
                    let _ = self.check_expr(elem)?;
                }
                Ok(Type::Array(Box::new(Type::I64)))
            }
            Expr::Dict(items) => {
                for (k, v) in items {
                    let _ = self.check_expr(k)?;
                    let _ = self.check_expr(v)?;
                }
                Ok(Type::Named("dict".to_string()))
            }
            _ => Ok(Type::I64),
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
            _ => false,
        }
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
}