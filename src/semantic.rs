use crate::ast::*;
use std::collections::HashMap;
use anyhow::{Result, bail};

#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub ty: String,
}

#[derive(Debug, Clone)]
pub struct Scope {
    pub symbols: HashMap<String, Symbol>,
}

impl Scope {
    pub fn new() -> Self {
        Self { symbols: HashMap::new() }
    }
    pub fn insert(&mut self, name: String, ty: String) {
        self.symbols.insert(name.clone(), Symbol { name, ty });
    }
    pub fn get(&self, name: &str) -> Option<&Symbol> {
        self.symbols.get(name)
    }
}

pub fn analyze_program(prog: &Program) -> Result<()> {
    for func in &prog.functions {
        analyze_function(func)?;
    }
    Ok(())
}

fn analyze_function(func: &Function) -> Result<()> {
    let mut scope = Scope::new();
    for param in &func.params {
        scope.insert(param.name.clone(), param.ty.clone());
    }
    for stmt in &func.body {
        analyze_stmt(stmt, &mut scope)?;
    }
    Ok(())
}

fn analyze_stmt(stmt: &Stmt, scope: &mut Scope) -> Result<()> {
    match stmt {
        Stmt::Let { name, expr } => {
            let ty = analyze_expr(expr, scope)?;
            scope.insert(name.clone(), ty);
            Ok(())
        }
        Stmt::Expr(expr) => {
            analyze_expr(expr, scope)?;
            Ok(())
        }
    }
}

fn analyze_expr(expr: &Expr, scope: &Scope) -> Result<String> {
    match expr {
        Expr::Int(_) => Ok("int".to_string()),
        Expr::Ident(name) => {
            if let Some(sym) = scope.get(name) {
                Ok(sym.ty.clone())
            } else {
                bail!("Undefined variable: {}", name)
            }
        }
        Expr::Call { name, args } => {
            // For now, assume all calls return int
            for arg in args {
                analyze_expr(arg, scope)?;
            }
            Ok("int".to_string())
        }
    }
}
