//! PEG Parser для Pylang.
//!
//! Recursive descent. Парсит type-annotated Python subset.

use crate::ast::{Class, CompGen, *};
use crate::lexer::{Lexer, Pos, Span, Spanned, TokenKind};
use crate::sema::Sema;

pub struct Parser<'src> {
    lexer: Lexer<'src>,
    ast: Vec<Stmt>,
    errors: Vec<ParseError>,
    current: Option<Spanned<TokenKind>>,
}

#[derive(Clone, Debug)]
pub enum ParseError {
    UnexpectedToken {
        expected: &'static str,
        found: TokenKind,
        span: Span,
    },
    UnterminatedString { start: Pos, end: Pos },
    InvalidSyntax { span: Span },
    DuplicateParam { name: String, span: Span },
    MissingReturnType { span: Span },
    MissingBody { span: Span },
    InvalidAssignmentTarget { span: Span },
    InvalidPattern { span: Span },
    ExpectedIdent { found: TokenKind, span: Span },
}

impl<'src> Parser<'src> {
    pub fn new(source: &'src str) -> Self {
        let mut lexer = Lexer::new(source);
        let current = lexer.next_token();
        Self {
            lexer,
            ast: Vec::new(),
            errors: Vec::new(),
            current,
        }
    }

    pub fn parse(&mut self, _sema: &mut Sema) -> Result<Vec<Stmt>, Vec<ParseError>> {
        while let Some(token) = &self.current {
            if token.value == TokenKind::Eof {
                break;
            }
            if token.value == TokenKind::Newline || token.value == TokenKind::Semicolon {
                self.bump();
                continue;
            }
            match self.parse_stmt() {
                Ok(stmt) => self.ast.push(stmt),
                Err(e) => {
                    self.errors.push(e);
                    self.skip_to_newline();
                }
            }
        }
        if self.errors.is_empty() {
            Ok(std::mem::take(&mut self.ast))
        } else {
            Err(std::mem::take(&mut self.errors))
        }
    }

    fn bump(&mut self) -> Option<Spanned<TokenKind>> {
        let prev = self.current.clone();
        self.current = self.lexer.next_token();
        prev
    }

    fn expect(&mut self, kind: TokenKind) -> Result<Spanned<TokenKind>, ParseError> {
        if let Some(ref t) = self.current {
            if std::mem::discriminant(&t.value) == std::mem::discriminant(&kind) {
                let span = t.span;
                return self.bump().ok_or(ParseError::InvalidSyntax { span });
            }
            return Err(ParseError::UnexpectedToken {
                expected: "expected token",
                found: t.value.clone(),
                span: t.span,
            });
        }
        Err(ParseError::InvalidSyntax { span: Span::default() })
    }

    fn skip_to_newline(&mut self) {
        while let Some(ref t) = self.current {
            if t.value == TokenKind::Eof || t.value == TokenKind::Newline {
                break;
            }
            self.bump();
        }
    }

    fn at(&self, kind: &TokenKind) -> bool {
        self.current
            .as_ref()
            .map(|t| std::mem::discriminant(&t.value) == std::mem::discriminant(kind))
            .unwrap_or(false)
    }

    fn parse_stmt(&mut self) -> Result<Stmt, ParseError> {
        let token = self.current.clone().ok_or(ParseError::InvalidSyntax { span: Span::default() })?;
        
        match &token.value {
            TokenKind::At => self.parse_decorated_fn(),
            TokenKind::Def => self.parse_fn(),
            TokenKind::Class => self.parse_class(),
            TokenKind::Struct => self.parse_struct(),
            TokenKind::Let => self.parse_let(),
            TokenKind::If => self.parse_if(),
            TokenKind::While => self.parse_while(),
            TokenKind::For => self.parse_for(),
            TokenKind::Loop => self.parse_loop(),
            TokenKind::Match => self.parse_match(),
            TokenKind::Try => self.parse_try(),
            TokenKind::With => self.parse_with(),
            TokenKind::Return => self.parse_return(),
            TokenKind::Yield => self.parse_yield(),
            TokenKind::Raise => self.parse_raise(),
            TokenKind::Break => {
                self.bump();
                Ok(Stmt::Break)
            }
            TokenKind::Continue => {
                self.bump();
                Ok(Stmt::Continue)
            }
            TokenKind::Pass => {
                self.bump();
                Ok(Stmt::Pass)
            }
            TokenKind::Assert => self.parse_assert(),
            TokenKind::Except | TokenKind::Finally | TokenKind::Eof => Err(ParseError::InvalidSyntax { span: token.span }),
            _ => {
                let expr = self.parse_expr()?;
                if self.at(&TokenKind::Eq) && (matches!(&expr, Expr::Ident(_)) || matches!(&expr, Expr::Dot { .. })) {
                    self.bump();
                    let val = self.parse_expr()?;
                    Ok(Stmt::Assign(Assign { target: Box::new(expr), val }))
                } else {
                    Ok(Stmt::Expr(expr))
                }
            }
        }
    }

    fn parse_decorated_fn(&mut self) -> Result<Stmt, ParseError> {
        let mut decorators = Vec::new();
        while self.at(&TokenKind::At) {
            self.bump();
            decorators.push(self.parse_expr()?);
            while self.at(&TokenKind::Newline) {
                self.bump();
            }
        }
        let mut fn_stmt = self.parse_fn()?;
        if let Stmt::Fn(ref mut f) = fn_stmt {
            f.decorators = decorators;
        }
        Ok(fn_stmt)
    }

