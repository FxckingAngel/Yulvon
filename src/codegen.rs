use crate::ir::*;
use inkwell::context::Context;
use inkwell::builder::Builder;
use inkwell::module::Module;
use inkwell::values::FunctionValue;
use anyhow::Result;

pub struct Codegen<'ctx> {
    pub context: &'ctx Context,
    pub module: Module<'ctx>,
    pub builder: Builder<'ctx>,
}

impl<'ctx> Codegen<'ctx> {
    pub fn new(context: &'ctx Context, name: &str) -> Self {
        let module = context.create_module(name);
        let builder = context.create_builder();
        Self { context, module, builder }
    }

    pub fn codegen_module(&mut self, ir: &IrModule) -> Result<()> {
        use inkwell::types::BasicTypeEnum;
        use inkwell::values::{BasicValueEnum, IntValue};

        // Declare external functions
        for ext in &ir.externs {
            let param_types: Vec<BasicTypeEnum> = ext.params.iter().map(|_| self.context.i64_type().into()).collect();
            let ret_type = self.context.i64_type(); // For now, assume all externs return i64
            let fn_type = ret_type.fn_type(&param_types, false);
            self.module.add_function(&ext.name, fn_type, None);
        }

        for func in &ir.functions {
            let fn_type = self.context.i64_type().fn_type(&[], false);
            let function = self.module.add_function(&func.name, fn_type, None);
            let entry = self.context.append_basic_block(function, "entry");
            self.builder.position_at_end(entry);
            let mut vars = std::collections::HashMap::new();
            for inst in &func.body {
                self.codegen_inst(inst, &mut vars)?;
            }
            self.builder.build_return(Some(&self.context.i64_type().const_zero()));
        }
        Ok(())
    }

    fn codegen_inst(&self, inst: &IrInst, vars: &mut std::collections::HashMap<String, IntValue>) -> Result<Option<IntValue>> {
        match inst {
            IrInst::Const(i) => Ok(Some(self.context.i64_type().const_int(*i as u64, true))),
            IrInst::Load(name) => {
                if let Some(&val) = vars.get(name) {
                    Ok(Some(val))
                } else {
                    Err(anyhow::anyhow!("Undefined variable: {}", name))
                }
            }
            IrInst::Store(name, value) => {
                let val = self.codegen_inst(value, vars)?.unwrap();
                vars.insert(name.clone(), val);
                Ok(None)
            }
            IrInst::Call(name, args) => {
                // Call an external or internal function
                let func = self.module.get_function(name)
                    .ok_or_else(|| anyhow::anyhow!("Unknown function: {}", name))?;
                let arg_vals: Vec<IntValue> = args.iter().map(|a| self.codegen_inst(a, vars).unwrap().unwrap()).collect();
                let arg_vals_ref: Vec<BasicValueEnum> = arg_vals.iter().map(|&v| v.into()).collect();
                let call = self.builder.build_call(func, &arg_vals_ref, "calltmp");
                Ok(Some(call.try_as_basic_value().left().unwrap().into_int_value()))
            }
            IrInst::AsyncCall(_name, _args) => {
                // For now, not implemented
                Ok(Some(self.context.i64_type().const_zero()))
            }
            IrInst::Await(inner) => {
                // Call the runtime's await function (assume it returns i64 for now)
                // In a real system, this would lower to a call to a runtime await function
                let awaited = self.codegen_inst(inner, vars)?.unwrap();
                // Here, you would insert a call to the runtime's await function
                // For demonstration, just return the awaited value
                Ok(Some(awaited))
            }
            IrInst::Spawn(inner) => {
                // Call the runtime's spawn function
                let spawned = self.codegen_inst(inner, vars)?.unwrap();
                // Here, you would insert a call to the runtime's spawn function
                // For demonstration, just return zero
                Ok(Some(self.context.i64_type().const_zero()))
            }
            IrInst::Ret(opt) => {
                if let Some(val) = opt {
                    let v = self.codegen_inst(val, vars)?.unwrap();
                    self.builder.build_return(Some(&v));
                } else {
                    self.builder.build_return(None);
                }
                Ok(None)
            }
        }
    }
}
