//! Lexer — tokenizer для Pylang.

#[derive(Clone, Debug, PartialEq)]
pub enum TokenKind {
    // Literals
    True,
    False,
    Int(i64),
    Float(f64),
    Char(char),
    Str(String),
    Bytes(Vec<u8>),

    // Identifiers
    Ident(String),

    // Keywords
    Def,
    Class,
    Struct,
    If,
    Else,
    Elif,
    Match,
    Case,
    For,
    While,
    Loop,
    In,
    As,
    Return,
    Yield,
    Raise,
    From,
    Import,
    Try,
    Except,
    Finally,
    With,
    Pass,
    Break,
    Continue,
    Assert,
    Type,
    Trait,
    Impl,
    Let,
    Mut,
    Pub,
    Static,
    Async,
    Await,
    Lock,
    Spawn,
    GetRef,
    Release,
    Lambda,

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Ampersand,
    Pipe,
    Tilde,
    Caret,
    Colon,
    Dot,
    DotDot,
    Comma,
    Semicolon,
    At,
    Hash,
    Dollar,
    Bang,

    // Underscore
    Underscore,
    
    // Special
    None,
    Eof,

    // Comparison
    EqEq,
    Ne,
    Le,
    Ge,
    Lt,
    Gt,

    // Assignment
    Eq,
    PlusEq,
    MinusEq,
    StarEq,
    SlashEq,
    PercentEq,
    AmpersandEq,
    PipeEq,
    CaretEq,
    LtLt,
    GtGt,

    // Logical
    AndAnd,
    OrOr,
    BangEq,

    // Delimiters
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,

    // Type annotations
    RArrow,
    FatArrow,
    QMark,
    ColonColon,

    // Indentation signal
    Newline,
    Indent,
    Dedent,

