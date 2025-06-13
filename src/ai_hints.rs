use crate::ast::*;
use crate::ir::*;

pub struct Hint {
    pub message: String,
    pub location: Option<String>,
}

pub fn analyze_ast(program: &Program) -> Vec<Hint> {
    let mut hints = Vec::new();
    for func in &program.functions {
        if func.body.len() > 100 {
            hints.push(Hint {
                message: format!("Function '{}' is large; consider splitting for better cache locality.", func.name),
                location: Some(func.name.clone()),
            });
        }
        for stmt in &func.body {
            if let Stmt::Let { name, expr } = stmt {
                if let Expr::Int(0) = expr {
                    hints.push(Hint {
                        message: format!("Variable '{}' initialized to 0; check if zero-initialization is needed.", name),
                        location: Some(name.clone()),
                    });
                }
            }
        }
    }
    hints
}

pub fn analyze_ir(ir: &IrModule) -> Vec<Hint> {
    let mut hints = Vec::new();
    for func in &ir.functions {
        if func.body.len() > 200 {
            hints.push(Hint {
                message: format!("IR function '{}' is very large; consider inlining or splitting.", func.name),
                location: Some(func.name.clone()),
            });
        }
    }
    hints
}
