use thiserror::Error;

#[derive(Error, Debug)]
pub enum YulcError {
    #[error("Lexing error: {0}")]
    Lex(String),
    #[error("Parsing error: {0}")]
    Parse(String),
    #[error("Codegen error: {0}")]
    Codegen(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Other error: {0}")]
    Other(String),
}

pub type YulcResult<T> = Result<T, YulcError>;