    fn parse_fn(&mut self) -> Result<Stmt, ParseError> {
        self.bump();
        
        let name_tok = self.expect(TokenKind::Ident(String::new()))?;
        let name = match &name_tok.value {
            TokenKind::Ident(s) => s.clone(),
            _ => return Err(ParseError::UnexpectedToken { expected: "function name", found: name_tok.value.clone(), span: name_tok.span }),
        };
        
        self.expect(TokenKind::LParen)?;
        let mut params = Vec::new();
        
        while !self.at(&TokenKind::RParen) {
            if !params.is_empty() {
                self.expect(TokenKind::Comma)?;
                if self.at(&TokenKind::RParen) { break; }
            }
            params.push(self.parse_param()?);
        }
        
        self.expect(TokenKind::RParen)?;
        
        let ret = if self.at(&TokenKind::RArrow) {
            self.bump();
            Some(self.parse_type()?)
        } else {
            None
        };
        
        self.expect(TokenKind::Colon)?;
        self.skip_newline()?;
        
        let body = self.parse_suite()?;
        
        Ok(Stmt::Fn(Fn { name, params, ret, body, decorators: vec![], captures: vec![] }))
    }

    fn parse_param(&mut self) -> Result<Param, ParseError> {
        let name_tok = self.expect(TokenKind::Ident(String::new()))?;
        let name = match &name_tok.value {
            TokenKind::Ident(s) => s.clone(),
            _ => return Err(ParseError::UnexpectedToken { expected: "parameter name", found: name_tok.value.clone(), span: name_tok.span }),
        };
        
        let ty = if self.at(&TokenKind::Colon) {
            self.bump();
            self.parse_type()?
        } else {
            Type::Named("int".to_string())
        };
        
        let default = if self.at(&TokenKind::Eq) {
            self.bump();
            Some(self.parse_expr()?)
        } else {
            None
        };
        
        Ok(Param { name, ty, default })
    }

    fn parse_type(&mut self) -> Result<Type, ParseError> {
        let token = self.current.clone().ok_or(ParseError::InvalidSyntax { span: Span::default() })?;
        
        let ty = match &token.value {
            TokenKind::Ident(name) => {
                self.bump();
                if self.at(&TokenKind::LBracket) {
                    self.bump();
                    let mut args = Vec::new();
                    while !self.at(&TokenKind::RBracket) {
                        if !args.is_empty() {
                            self.expect(TokenKind::Comma)?;
                        }
                        args.push(self.parse_type()?);
                    }
                    self.expect(TokenKind::RBracket)?;
                    Type::Generic { base: name.clone(), args }
                } else if name == "int" {
                    Type::I64
                } else if name == "float" {
                    Type::F64
                } else if name == "bool" {
                    Type::Bool
                } else if name == "str" || name == "String" {
                    Type::String
                } else if name == "char" {
                    Type::Char
                } else {
                    Type::Named(name.clone())
                }
            }
            TokenKind::LParen => {
                self.bump();
                let mut args = Vec::new();
                while !self.at(&TokenKind::RParen) {
                    if !args.is_empty() {
                        self.expect(TokenKind::Comma)?;
                    }
                    args.push(self.parse_type()?);
                }
                self.expect(TokenKind::RParen)?;
                Type::Tuple(args)
            }
            TokenKind::Star => {
                self.bump();
                let inner = self.parse_type()?;
                Type::Box(Box::new(inner))
            }
            TokenKind::Ampersand => {
                self.bump();
                let inner = self.parse_type()?;
                Type::Ref(Box::new(inner))
            }
            _ => return Err(ParseError::UnexpectedToken { expected: "type", found: token.value.clone(), span: token.span }),
        };
        
        Ok(ty)
    }

    fn parse_suite(&mut self) -> Result<Vec<Stmt>, ParseError> {
        let mut stmts = Vec::new();
        
        // Skip leading newlines to determine base column (indentation level)
        let base_col = loop {
            if let Some(ref t) = self.current {
                if t.value == TokenKind::Newline {
                    self.bump();
                    continue;
                }
                break t.span.start.col;
            } else {
                return Ok(stmts);
            }
        };
        
        while let Some(ref t) = self.current {
            match &t.value {
                TokenKind::Eof => break,
                TokenKind::Class | TokenKind::Struct => break,
                TokenKind::Newline => {
                    self.bump();
                    continue;
                }
                _ => {
                    let col = t.span.start.col;
                    // Stop when we see a token at a lower column than the base
                    // indentation of this block
                    if col < base_col {
                        break;
                    }
                    if let Ok(stmt) = self.parse_stmt() {
                        stmts.push(stmt);
                    } else {
                        break;
                    }
                }
            }
        }
        
        Ok(stmts)
    }

    fn skip_newline(&mut self) -> Result<(), ParseError> {
        while self.at(&TokenKind::Newline) {
            self.bump();
        }
        Ok(())
    }

    fn parse_let(&mut self) -> Result<Stmt, ParseError> {
        self.bump();
        
        let name_tok = self.expect(TokenKind::Ident(String::new()))?;
        let name = match &name_tok.value {
            TokenKind::Ident(s) => s.clone(),
            _ => return Err(ParseError::UnexpectedToken { expected: "variable name", found: name_tok.value.clone(), span: name_tok.span }),
        };
        
        let ty = if self.at(&TokenKind::Colon) {
            self.bump();
            Some(self.parse_type()?)
        } else {
            None
        };
        
        self.expect(TokenKind::Eq)?;
        let val = self.parse_expr()?;
        
        Ok(Stmt::Let(Let { name, ty, val }))
    }

