#[derive(Debug, Clone)]
pub struct Program {
    pub functions: Vec<Function>,
    pub externs: Vec<ExternFunction>,
    pub macros: Vec<MacroDef>,
}

#[derive(Debug, Clone)]
pub struct MacroDef {
    pub name: String,
    pub params: Vec<String>,
    pub body: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub struct ExternFunction {
    pub name: String,
    pub params: Vec<Param>,
    pub ret_ty: String,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub params: Vec<Param>,
    pub body: Vec<Stmt>,
    pub is_async: bool,
}

#[derive(Debug, Clone)]
pub struct Param {
    pub name: String,
    pub ty: String,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Let { name: String, expr: Expr },
    Expr(Expr),
    MacroInvoke { name: String, args: Vec<Expr> },
}

#[derive(Debug, Clone)]
pub enum Expr {
    Int(i64),
    Ident(String),
    Call { name: String, args: Vec<Expr> },
    Await(Box<Expr>),
    Spawn(Box<Expr>),
}
