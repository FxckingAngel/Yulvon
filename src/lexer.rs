use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Token {
    #[token("fn")] Fn,
    #[token("let")] Let,
    #[token("=>")] Arrow,
    #[token("(")] LParen,
    #[token(")")] RParen,
    #[token("{")] LBrace,
    #[token("}")] RBrace,
    #[token(",")] Comma,
    #[token(":")] Colon,
    #[token(";")] Semicolon,
    #[token("=")] Eq,
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Ident(String),
    #[regex(r"[0-9]+", |lex| lex.slice().parse())]
    Int(i64),
    #[regex(r"[ \t\n\r]+", logos::skip)] Whitespace,
    #[error]
    Error,
}

#[derive(Debug, Clone)]
pub struct SpannedToken {
    pub token: Token,
    pub span: std::ops::Range<usize>,
}

pub fn lex(source: &str) -> Vec<SpannedToken> {
    let mut lexer = Token::lexer(source);
    let mut tokens = Vec::new();
    while let Some(token) = lexer.next() {
        let span = lexer.span();
        tokens.push(SpannedToken {
            token,
            span,
        });
    }
    tokens
}