    fn parse_return(&mut self) -> Result<Stmt, ParseError> {
        self.bump();
        
        let val = if self.at(&TokenKind::Newline) || self.current.is_none() {
            None
        } else {
            Some(self.parse_expr()?)
        };
        
        Ok(Stmt::Return(Return { val }))
    }

    fn parse_expr(&mut self) -> Result<Expr, ParseError> {
        self.parse_or()
    }

    fn parse_or(&mut self) -> Result<Expr, ParseError> {
        let mut lhs = self.parse_and()?;
        
        while self.at(&TokenKind::OrOr) || self.at(&TokenKind::Pipe) {
            self.bump();
            let rhs = self.parse_and()?;
            lhs = Expr::BinOp {
                op: BinOp::BitOr,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            };
        }
        
        Ok(lhs)
    }

    fn parse_and(&mut self) -> Result<Expr, ParseError> {
        let mut lhs = self.parse_not()?;
        
        while self.at(&TokenKind::AndAnd) || self.at(&TokenKind::Ampersand) {
            self.bump();
            let rhs = self.parse_not()?;
            lhs = Expr::BinOp {
                op: BinOp::BitAnd,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            };
        }
        
        Ok(lhs)
    }

    fn parse_not(&mut self) -> Result<Expr, ParseError> {
        if self.at(&TokenKind::Bang) {
            self.bump();
            let val = Box::new(self.parse_not()?);
            return Ok(Expr::UnOp { op: UnOp::Not, val });
        }
        self.parse_cmp()
    }

    fn parse_cmp(&mut self) -> Result<Expr, ParseError> {
        let mut lhs = self.parse_arith()?;
        
        while let Some(ref t) = self.current {
            let cmp_op = match &t.value {
                TokenKind::EqEq => Some(CmpOp::Eq),
                TokenKind::Ne => Some(CmpOp::Ne),
                TokenKind::Lt => Some(CmpOp::Lt),
                TokenKind::Le => Some(CmpOp::Le),
                TokenKind::Gt => Some(CmpOp::Gt),
                TokenKind::Ge => Some(CmpOp::Ge),
                _ => None,
            };
            
            if let Some(op) = cmp_op {
                self.bump();
                let rhs = self.parse_arith()?;
                lhs = Expr::Cmp { op, lhs: Box::new(lhs), rhs: Box::new(rhs) };
            } else {
                break;
            }
        }
        
        Ok(lhs)
    }

    fn parse_arith(&mut self) -> Result<Expr, ParseError> {
        let mut lhs = self.parse_term()?;
        
        while let Some(ref t) = self.current {
            let bin_op = match &t.value {
                TokenKind::Plus => Some(BinOp::Add),
                TokenKind::Minus => Some(BinOp::Sub),
                _ => None,
            };
            
            if let Some(op) = bin_op {
                self.bump();
                let rhs = self.parse_term()?;
                lhs = Expr::BinOp { op, lhs: Box::new(lhs), rhs: Box::new(rhs) };
            } else {
                break;
            }
        }
        
        Ok(lhs)
    }

    fn parse_term(&mut self) -> Result<Expr, ParseError> {
        let mut lhs = self.parse_unary()?;
        
        while let Some(ref t) = self.current {
            let bin_op = match &t.value {
                TokenKind::Star => Some(BinOp::Mul),
                TokenKind::Slash => Some(BinOp::Div),
                TokenKind::Percent => Some(BinOp::Rem),
                _ => None,
            };
            
            if let Some(op) = bin_op {
                self.bump();
                let rhs = self.parse_unary()?;
                lhs = Expr::BinOp { op, lhs: Box::new(lhs), rhs: Box::new(rhs) };
            } else {
                break;
            }
        }
        
        Ok(lhs)
    }

    fn parse_unary(&mut self) -> Result<Expr, ParseError> {
        if let Some(ref t) = self.current {
            match &t.value {
                TokenKind::Plus => {
                    self.bump();
                    let val = Box::new(self.parse_unary()?);
                    return Ok(Expr::UnOp { op: UnOp::Pos, val });
                }
                TokenKind::Minus => {
                    self.bump();
                    let val = Box::new(self.parse_unary()?);
                    return Ok(Expr::UnOp { op: UnOp::Neg, val });
                }
                TokenKind::Bang => {
                    self.bump();
                    let val = Box::new(self.parse_unary()?);
                    return Ok(Expr::UnOp { op: UnOp::Not, val });
                }
                TokenKind::Tilde => {
                    self.bump();
                    let val = Box::new(self.parse_unary()?);
                    return Ok(Expr::UnOp { op: UnOp::BitNot, val });
                }
                _ => {}
            }
        }
        self.parse_call()
    }

