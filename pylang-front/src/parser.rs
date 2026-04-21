//! PEG Parser для Pylang.
//!
//! Recursive descent. Парсит type-annotated Python subset.

use crate::ast::*;
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
}

impl<'src> Parser<'src> {
    pub fn new(source: &'src str) -> Self {
        let mut lexer = Lexer::new(source);
        let current = lexer.next();
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
        self.current = self.lexer.next();
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
            _ => {
                let expr = self.parse_expr()?;
                Ok(Stmt::Expr(expr))
            }
        }
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
        
        Ok(Stmt::Fn(Fn { name, params, ret, body }))
    }

    fn parse_param(&mut self) -> Result<Param, ParseError> {
        let name_tok = self.expect(TokenKind::Ident(String::new()))?;
        let name = match &name_tok.value {
            TokenKind::Ident(s) => s.clone(),
            _ => return Err(ParseError::UnexpectedToken { expected: "parameter name", found: name_tok.value.clone(), span: name_tok.span }),
        };
        
        self.expect(TokenKind::Colon)?;
        let ty = self.parse_type()?;
        
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
                } else if name == "str" {
                    Type::Named("str".to_string())
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
        
        while let Some(ref t) = self.current {
            match &t.value {
                TokenKind::Eof => break,
                TokenKind::Def | TokenKind::Class | TokenKind::Struct | TokenKind::If 
                | TokenKind::While | TokenKind::For | TokenKind::Loop | TokenKind::Match 
                | TokenKind::Try | TokenKind::With => break,
                TokenKind::Newline => {
                    self.bump();
                    continue;
                }
                TokenKind::Return => {
                    stmts.push(self.parse_stmt()?);
                }
                TokenKind::Let => {
                    stmts.push(self.parse_stmt()?);
                }
                _ => {
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
        if self.at(&TokenKind::Newline) {
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
                expr = Expr::Dot { obj: Box::new(expr), name };
            } else if self.at(&TokenKind::LBracket) {
                self.bump();
                let idx = self.parse_expr()?;
                self.expect(TokenKind::RBracket)?;
                expr = Expr::Index { obj: Box::new(expr), idx: Box::new(idx) };
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
                let mut elems = Vec::new();
                while !self.at(&TokenKind::RBracket) {
                    if !elems.is_empty() {
                        self.expect(TokenKind::Comma)?;
                    }
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
                let mut items = Vec::new();
                while !self.at(&TokenKind::RBrace) {
                    if !items.is_empty() {
                        self.expect(TokenKind::Comma)?;
                    }
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
            _ => Err(ParseError::UnexpectedToken {
                expected: "expression",
                found: token.value.clone(),
                span: token.span,
            }),
        }
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
    
    fn parse_struct(&mut self) -> Result<Stmt, ParseError> { Ok(Stmt::Pass) }
    fn parse_if(&mut self) -> Result<Stmt, ParseError> { Ok(Stmt::Pass) }
    fn parse_while(&mut self) -> Result<Stmt, ParseError> { Ok(Stmt::Pass) }
    fn parse_for(&mut self) -> Result<Stmt, ParseError> { Ok(Stmt::Pass) }
    fn parse_loop(&mut self) -> Result<Stmt, ParseError> { Ok(Stmt::Pass) }
    fn parse_match(&mut self) -> Result<Stmt, ParseError> { Ok(Stmt::Pass) }
    fn parse_try(&mut self) -> Result<Stmt, ParseError> { Ok(Stmt::Pass) }
    fn parse_with(&mut self) -> Result<Stmt, ParseError> { Ok(Stmt::Pass) }
    fn parse_yield(&mut self) -> Result<Stmt, ParseError> { Ok(Stmt::Pass) }
    fn parse_raise(&mut self) -> Result<Stmt, ParseError> { Ok(Stmt::Pass) }
    fn parse_assert(&mut self) -> Result<Stmt, ParseError> { Ok(Stmt::Pass) }
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
    fn test_parse_if() {
        let code = "if true:\n    let x: int = 1";
        let mut parser = Parser::new(code);
        let mut sema = Sema::new();
        let result = parser.parse(&mut sema);
        assert!(result.is_ok());
    }
}
