use clap::{Parser, Subcommand};
use crate::error::YulcResult;

#[derive(Parser)]
#[command(name = "yulc", about = "Yulvon Compiler: The fastest programming language in the world.")]
pub struct YulcCli {
    #[command(subcommand)]
    pub command: YulcCommand,
}

#[derive(Subcommand)]
pub enum YulcCommand {
    /// Compile a Yulvon source file
    Build {
        #[arg(value_name = "FILE")] file: String,
        #[arg(short, long)]
        release: bool,
        #[arg(short, long)]
        target: Option<String>, // e.g. "x86_64", "wasm32"
        #[arg(long)]
        output: Option<String>, // output file name
    },
}

impl YulcCli {
    pub fn run(&self) -> YulcResult<()> {
        match &self.command {
            YulcCommand::Build { file, release, target } => {
                use std::fs;
                use crate::lexer::lex;
                use crate::parser::Parser;
                use crate::ast::Program;
                use crate::error::YulcError;

                let source = fs::read_to_string(file)
                    .map_err(|e| YulcError::Io(e))?;
                let tokens = lex(&source);
                let mut parser = Parser::new(&tokens);
                let program: Program = parser.parse_program()
                    .map_err(|e| YulcError::Parse(format!("{e}")))?;
                crate::semantic::analyze_program(&program)
                    .map_err(|e| YulcError::Other(format!("semantic error: {e}")))?;
                println!("AST: {:#?}", program);
                let irmod = crate::irgen::lower_program(&program);
                println!("IR: {:#?}", irmod);
                use inkwell::context::Context;
                let context = Context::create();
                let mut codegen = crate::codegen::Codegen::new(&context, "yulmod");
                codegen.codegen_module(&irmod)
                    .map_err(|e| YulcError::Codegen(format!("{e}")))?;
                let target = target.as_deref().unwrap_or("x86_64");
                let output = output.as_deref().unwrap_or("a.out");
                match target {
                    "x86_64" => {
                        use inkwell::targets::{InitializationConfig, Target, TargetMachine, FileType};
                        Target::initialize_all(&InitializationConfig::default());
                        let triple = TargetMachine::get_default_triple();
                        let target = Target::from_triple(&triple).unwrap();
                        let machine = target
                            .create_target_machine(
                                &triple,
                                "generic",
                                "",
                                inkwell::OptimizationLevel::Default,
                                inkwell::targets::RelocMode::Default,
                                inkwell::targets::CodeModel::Default,
                            )
                            .unwrap();
                        codegen.module.write_to_file(FileType::Object, output).unwrap();
                        println!("Native object file written to {}", output);
                    }
                    "wasm32" => {
                        // WASM output via LLVM (requires LLVM built with WASM backend)
                        use inkwell::targets::{InitializationConfig, Target, FileType};
                        Target::initialize_webassembly(&InitializationConfig::default());
                        let triple = inkwell::targets::TargetTriple::create("wasm32-unknown-unknown");
                        let target = Target::from_triple(&triple).unwrap();
                        let machine = target
                            .create_target_machine(
                                &triple,
                                "generic",
                                "",
                                inkwell::OptimizationLevel::Default,
                                inkwell::targets::RelocMode::Default,
                                inkwell::targets::CodeModel::Default,
                            )
                            .unwrap();
                        codegen.module.write_to_file(FileType::Object, output).unwrap();
                        println!("WASM object file written to {}", output);
                    }
                    _ => {
                        return Err(YulcError::Other(format!("Unknown target: {}", target)));
                    }
                }
                Ok(())
            }
        }
    }
}
