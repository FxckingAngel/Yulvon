use crate::ast::*;

pub fn expand_macros(program: &mut Program) {
    let macro_map = program.macros.iter().map(|m| (m.name.clone(), m)).collect::<std::collections::HashMap<_, _>>();
    for func in &mut program.functions {
        expand_stmts(&macro_map, &mut func.body);
    }
}

fn expand_stmts<'a>(macro_map: &std::collections::HashMap<String, &'a MacroDef>, stmts: &mut Vec<Stmt>) {
    let mut i = 0;
    while i < stmts.len() {
        match &stmts[i] {
            Stmt::MacroInvoke { name, args } => {
                if let Some(mac) = macro_map.get(name) {
                    let mut expanded = mac.body.clone();
                    // TODO: parameter substitution for macro args
                    stmts.splice(i..=i, expanded);
                    // Do not increment i, as we want to process the newly inserted stmts
                    continue;
                }
            }
            Stmt::Expr(_) | Stmt::Let { .. } => {}
        }
        i += 1;
    }
}
