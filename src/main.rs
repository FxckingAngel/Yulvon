mod cli;
mod lexer;
mod parser;
mod ast;
mod ir;
mod codegen;
mod error;
mod semantic;
mod irgen;
mod runtime;

use cli::YulcCli;
use error::YulcResult;

fn main() -> YulcResult<()> {
    let cli = YulcCli::parse();
    cli.run()
}