    fn parse_call(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_atom()?;
        
        loop {
            if self.at(&TokenKind::LParen) {
                self.bump();
                let mut args = Vec::new();
                while !self.at(&TokenKind::RParen) {
                    if !args.is_empty() {
                        self.expect(TokenKind::Comma)?;
                    }
                    args.push(self.parse_expr()?);
                }
                self.expect(TokenKind::RParen)?;
                expr = Expr::Call { func: Box::new(expr), args };
            } else if self.at(&TokenKind::Dot) {
                self.bump();
                let name_tok = self.expect(TokenKind::Ident(String::new()))?;
                let name = match &name_tok.value {
                    TokenKind::Ident(s) => s.clone(),
                    _ => String::new(),
                };
                if self.at(&TokenKind::LParen) {
                    self.bump();
                    let mut args = Vec::new();
                    while !self.at(&TokenKind::RParen) {
                        if !args.is_empty() {
                            self.expect(TokenKind::Comma)?;
                        }
                        args.push(self.parse_expr()?);
                    }
                    self.expect(TokenKind::RParen)?;
                    expr = Expr::Method { obj: Box::new(expr), name, args };
                } else {
                    expr = Expr::Dot { obj: Box::new(expr), name };
                }
            } else if self.at(&TokenKind::LBracket) {
                self.bump();
                // Проверяем slice: obj[start:end] или obj[start:end:step]
                if self.at(&TokenKind::RBracket) {
                    self.bump();
                    expr = Expr::Index { obj: Box::new(expr), idx: Box::new(Expr::None) };
                } else if self.at(&TokenKind::Colon) {
                    // Slice без start: obj[:end] или obj[:]
                    self.bump();
                    let end = if self.at(&TokenKind::RBracket) || self.at(&TokenKind::Colon) {
                        None
                    } else {
                        Some(Box::new(self.parse_expr()?))
                    };
                    let step = if self.at(&TokenKind::Colon) {
                        self.bump();
                        if self.at(&TokenKind::RBracket) {
                            None
                        } else {
                            Some(Box::new(self.parse_expr()?))
                        }
                    } else {
                        None
                    };
                    self.expect(TokenKind::RBracket)?;
                    expr = Expr::Slice {
                        obj: Box::new(expr),
                        start: None,
                        end,
                        step,
                    };
                } else {
                    let first = self.parse_expr()?;
                    if self.at(&TokenKind::Colon) {
                        // Slice: obj[start:end] или obj[start:end:step]
                        self.bump();
                        let end = if self.at(&TokenKind::RBracket) || self.at(&TokenKind::Colon) {
                            None
                        } else {
                            Some(Box::new(self.parse_expr()?))
                        };
                        let step = if self.at(&TokenKind::Colon) {
                            self.bump();
                            if self.at(&TokenKind::RBracket) {
                                None
                            } else {
                                Some(Box::new(self.parse_expr()?))
                            }
                        } else {
                            None
                        };
                        self.expect(TokenKind::RBracket)?;
                        expr = Expr::Slice {
                            obj: Box::new(expr),
                            start: Some(Box::new(first)),
                            end,
                            step,
                        };
                    } else {
                        // Index: obj[idx]
                        self.expect(TokenKind::RBracket)?;
                        expr = Expr::Index { obj: Box::new(expr), idx: Box::new(first) };
                    }
                }
            } else {
                break;
            }
        }
        
        Ok(expr)
    }

