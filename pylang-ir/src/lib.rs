//! Pylang IR — минимальная SSA-подобная промежуточное представление.
//!
//! ~20 core ops. Кастомные: Lock/Spawn, GetRef/Release, Yield/Await.
//! Подробности — в PLAN.md.

use std::fmt;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Name(pub String);

impl Name {
    pub fn new(s: &str) -> Self {
        Name(s.to_string())
    }
}

impl fmt::Debug for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Default for Name {
    fn default() -> Self {
        Name(String::new())
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct TypeId(pub u32);

impl fmt::Debug for TypeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TypeId({})", self.0)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PrimType {
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
}

impl PrimType {
    pub fn to_clif(self) -> &'static str {
        match self {
            PrimType::Unit => "void",
            PrimType::Bool => "b1",
            PrimType::I8 => "i8",
            PrimType::I16 => "i16",
            PrimType::I32 => "i32",
            PrimType::I64 => "i64",
            PrimType::F32 => "f32",
            PrimType::F64 => "f64",
            PrimType::Usize => "i64",
            PrimType::Char => "i32",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    Prim(PrimType),
    Never,
    Ref(TypeId),
    Tuple(Vec<Type>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Function {
    pub name: Name,
    pub params: Vec<(Name, Type)>,
    pub ret: Box<Type>,
    pub body: Vec<Inst>,
    pub res: Value,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Imm(Imm),
    Arg(Name),
    Inst(Box<Inst>),
    Undefined,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Imm {
    Unit,
    Bool(bool),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    Char(char),
    Str(String),
    Name(Name),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Inst {
    Unreachable,
    Nop,

    // Memory
    Alloc {
        ty: Type,
        size: Box<Value>,
        init: Option<Value>,
    },
    Load {
        ptr: Value,
        ty: Type,
        offset: Box<Value>,
    },
    Store {
        ptr: Value,
        val: Value,
        offset: Box<Value>,
    },
    Free(Value),

    // Arithmetic
    BinOp {
        op: BinOp,
        lhs: Value,
        rhs: Value,
    },
    UnOp {
        op: UnOp,
        val: Value,
    },
    Cmp {
        op: CmpOp,
        lhs: Value,
        rhs: Value,
    },

    // Control flow
    Call {
        func: Name,
        args: Vec<Value>,
    },
    Closure {
        func: Name,
        captured: Vec<Value>,
    },
    Branch {
        cond: Value,
        then: Name,
        else_: Name,
    },
    Jump(Name),
    Return(Value),
    Phi {
        incoming: Vec<(Name, Value)>,
    },

    // Generators / async
    Yield(Value),
    Await(Value),

    // Exceptions
    Try {
        body: Vec<Inst>,
        handlers: Vec<Handler>,
        finally: Option<Vec<Inst>>,
    },
    Raise(Value),
    Rethrow,

    // Custom concurrency (lowered to Cranelift atomics)
    Lock {
        ptr: Value,
        body: Name,
    },
    Spawn {
        func: Name,
        args: Vec<Value>,
    },
    GetRef {
        ptr: Value,
        mutable: bool,
    },
    Release(Value),

    // Aggregate
    Tuple(Vec<Value>),
    ProjTuple {
        val: Value,
        index: u32,
    },
    Cast {
        val: Value,
        ty: Type,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BinOp {
    Add, Sub, Mul, Div, Rem,
    And, Or, Xor,
    Shl, Shr,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum UnOp {
    Not, Neg,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CmpOp {
    Eq, Ne,
    Lt, Le, Gt, Ge,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Handler {
    pub exc: Option<TypeId>,
    pub binding: Option<Name>,
    pub body: Vec<Inst>,
}