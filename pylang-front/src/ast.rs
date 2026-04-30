//! AST узлы для Pylang.
//!
//! Все узлы неизменяемые (immutable by default) — соответствует borrow semantics.



#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    Char(char),
}

#[derive(Clone, Debug)]
pub enum Stmt {
    Fn(Fn),
    Class(Class),
    Struct(Struct),
    Let(Let),
    LetMut(LetMut),
    Assign(Assign),
    AssignOp(AssignOp),
    If(If),
    While(While),
    For(For),
    Loop(Loop),
    Match(Match),
    Try(Try),
    With(With),
    Return(Return),
    Yield(Yield),
    Raise(Raise),
    Break,
    Continue,
    Pass,
    Assert(Assert),
    Expr(Expr),
}

#[derive(Clone, Debug)]
pub struct Fn {
    pub name: String,
    pub params: Vec<Param>,
    pub ret: Option<Type>,
    pub body: Vec<Stmt>,
    pub decorators: Vec<Expr>,
    pub captures: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct Param {
    pub name: String,
    pub ty: Type,
    pub default: Option<Expr>,
}

#[derive(Clone, Debug)]
pub struct Class {
    pub name: String,
    pub bases: Vec<Type>,
    pub body: Vec<Stmt>,
}

#[derive(Clone, Debug)]
pub struct Struct {
    pub name: String,
    pub fields: Vec<(String, Type)>,
}

#[derive(Clone, Debug)]
pub struct Let {
    pub name: String,
    pub ty: Option<Type>,
    pub val: Expr,
}

#[derive(Clone, Debug)]
pub struct LetMut {
    pub name: String,
    pub ty: Option<Type>,
    pub val: Expr,
}

#[derive(Clone, Debug)]
pub struct Assign {
    pub target: Box<Expr>,
    pub val: Expr,
}

#[derive(Clone, Debug)]
pub struct AssignOp {
    pub target: Expr,
    pub op: BinOp,
    pub val: Expr,
}

#[derive(Clone, Debug)]
pub enum BinOp {
    Add, Sub, Mul, Div, Rem,
    FloorDiv,
    Pow,
    BitAnd, BitOr, BitXor,
    LShift, RShift,
}

#[derive(Clone, Debug)]
pub enum CmpOp {
    Eq, Ne, Lt, Le, Gt, Ge,
    Is, IsNot,
    In, NotIn,
}

#[derive(Clone, Debug)]
pub struct If {
    pub cond: Expr,
    pub then: Vec<Stmt>,
    pub elif: Vec<Elif>,
    pub else_: Option<Vec<Stmt>>,
}

#[derive(Clone, Debug)]
pub struct Elif {
    pub cond: Expr,
    pub body: Vec<Stmt>,
}

#[derive(Clone, Debug)]
pub struct While {
    pub cond: Expr,
    pub body: Vec<Stmt>,
}

#[derive(Clone, Debug)]
pub struct For {
    pub target: String,
    pub iter: Expr,
    pub body: Vec<Stmt>,
}

#[derive(Clone, Debug)]
pub struct Loop {
    pub body: Vec<Stmt>,
}

#[derive(Clone, Debug)]
pub struct Match {
    pub expr: Expr,
    pub arms: Vec<MatchArm>,
}

#[derive(Clone, Debug)]
pub struct MatchArm {
    pub pat: Pattern,
    pub guard: Option<Expr>,
    pub body: Vec<Stmt>,
}

#[derive(Clone, Debug)]
pub enum Pattern {
    Wildcard,
    Literal(Literal),
    Binding(String),
    Tuple(Vec<Pattern>),
    Variant(String, Vec<Pattern>),
}

#[derive(Clone, Debug)]
pub struct Try {
    pub body: Vec<Stmt>,
    pub handlers: Vec<Handler>,
    pub finally: Option<Vec<Stmt>>,
}

#[derive(Clone, Debug)]
pub struct Handler {
    pub exc: Option<Type>,
    pub binding: Option<String>,
    pub body: Vec<Stmt>,
}

#[derive(Clone, Debug)]
pub struct With {
    pub items: Vec<WithItem>,
    pub body: Vec<Stmt>,
}

#[derive(Clone, Debug)]
pub struct WithItem {
    pub expr: Expr,
    pub as_: Option<String>,
}

#[derive(Clone, Debug)]
pub struct Return {
    pub val: Option<Expr>,
}

#[derive(Clone, Debug)]
pub struct Yield {
    pub val: Option<Expr>,
}

#[derive(Clone, Debug)]
pub struct Raise {
    pub exc: Expr,
}

#[derive(Clone, Debug)]
pub struct Assert {
    pub cond: Expr,
    pub msg: Option<Expr>,
}

#[derive(Clone, Debug)]
pub enum Expr {
    None,
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    Char(char),
    Bytes(Vec<u8>),
    List(Vec<Expr>),
    Dict(Vec<(Expr, Expr)>),
    Set(Vec<Expr>),
    BinOp {
        op: BinOp,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    UnOp {
        op: UnOp,
        val: Box<Expr>,
    },
    Cmp {
        op: CmpOp,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    Call {
        func: Box<Expr>,
        args: Vec<Expr>,
    },
    Method {
        obj: Box<Expr>,
        name: String,
        args: Vec<Expr>,
    },
    Index {
        obj: Box<Expr>,
        idx: Box<Expr>,
    },
    Slice {
        obj: Box<Expr>,
        start: Option<Box<Expr>>,
        end: Option<Box<Expr>>,
        step: Option<Box<Expr>>,
    },
    Dot {
        obj: Box<Expr>,
        name: String,
    },
    Lambda {
        params: Vec<Param>,
        body: Box<Expr>,
    },
    If {
        cond: Box<Expr>,
        then: Box<Expr>,
        else_: Box<Expr>,
    },
    Match {
        expr: Box<Expr>,
        arms: Vec<MatchArm>,
    },
    Tuple(Vec<Expr>),
    ListComp {
        body: Box<Expr>,
        generators: Vec<CompGen>,
    },
    DictComp {
        key: Box<Expr>,
        val: Box<Expr>,
        generators: Vec<CompGen>,
    },
    Await(Box<Expr>),
    Async {
        params: Vec<Param>,
        body: Vec<Stmt>,
    },
    YieldFrom(Box<Expr>),
    Ident(String),
    Subscript(Vec<Expr>),
    FString(Vec<FStringPart>),
}

#[derive(Clone, Debug)]
pub enum FStringPart {
    Lit(String),
    Expr(Box<Expr>),
}

#[derive(Clone, Debug)]
pub enum UnOp {
    Not, Pos, Neg, BitNot,
}

#[derive(Clone, Debug)]
pub struct CompGen {
    pub target: String,
    pub iter: Expr,
    pub cond: Option<Expr>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    Unit,
    Bool,
    I8,
    I16,
    I32,
    I64,
    F32,
    F64,
    Usize,
    Char,
    String,
    Named(String),
    Class(String),
    Tuple(Vec<Type>),
    Fn {
        params: Vec<Type>,
        ret: Box<Type>,
    },
    Array(Box<Type>),
    Slice(Box<Type>),
    Generic {
        base: String,
        args: Vec<Type>,
    },
    Box(Box<Type>),
    Ref(Box<Type>),
    Ptr(Box<Type>),
}