    fn parse_atom(&mut self) -> Result<Expr, ParseError> {
        let token = self.current.clone().ok_or(ParseError::InvalidSyntax { span: Span::default() })?;
        
        match &token.value {
            TokenKind::True => {
                self.bump();
                Ok(Expr::Bool(true))
            }
            TokenKind::False => {
                self.bump();
                Ok(Expr::Bool(false))
            }
            TokenKind::Int(n) => {
                self.bump();
                Ok(Expr::Int(*n))
            }
            TokenKind::Float(n) => {
                self.bump();
                Ok(Expr::Float(*n))
            }
            TokenKind::Str(s) => {
                self.bump();
                Ok(Expr::Str(s.clone()))
            }
            TokenKind::Char(c) => {
                self.bump();
                Ok(Expr::Char(*c))
            }
            TokenKind::LParen => {
                self.bump();
                if self.at(&TokenKind::RParen) {
                    self.bump();
                    return Ok(Expr::Tuple(Vec::new()));
                }
                let expr = self.parse_expr()?;
                if self.at(&TokenKind::Comma) {
                    self.bump();
                    let mut elems = vec![expr];
                    while !self.at(&TokenKind::RParen) {
                        self.expect(TokenKind::Comma)?;
                        elems.push(self.parse_expr()?);
                    }
                    self.expect(TokenKind::RParen)?;
                    Ok(Expr::Tuple(elems))
                } else {
                    self.expect(TokenKind::RParen)?;
                    Ok(expr)
                }
            }
            TokenKind::LBracket => {
                self.bump();
                if self.at(&TokenKind::RBracket) {
                    self.bump();
                    return Ok(Expr::List(Vec::new()));
                }
                let first = self.parse_expr()?;
                // Проверяем list comprehension: [body for target in iter]
                if self.at(&TokenKind::For) {
                    self.bump();
                    let target_tok = self.expect(TokenKind::Ident(String::new()))?;
                    let target = match &target_tok.value {
                        TokenKind::Ident(s) => s.clone(),
                        _ => return Err(ParseError::ExpectedIdent { found: target_tok.value.clone(), span: target_tok.span }),
                    };
                    self.expect(TokenKind::In)?;
                    let iter = self.parse_expr()?;
                    let cond = if self.at(&TokenKind::If) {
                        self.bump();
                        Some(self.parse_expr()?)
                    } else {
                        None
                    };
                    let mut generators = vec![CompGen { target, iter, cond }];
                    // Дополнительные for-генераторы
                    while self.at(&TokenKind::For) {
                        self.bump();
                        let t2 = self.expect(TokenKind::Ident(String::new()))?;
                        let target2 = match &t2.value {
                            TokenKind::Ident(s) => s.clone(),
                            _ => return Err(ParseError::ExpectedIdent { found: t2.value.clone(), span: t2.span }),
                        };
                        self.expect(TokenKind::In)?;
                        let iter2 = self.parse_expr()?;
                        let cond2 = if self.at(&TokenKind::If) {
                            self.bump();
                            Some(self.parse_expr()?)
                        } else {
                            None
                        };
                        generators.push(CompGen { target: target2, iter: iter2, cond: cond2 });
                    }
                    self.expect(TokenKind::RBracket)?;
                    return Ok(Expr::ListComp {
                        body: Box::new(first),
                        generators,
                    });
                }
                // Обычный list literal
                let mut elems = vec![first];
                while self.at(&TokenKind::Comma) {
                    self.bump();
                    if self.at(&TokenKind::RBracket) { break; }
                    elems.push(self.parse_expr()?);
                }
                self.expect(TokenKind::RBracket)?;
                Ok(Expr::List(elems))
            }
            TokenKind::LBrace => {
                self.bump();
                if self.at(&TokenKind::RBrace) {
                    self.bump();
                    return Ok(Expr::Dict(Vec::new()));
                }
                let first_key = self.parse_expr()?;
                self.expect(TokenKind::Colon)?;
                let first_val = self.parse_expr()?;
                // Проверяем dict comprehension: {key: val for target in iter}
                if self.at(&TokenKind::For) {
                    self.bump();
                    let target_tok = self.expect(TokenKind::Ident(String::new()))?;
                    let target = match &target_tok.value {
                        TokenKind::Ident(s) => s.clone(),
                        _ => return Err(ParseError::ExpectedIdent { found: target_tok.value.clone(), span: target_tok.span }),
                    };
                    self.expect(TokenKind::In)?;
                    let iter = self.parse_expr()?;
                    let cond = if self.at(&TokenKind::If) {
                        self.bump();
                        Some(self.parse_expr()?)
                    } else {
                        None
                    };
                    let mut generators = vec![CompGen { target, iter, cond }];
                    while self.at(&TokenKind::For) {
                        self.bump();
                        let t2 = self.expect(TokenKind::Ident(String::new()))?;
                        let target2 = match &t2.value {
                            TokenKind::Ident(s) => s.clone(),
                            _ => return Err(ParseError::ExpectedIdent { found: t2.value.clone(), span: t2.span }),
                        };
                        self.expect(TokenKind::In)?;
                        let iter2 = self.parse_expr()?;
                        let cond2 = if self.at(&TokenKind::If) {
                            self.bump();
                            Some(self.parse_expr()?)
                        } else {
                            None
                        };
                        generators.push(CompGen { target: target2, iter: iter2, cond: cond2 });
                    }
                    self.expect(TokenKind::RBrace)?;
                    return Ok(Expr::DictComp {
                        key: Box::new(first_key),
                        val: Box::new(first_val),
                        generators,
                    });
                }
                // Обычный dict literal
                let mut items = vec![(first_key, first_val)];
                while self.at(&TokenKind::Comma) {
                    self.bump();
                    if self.at(&TokenKind::RBrace) { break; }
                    let key = self.parse_expr()?;
                    self.expect(TokenKind::Colon)?;
                    let val = self.parse_expr()?;
                    items.push((key, val));
                }
                self.expect(TokenKind::RBrace)?;
                Ok(Expr::Dict(items))
            }
            TokenKind::Ident(name) => {
                self.bump();
                Ok(Expr::Ident(name.clone()))
            }
            TokenKind::None => {
                self.bump();
                Ok(Expr::None)
            }
            TokenKind::Lambda => self.parse_lambda(),
            _ => Err(ParseError::UnexpectedToken {
                expected: "expression",
                found: token.value.clone(),
                span: token.span,
            }),
        }
}
        
    fn parse_lambda(&mut self) -> Result<Expr, ParseError> {
        self.bump();
        
        let mut params = Vec::new();
        
        if !self.at(&TokenKind::Colon) {
            loop {
                let name_tok = self.expect(TokenKind::Ident(String::new()))?;
                let name = match &name_tok.value {
                    TokenKind::Ident(s) => s.clone(),
                    _ => return Err(ParseError::UnexpectedToken {
                        expected: "parameter name",
                        found: name_tok.value.clone(),
                        span: name_tok.span,
                    }),
                };
                
                let ty = if self.at(&TokenKind::Colon) {
                    self.bump();
                    Some(self.parse_type()?)
                } else {
                    None
                };
                
                params.push(Param {
                    name,
                    ty: ty.unwrap_or(Type::I64),
                    default: None,
                });
                
                if self.at(&TokenKind::Comma) {
                    self.bump();
                    continue;
                }
                break;
            }
        }
        
        self.expect(TokenKind::Colon)?;
        
        let body = Box::new(self.parse_expr()?);
        
        Ok(Expr::Lambda { params, body })
    }
    
