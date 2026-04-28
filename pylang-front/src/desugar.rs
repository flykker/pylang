use crate::ast::{Expr, Stmt};

pub fn desugar_decorators(stmts: Vec<Stmt>) -> Vec<Stmt> {
    let mut result = Vec::new();
    for stmt in stmts {
        match stmt {
            Stmt::Fn(f) if !f.decorators.is_empty() => {
                let mut fn_stmt = f.clone();
                fn_stmt.decorators = vec![];
                result.push(Stmt::Fn(fn_stmt));

                let inner = build_decorator_chain(&f.decorators, &f.name);
                result.push(Stmt::Assign(crate::ast::Assign {
                    target: Box::new(Expr::Ident(f.name.clone())),
                    val: inner,
                }));
            }
            _ => result.push(stmt),
        }
    }
    result
}

fn build_decorator_chain(decorators: &[Expr], fn_name: &str) -> Expr {
    let mut expr = Expr::Ident(fn_name.to_string());
    for dec in decorators.iter().rev() {
        expr = Expr::Call {
            func: Box::new(dec.clone()),
            args: vec![expr],
        };
    }
    expr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_desugar_single_decorator() {
        let ast = vec![Stmt::Fn(crate::ast::Fn {
            name: "foo".to_string(),
            params: vec![],
            ret: None,
            body: vec![],
            decorators: vec![Expr::Ident("decorator".to_string())],
            captures: vec![],
        })];
        let result = desugar_decorators(ast);
        assert_eq!(result.len(), 2);
        assert!(matches!(&result[0], Stmt::Fn(f) if f.decorators.is_empty()));
        assert!(matches!(&result[1], Stmt::Assign(a) if matches!(&*a.target, Expr::Ident(n) if n == "foo")));
    }

    #[test]
    fn test_desugar_decorator_chain() {
        let ast = vec![Stmt::Fn(crate::ast::Fn {
            name: "foo".to_string(),
            params: vec![],
            ret: None,
            body: vec![],
            decorators: vec![
                Expr::Ident("dec1".to_string()),
                Expr::Ident("dec2".to_string()),
            ],
            captures: vec![],
        })];
        let result = desugar_decorators(ast);
        assert_eq!(result.len(), 2);
        if let Stmt::Assign(a) = &result[1] {
            assert!(matches!(&a.val, Expr::Call { func, args } if {
                let dec1_check = matches!(&**func, Expr::Ident(n) if n == "dec1");
                let inner = args.first();
                let inner_call = inner.and_then(|e| match e {
                    Expr::Call { func: inner_func, args: inner_args } => Some((inner_func.as_ref(), inner_args.as_slice())),
                    _ => None,
                });
                let inner_ok = inner_call.map(|(inner_func, inner_args)| {
                    matches!(inner_func, Expr::Ident(n) if n == "dec2")
                    && matches!(inner_args.first(), Some(Expr::Ident(n)) if n == "foo")
                }).unwrap_or(false);
                dec1_check && inner_ok
            }));
        }
    }

    #[test]
    fn test_desugar_no_decorators() {
        let ast = vec![Stmt::Fn(crate::ast::Fn {
            name: "foo".to_string(),
            params: vec![],
            ret: None,
            body: vec![],
            decorators: vec![],
            captures: vec![],
        })];
        let result = desugar_decorators(ast);
        assert_eq!(result.len(), 1);
    }
}