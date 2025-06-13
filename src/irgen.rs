use crate::ast::*;
use crate::ir::*;

pub fn lower_program(prog: &Program) -> IrModule {
    let mut functions = Vec::new();
    let mut externs = Vec::new();
    for func in &prog.functions {
        functions.push(lower_function(func));
    }
    for ext in &prog.externs {
        externs.push(IrExtern {
            name: ext.name.clone(),
            params: ext.params.iter().map(|p| p.ty.clone()).collect(),
            ret_ty: ext.ret_ty.clone(),
        });
    }
    IrModule { functions, externs }
}

fn lower_function(func: &Function) -> IrFunction {
    let mut body = Vec::new();
    for stmt in &func.body {
        lower_stmt(stmt, &mut body);
    }
    IrFunction {
        name: func.name.clone(),
        body,
    }
}

fn lower_stmt(stmt: &Stmt, body: &mut Vec<IrInst>) {
    match stmt {
        Stmt::Let { name, expr } => {
            let value = lower_expr(expr);
            body.push(IrInst::Store(name.clone(), Box::new(value)));
        }
        Stmt::Expr(expr) => {
            let _ = lower_expr(expr);
        }
    }
}

fn lower_expr(expr: &Expr) -> IrInst {
    match expr {
        Expr::Int(i) => IrInst::Const(*i),
        Expr::Ident(name) => IrInst::Load(name.clone()),
        Expr::Call { name, args } => {
            let lowered_args = args.iter().map(lower_expr).collect();
            IrInst::Call(name.clone(), lowered_args)
        }
        Expr::Await(inner) => {
            let inner_ir = lower_expr(inner);
            IrInst::Await(Box::new(inner_ir))
        }
        Expr::Spawn(inner) => {
            let inner_ir = lower_expr(inner);
            IrInst::Spawn(Box::new(inner_ir))
        }
    }
}