    // Stub implementations for remaining statement types
    fn parse_class(&mut self) -> Result<Stmt, ParseError> {
        self.bump();
        
        let name_tok = self.expect(TokenKind::Ident(String::new()))?;
        let name = match &name_tok.value {
            TokenKind::Ident(s) => s.clone(),
            _ => return Err(ParseError::UnexpectedToken { expected: "class name", found: name_tok.value.clone(), span: name_tok.span }),
        };
        
        let mut bases = Vec::new();
        if self.at(&TokenKind::LParen) {
            self.bump();
            while !self.at(&TokenKind::RParen) {
                if !bases.is_empty() {
                    self.expect(TokenKind::Comma)?;
                }
                bases.push(self.parse_type()?);
            }
            self.expect(TokenKind::RParen)?;
        }
        
        self.expect(TokenKind::Colon)?;
        self.skip_newline()?;
        
        let body = self.parse_suite()?;
        
        Ok(Stmt::Class(Class { name, bases, body }))
    }
    
    fn parse_struct(&mut self) -> Result<Stmt, ParseError> {
        self.bump();
        
        let name_tok = self.expect(TokenKind::Ident(String::new()))?;
        let name = match &name_tok.value {
            TokenKind::Ident(s) => s.clone(),
            _ => return Err(ParseError::UnexpectedToken { expected: "struct name", found: name_tok.value.clone(), span: name_tok.span }),
        };
        
        self.expect(TokenKind::Colon)?;
        self.skip_newline()?;
        
        let mut fields = Vec::new();
        while self.at(&TokenKind::Ident(String::new())) {
            let field_name_tok = self.expect(TokenKind::Ident(String::new()))?;
            let field_name = match &field_name_tok.value {
                TokenKind::Ident(s) => s.clone(),
                _ => return Err(ParseError::UnexpectedToken { expected: "field name", found: field_name_tok.value.clone(), span: field_name_tok.span }),
            };
            self.expect(TokenKind::Colon)?;
            let field_ty = self.parse_type()?;
            fields.push((field_name, field_ty));
            self.skip_newline()?;
        }
        
        Ok(Stmt::Struct(Struct { name, fields }))
    }
    fn parse_if(&mut self) -> Result<Stmt, ParseError> {
        self.bump();
        
        let cond = self.parse_expr()?;
        self.expect(TokenKind::Colon)?;
        self.skip_newline()?;
        
        let then = self.parse_suite()?;
        
        let mut elif = Vec::new();
        while self.at(&TokenKind::Elif) {
            self.bump();
            
            let elif_cond = self.parse_expr()?;
            self.expect(TokenKind::Colon)?;
            self.skip_newline()?;
            
            let elif_body = self.parse_suite()?;
            
            elif.push(Elif {
                cond: elif_cond,
                body: elif_body,
            });
        }
        
        let else_ = if self.at(&TokenKind::Else) {
            self.bump();
            self.expect(TokenKind::Colon)?;
            self.skip_newline()?;
            Some(self.parse_suite()?)
        } else {
            None
        };
        
        Ok(Stmt::If(If { cond, then, elif, else_ }))
    }
    fn parse_while(&mut self) -> Result<Stmt, ParseError> {
        self.bump();
        
        let cond = self.parse_expr()?;
        self.expect(TokenKind::Colon)?;
        self.skip_newline()?;
        
        let body = self.parse_suite()?;
        
        Ok(Stmt::While(While { cond, body }))
    }
    fn parse_for(&mut self) -> Result<Stmt, ParseError> {
        self.bump();
        
        let target_tok = self.expect(TokenKind::Ident(String::new()))?;
        let target = match &target_tok.value {
            TokenKind::Ident(s) => s.clone(),
            _ => return Err(ParseError::ExpectedIdent { found: target_tok.value.clone(), span: target_tok.span }),
        };
        
        self.expect(TokenKind::In)?;
        
        let iter = self.parse_expr()?;
        self.expect(TokenKind::Colon)?;
        self.skip_newline()?;
        
        let body = self.parse_suite()?;
        
        Ok(Stmt::For(For { target, iter, body }))
    }
    fn parse_loop(&mut self) -> Result<Stmt, ParseError> {
        self.bump();
        
        self.expect(TokenKind::Colon)?;
        self.skip_newline()?;
        
        let body = self.parse_suite()?;
        
        Ok(Stmt::Loop(Loop { body }))
    }
    fn parse_match(&mut self) -> Result<Stmt, ParseError> {
        self.bump();
        
        let expr = self.parse_expr()?;
        self.expect(TokenKind::Colon)?;
        self.skip_newline()?;
        
        let mut arms = Vec::new();
        while let Some(ref t) = self.current {
            if t.value == TokenKind::Eof {
                break;
            }
            if t.value == TokenKind::Newline {
                self.bump();
                continue;
            }
            let pat = self.parse_pattern()?;
            
            let guard = if self.at(&TokenKind::If) {
                self.bump();
                Some(self.parse_expr()?)
            } else {
                None
            };
            
            self.expect(TokenKind::FatArrow)?;
            self.skip_newline()?;
            
            let body = self.parse_suite()?;
            
            arms.push(MatchArm { pat, guard, body });
        }
        
        Ok(Stmt::Match(Match { expr, arms }))
    }
    
