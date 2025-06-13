#[derive(Debug, Clone)]
pub enum IrInst {
    Const(i64),
    Load(String),
    Store(String, Box<IrInst>),
    Call(String, Vec<IrInst>),
    Ret(Option<Box<IrInst>>),
    AsyncCall(String, Vec<IrInst>),
    Await(Box<IrInst>),
    Spawn(Box<IrInst>),
}

#[derive(Debug, Clone)]
pub struct IrFunction {
    pub name: String,
    pub body: Vec<IrInst>,
}

#[derive(Debug, Clone)]
pub struct IrModule {
    pub functions: Vec<IrFunction>,
    pub externs: Vec<IrExtern>,
}

#[derive(Debug, Clone)]
pub struct IrExtern {
    pub name: String,
    pub params: Vec<String>,
    pub ret_ty: String,
}
