use crate::ir::*;
use crate::codegen::Codegen;
use inkwell::context::Context;
use inkwell::execution_engine::JitFunction;
use anyhow::Result;

pub fn run_jit(ir: &IrModule, entry: &str) -> Result<i64> {
    let context = Context::create();
    let mut codegen = Codegen::new(&context, "yuljit");
    codegen.codegen_module(ir)?;
    let engine = codegen.module.create_jit_execution_engine(inkwell::OptimizationLevel::None)?;
    unsafe {
        let main: JitFunction<unsafe extern "C" fn() -> i64> = engine.get_function(entry)?;
        Ok(main.call())
    }
}