    // F-strings
    FString(Vec<FStringSeg>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum FStringSeg {
    Lit(String),
    Expr(String),
}

#[derive(Clone, Debug)]
pub struct Spanned<T> {
    pub span: Span,
    pub value: T,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Span {
    pub start: Pos,
    pub end: Pos,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Pos {
    pub offset: usize,
    pub line: usize,
    pub col: usize,
}

pub type Token = Spanned<TokenKind>;

#[derive(Clone, Debug)]
pub struct LexerErrors {
    pub errors: Vec<LexError>,
}

#[derive(Clone, Debug)]
pub enum LexError {
    UnterminatedString { start: Pos },
    InvalidEscape { pos: Pos },
    InvalidSuffix { pos: Pos },
    UnterminatedFString { start: Pos },
}

impl LexerErrors {
    pub fn new() -> Self {
        Self { errors: Vec::new() }
    }
}

impl Default for LexerErrors {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Lexer<'src> {
    source: &'src str,
    offset: usize,
    line: usize,
    col: usize,
#[allow(dead_code)]
    errors: Vec<LexError>,
}

impl<'src> Lexer<'src> {
    pub fn new(source: &'src str) -> Self {
        Self {
            source,
            offset: 0,
            line: 1,
            col: 1,
            errors: Vec::new(),
        }
    }

    pub fn next_token(&mut self) -> Option<Spanned<TokenKind>> {
        if self.offset >= self.source.len() {
            return None;
        }
        self.skip_whitespace()?;
        
        if self.offset >= self.source.len() {
            return None;
        }
        
        let start = self.pos();
        let c = self.peek()?;
        
        let (kind, _consumed) = if c.is_ascii_digit() {
            self.read_number()?
        } else if c.is_alphabetic() || c == '_' {
            self.read_ident()?
        } else {
            self.read_punct()?
        };
        
        let end = self.pos();
        Some(Spanned {
            span: Span { start, end },
            value: kind,
        })
    }

    fn pos(&self) -> Pos {
        Pos { offset: self.offset, line: self.line, col: self.col }
    }

    fn peek(&self) -> Option<char> {
        self.source[self.offset..].chars().next()
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.source[self.offset..].chars().next()?;
        let len = c.len_utf8();
        self.offset += len;
        if c == '\n' {
            self.line += 1;
            self.col = 1;
        } else {
            self.col += 1;
        }
        Some(c)
    }

    fn skip_whitespace(&mut self) -> Option<()> {
        while let Some(c) = self.peek() {
            if c.is_whitespace() && c != '\n' {
                self.advance();
            } else {
                break;
            }
        }
        Some(())
    }

    fn read_number(&mut self) -> Option<(TokenKind, usize)> {
        let start = self.offset;
        let mut has_dot = false;
        let mut has_exp = false;
        
        while let Some(c) = self.peek() {
            if c.is_ascii_digit() {
                self.advance();
            } else if c == '.' && !has_dot && !has_exp {
                has_dot = true;
                self.advance();
            } else if (c == 'e' || c == 'E') && !has_exp {
                has_exp = true;
                self.advance();
                if let Some(sign) = self.peek() {
                    if sign == '+' || sign == '-' {
                        self.advance();
                    }
                }
            } else {
                break;
            }
        }
        
        let num_str = &self.source[start..self.offset];
        
        if has_dot || has_exp {
            let n: f64 = num_str.parse().ok()?;
            Some((TokenKind::Float(n), self.offset - start))
        } else {
            let n: i64 = num_str.parse().ok()?;
            Some((TokenKind::Int(n), self.offset - start))
        }
    }

    fn read_ident(&mut self) -> Option<(TokenKind, usize)> {
        let start = self.offset;
        
        while let Some(c) = self.peek() {
            if c.is_alphanumeric() || c == '_' {
                self.advance();
            } else {
                break;
            }
        }
        
        let ident = &self.source[start..self.offset].to_string();
        
        // Check for f-string: "f" or "F" followed by " or '
        if (ident == "f" || ident == "F") && self.peek().map(|c| c == '"' || c == '\'').unwrap_or(false) {
            return self.read_fstring();
        }
        
        let kind = match ident.as_str() {
            "def" => TokenKind::Def,
            "class" => TokenKind::Class,
            "struct" => TokenKind::Struct,
            "if" => TokenKind::If,
            "elif" => TokenKind::Elif,
            "else" => TokenKind::Else,
            "match" => TokenKind::Match,
            "case" => TokenKind::Case,
            "for" => TokenKind::For,
            "while" => TokenKind::While,
            "loop" => TokenKind::Loop,
            "in" => TokenKind::In,
            "as" => TokenKind::As,
            "return" => TokenKind::Return,
            "yield" => TokenKind::Yield,
            "raise" => TokenKind::Raise,
            "lambda" => TokenKind::Lambda,
            "from" => TokenKind::From,
            "import" => TokenKind::Import,
            "try" => TokenKind::Try,
            "except" => TokenKind::Except,
            "finally" => TokenKind::Finally,
            "with" => TokenKind::With,
            "pass" => TokenKind::Pass,
            "break" => TokenKind::Break,
            "continue" => TokenKind::Continue,
            "assert" => TokenKind::Assert,
            "let" => TokenKind::Let,
            "mut" => TokenKind::Mut,
            "pub" => TokenKind::Pub,
            "true" => TokenKind::True,
            "false" => TokenKind::False,
            "none" => TokenKind::None,
            "and" => TokenKind::AndAnd,
            "or" => TokenKind::OrOr,
            "not" => TokenKind::Bang,
            _ => TokenKind::Ident(ident.clone()),
        };
        
        Some((kind, self.offset - start))
    }

    fn read_punct(&mut self) -> Option<(TokenKind, usize)> {
        let c = self.advance()?;
        
        let kind = match c {
            '(' => TokenKind::LParen,
            ')' => TokenKind::RParen,
            '[' => TokenKind::LBracket,
            ']' => TokenKind::RBracket,
            '{' => TokenKind::LBrace,
            '}' => TokenKind::RBrace,
            ',' => TokenKind::Comma,
            ';' => TokenKind::Semicolon,
            ':' => TokenKind::Colon,
            '+' => TokenKind::Plus,
            '-' => {
                if self.peek() == Some('>') {
                    self.advance();
                    return Some((TokenKind::RArrow, 2));
                }
                TokenKind::Minus
            }
            '*' => TokenKind::Star,
            '/' => TokenKind::Slash,
            '%' => TokenKind::Percent,
            '&' => {
                if self.peek() == Some('&') {
                    self.advance();
                    return Some((TokenKind::AndAnd, 2));
                }
                TokenKind::Ampersand
            }
            '|' => {
                if self.peek() == Some('|') {
                    self.advance();
                    return Some((TokenKind::OrOr, 2));
                }
                TokenKind::Pipe
            }
            '^' => TokenKind::Caret,
            '~' => TokenKind::Tilde,
            '=' => {
                if self.peek() == Some('=') {
                    self.advance();
                    return Some((TokenKind::EqEq, 2));
                }
                TokenKind::Eq
            }
            '!' => {
                if self.peek() == Some('=') {
                    self.advance();
                    return Some((TokenKind::Ne, 2));
                }
                TokenKind::Bang
            }
            '<' => {
                if self.peek() == Some('<') {
                    self.advance();
                    return Some((TokenKind::LtLt, 2));
                }
                if self.peek() == Some('=') {
                    self.advance();
                    return Some((TokenKind::Le, 2));
                }
                TokenKind::Lt
            }
            '>' => {
                if self.peek() == Some('>') {
                    self.advance();
                    return Some((TokenKind::GtGt, 2));
                }
                if self.peek() == Some('=') {
                    self.advance();
                    return Some((TokenKind::Ge, 2));
                }
                TokenKind::Gt
            }
            '.' => TokenKind::Dot,
            '@' => TokenKind::At,
            '_' => TokenKind::Underscore,
            '"' => return self.read_string(),
            '\'' => return self.read_char(),
            '\n' => {
                return Some((TokenKind::Newline, 1));
            }
            _ => return None,
        };
        
        Some((kind, 1))
    }

    fn read_string(&mut self) -> Option<(TokenKind, usize)> {
        let start = self.offset;
        let mut s = String::new();
        
        while let Some(c) = self.peek() {
            if c == '"' {
                self.advance();
                return Some((TokenKind::Str(s), self.offset - start));
            }
            if c == '\\' {
                self.advance();
                let e = self.advance()?;
                let escaped = match e {
                    'n' => '\n',
                    't' => '\t',
                    'r' => '\r',
                    '\\' => '\\',
                    '"' => '"',
                    _ => e,
                };
                s.push(escaped);
            } else {
                s.push(c);
                self.advance();
            }
        }
        
        None
    }

    fn read_fstring(&mut self) -> Option<(TokenKind, usize)> {
        let start = self.offset;
        let quote = self.advance()?;
        let mut parts = Vec::new();
        let mut current_lit = String::new();

        while let Some(c) = self.peek() {
            if c == quote {
                self.advance();
                if !current_lit.is_empty() {
                    parts.push(FStringSeg::Lit(current_lit));
                }
                return Some((TokenKind::FString(parts), self.offset - start));
            }
            if c == '{' {
                self.advance();
                if self.peek() == Some('{') {
                    self.advance();
                    current_lit.push('{');
                    continue;
                }
                if !current_lit.is_empty() {
                    parts.push(FStringSeg::Lit(std::mem::take(&mut current_lit)));
                }
                let mut depth = 1u32;
                let mut expr = String::new();
                while let Some(ec) = self.peek() {
                    if ec == '}' {
                        depth -= 1;
                        if depth == 0 {
                            self.advance();
                            parts.push(FStringSeg::Expr(expr));
                            break;
                        }
                        expr.push('}');
                        self.advance();
                    } else if ec == '{' {
                        depth += 1;
                        expr.push('{');
                        self.advance();
                    } else {
                        expr.push(ec);
                        self.advance();
                    }
                }
                if depth > 0 {
                    return None;
                }
            } else if c == '}' {
                self.advance();
                if self.peek() == Some('}') {
                    self.advance();
                    current_lit.push('}');
                } else {
                    current_lit.push('}');
                }
            } else if c == '\\' {
                self.advance();
                let e = self.advance()?;
                let escaped = match e {
                    'n' => '\n',
                    't' => '\t',
                    'r' => '\r',
                    '\\' => '\\',
                    '"' => '"',
                    '\'' => '\'',
                    _ => e,
                };
                current_lit.push(escaped);
            } else {
                current_lit.push(c);
                self.advance();
            }
        }

        None
    }

    fn read_char(&mut self) -> Option<(TokenKind, usize)> {
        let start = self.offset;
        
        let c = if self.peek() == Some('\\') {
            self.advance();
            let e = self.advance()?;
            match e {
                'n' => '\n',
                't' => '\t',
                'r' => '\r',
                '\\' => '\\',
                '\'' => '\'',
                _ => e,
            }
        } else {
            self.advance()?
        };
        
        if self.peek() == Some('\'') {
            self.advance();
            Some((TokenKind::Char(c), self.offset - start))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integer_token() {
        let mut lexer = Lexer::new("42");
        let tok = lexer.next_token().unwrap();
        assert!(matches!(tok.value, TokenKind::Int(42)));
    }

    #[test]
    fn test_float_token() {
        let mut lexer = Lexer::new("3.14");
        let tok = lexer.next_token().unwrap();
        assert!(matches!(tok.value, TokenKind::Float(f) if (f - 3.14).abs() < 0.001));
    }

    #[test]
    fn test_string_token() {
        let mut lexer = Lexer::new("\"hello\"");
        let tok = lexer.next_token().unwrap();
        assert!(matches!(tok.value, TokenKind::Str(s) if s == "hello"));
    }

    #[test]
    fn test_keywords() {
        let mut lexer = Lexer::new("def if while return");
        assert!(matches!(lexer.next_token().unwrap().value, TokenKind::Def));
        assert!(matches!(lexer.next_token().unwrap().value, TokenKind::If));
        assert!(matches!(lexer.next_token().unwrap().value, TokenKind::While));
        assert!(matches!(lexer.next_token().unwrap().value, TokenKind::Return));
    }

    #[test]
    fn test_identifiers() {
        let mut lexer = Lexer::new("foo bar");
        assert!(matches!(lexer.next_token().unwrap().value, TokenKind::Ident(s) if s == "foo"));
        assert!(matches!(lexer.next_token().unwrap().value, TokenKind::Ident(s) if s == "bar"));
    }

    #[test]
    fn test_operators() {
        let mut lexer = Lexer::new("+ - * /");
        assert!(matches!(lexer.next_token().unwrap().value, TokenKind::Plus));
        assert!(matches!(lexer.next_token().unwrap().value, TokenKind::Minus));
        assert!(matches!(lexer.next_token().unwrap().value, TokenKind::Star));
        assert!(matches!(lexer.next_token().unwrap().value, TokenKind::Slash));
    }
}