    fn parse_pattern(&mut self) -> Result<Pattern, ParseError> {
        let span = self.current.as_ref().map(|t| t.span).unwrap_or_default();
        
        match &self.current {
            Some(tok) => match &tok.value {
                TokenKind::Underscore => {
                    self.bump();
                    Ok(Pattern::Wildcard)
                }
                TokenKind::LParen => {
                    self.bump();
                    let mut patterns = Vec::new();
                    while !self.at(&TokenKind::RParen) && !self.at(&TokenKind::Eof) {
                        patterns.push(self.parse_pattern()?);
                        if !self.at(&TokenKind::RParen) {
                            self.expect(TokenKind::Comma)?;
                        }
                    }
                    self.expect(TokenKind::RParen)?;
                    Ok(Pattern::Tuple(patterns))
                }
                TokenKind::Int(n) => {
                    let val = *n;
                    self.bump();
                    Ok(Pattern::Literal(Literal::Int(val)))
                }
                TokenKind::Float(f) => {
                    let val = *f;
                    self.bump();
                    Ok(Pattern::Literal(Literal::Float(val)))
                }
                TokenKind::Str(s) => {
                    let val = s.clone();
                    self.bump();
                    Ok(Pattern::Literal(Literal::Str(val)))
                }
                TokenKind::Ident(_) => {
                    let name_tok = self.bump().unwrap();
                    match &name_tok.value {
                        TokenKind::Ident(s) => Ok(Pattern::Binding(s.clone())),
                        _ => Err(ParseError::UnexpectedToken {
                            expected: "identifier",
                            found: name_tok.value.clone(),
                            span,
                        }),
                    }
                }
                _ => Err(ParseError::UnexpectedToken {
                    expected: "pattern",
                    found: tok.value.clone(),
                    span,
                }),
            },
            None => Err(ParseError::UnexpectedToken {
                expected: "pattern",
                found: TokenKind::Eof,
                span,
            }),
        }
    }
    fn parse_try(&mut self) -> Result<Stmt, ParseError> {
        self.bump();
        self.expect(TokenKind::Colon)?;
        self.skip_newline()?;
        
        let body = self.parse_suite()?;
        
        let mut handlers = Vec::new();
        while self.at(&TokenKind::Except) {
            self.bump();
            
            let exc = if !self.at(&TokenKind::Colon) && !self.at(&TokenKind::As) {
                Some(self.parse_type()?)
            } else {
                None
            };
            
            let binding = if self.at(&TokenKind::As) {
                self.bump();
                let name_tok = self.expect(TokenKind::Ident(String::new()))?;
                match &name_tok.value {
                    TokenKind::Ident(s) => Some(s.clone()),
                    _ => None,
                }
            } else {
                None
            };
            
            self.expect(TokenKind::Colon)?;
            self.skip_newline()?;
            
            let handler_body = self.parse_suite()?;
            
            handlers.push(Handler {
                exc,
                binding,
                body: handler_body,
            });
        }
        
        let finally = if self.at(&TokenKind::Finally) {
            self.bump();
            self.expect(TokenKind::Colon)?;
            self.skip_newline()?;
            Some(self.parse_suite()?)
        } else {
            None
        };
        
        Ok(Stmt::Try(Try {
            body,
            handlers,
            finally,
        }))
    }
    fn parse_with(&mut self) -> Result<Stmt, ParseError> {
        self.bump();
        
        let mut items = Vec::new();
        
        loop {
            let expr = self.parse_expr()?;
            
            let as_ = if self.at(&TokenKind::As) {
                self.bump();
                let name_tok = self.expect(TokenKind::Ident(String::new()))?;
                match &name_tok.value {
                    TokenKind::Ident(s) => Some(s.clone()),
                    _ => None,
                }
            } else {
                None
            };
            
            items.push(WithItem { expr, as_ });
            
            if self.at(&TokenKind::Comma) {
                self.bump();
                continue;
            }
            break;
        }
        
        self.expect(TokenKind::Colon)?;
        self.skip_newline()?;
        
        let body = self.parse_suite()?;
        
        Ok(Stmt::With(With { items, body }))
    }
    fn parse_yield(&mut self) -> Result<Stmt, ParseError> {
        self.bump();
        
        let val = if self.at(&TokenKind::Newline) || self.at(&TokenKind::Colon) {
            None
        } else {
            Some(self.parse_expr()?)
        };
        
        Ok(Stmt::Yield(Yield { val }))
    }
    fn parse_raise(&mut self) -> Result<Stmt, ParseError> {
        self.bump();
        
        let exc = self.parse_expr()?;
        
        Ok(Stmt::Raise(Raise { exc }))
    }
    fn parse_assert(&mut self) -> Result<Stmt, ParseError> {
        self.bump();
        
        let cond = self.parse_expr()?;
        
        let msg = if self.at(&TokenKind::Comma) {
            self.bump();
            Some(self.parse_expr()?)
        } else {
            None
        };
        
        Ok(Stmt::Assert(Assert { cond, msg }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_int() {
        let mut parser = Parser::new("42");
        let mut sema = Sema::new();
        let result = parser.parse(&mut sema);
        assert!(result.is_ok());
        let ast = result.unwrap();
        assert_eq!(ast.len(), 1);
    }

    #[test]
    fn test_parse_function() {
        let code = "def main() -> int:\n    return 0";
        let mut parser = Parser::new(code);
        let mut sema = Sema::new();
        let result = parser.parse(&mut sema);
        assert!(result.is_ok());
        let ast = result.unwrap();
        assert_eq!(ast.len(), 1);
    }

    #[test]
    fn test_parse_lambda() {
        let code = "lambda x: int, y: int: x + y";
        let mut parser = Parser::new(code);
        let mut sema = Sema::new();
        let result = parser.parse(&mut sema);
        assert!(result.is_ok());
        let ast = result.unwrap();
        assert_eq!(ast.len(), 1);
    }

    #[test]
    fn test_parse_let() {
        let code = "let x: int = 42";
        let mut parser = Parser::new(code);
        let mut sema = Sema::new();
        let result = parser.parse(&mut sema);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_binary_op() {
        let code = "1 + 2";
        let mut parser = Parser::new(code);
        let mut sema = Sema::new();
        let result = parser.parse(&mut sema);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_parse_struct() {
        let code = "struct Point:\n    x: int\n    y: int";
        let mut parser = Parser::new(code);
        let mut sema = Sema::new();
        let result = parser.parse(&mut sema);
        assert!(result.is_ok());
        let ast = result.unwrap();
        assert_eq!(ast.len(), 1);
    }

    #[test]
    fn test_parse_if() {
        let code = "if true:\n    let x: int = 1";
        let mut parser = Parser::new(code);
        let mut sema = Sema::new();
        let result = parser.parse(&mut sema);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_parse_class() {
        let code = "class Foo:\n    let x: int = 1";
        let mut parser = Parser::new(code);
        let mut sema = Sema::new();
        let result = parser.parse(&mut sema);
        assert!(result.is_ok());
        let ast = result.unwrap();
        assert_eq!(ast.len(), 1);
    }
    
    #[test]
    fn test_parse_try() {
        let code = "try:\n    pass\nexcept:\n    pass";
        let mut parser = Parser::new(code);
        let mut sema = Sema::new();
        let result = parser.parse(&mut sema);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_parse_raise() {
        let code = "raise Exception()";
        let mut parser = Parser::new(code);
        let mut sema = Sema::new();
        let result = parser.parse(&mut sema);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_list_literal() {
        let code = "let x: int = [1, 2, 3]";
        let mut parser = Parser::new(code);
        let mut sema = Sema::new();
        let result = parser.parse(&mut sema);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_dict_literal() {
        let code = "let x: int = {1: 2, 3: 4}";
        let mut parser = Parser::new(code);
        let mut sema = Sema::new();
        let result = parser.parse(&mut sema);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_listcomp() {
        let code = "let x: int = [i for i in items]";
        let mut parser = Parser::new(code);
        let mut sema = Sema::new();
        let result = parser.parse(&mut sema);
        assert!(result.is_ok(), "ListComp parse failed: {:?}", result.err());
    }

    #[test]
    fn test_parse_listcomp_with_cond() {
        let code = "let x: int = [i for i in items if i > 0]";
        let mut parser = Parser::new(code);
        let mut sema = Sema::new();
        let result = parser.parse(&mut sema);
        assert!(result.is_ok(), "ListComp with cond parse failed: {:?}", result.err());
    }

    #[test]
    fn test_parse_dictcomp() {
        let code = "let x: int = {i: i * 2 for i in items}";
        let mut parser = Parser::new(code);
        let mut sema = Sema::new();
        let result = parser.parse(&mut sema);
        assert!(result.is_ok(), "DictComp parse failed: {:?}", result.err());
    }

    #[test]
    fn test_parse_slice() {
        let code = "let x: int = arr[1:3]";
        let mut parser = Parser::new(code);
        let mut sema = Sema::new();
        let result = parser.parse(&mut sema);
        assert!(result.is_ok(), "Slice parse failed: {:?}", result.err());
    }

    #[test]
    fn test_parse_slice_full() {
        let code = "let x: int = arr[0:10:2]";
        let mut parser = Parser::new(code);
        let mut sema = Sema::new();
        let result = parser.parse(&mut sema);
        assert!(result.is_ok(), "Slice full parse failed: {:?}", result.err());
    }

    #[test]
    fn test_parse_index() {
        let code = "let x: int = arr[0]";
        let mut parser = Parser::new(code);
        let mut sema = Sema::new();
        let result = parser.parse(&mut sema);
        assert!(result.is_ok(), "Index parse failed: {:?}", result.err());
    }

    #[test]
    fn test_parse_decorator_simple() {
        let code = "@dec\ndef foo():\n    pass";
        let mut parser = Parser::new(code);
        let mut sema = Sema::new();
        let result = parser.parse(&mut sema);
        assert!(result.is_ok(), "Decorator parse failed: {:?}", result.err());
        let ast = result.unwrap();
        assert_eq!(ast.len(), 1);
    }

    #[test]
    fn test_parse_decorator_chain() {
        let code = "@dec1\n@dec2\ndef foo():\n    pass";
        let mut parser = Parser::new(code);
        let mut sema = Sema::new();
        let result = parser.parse(&mut sema);
        assert!(result.is_ok(), "Decorator chain parse failed: {:?}", result.err());
        let ast = result.unwrap();
        assert_eq!(ast.len(), 1);
    }

    #[test]
    fn test_parse_decorator_call() {
        let code = "@dec(arg)\ndef foo():\n    pass";
        let mut parser = Parser::new(code);
        let mut sema = Sema::new();
        let result = parser.parse(&mut sema);
        assert!(result.is_ok(), "Decorator call parse failed: {:?}", result.err());
    }

    #[test]
    fn test_parse_assignment() {
        let code = "x = 42";
        let mut parser = Parser::new(code);
        let mut sema = Sema::new();
        let result = parser.parse(&mut sema);
        assert!(result.is_ok(), "Assignment parse failed: {:?}", result.err());
    }
}
