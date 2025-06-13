use crate::lexer::{Token, SpannedToken};
use crate::ast::*;
use anyhow::Result;

pub struct Parser<'a> {
    tokens: &'a [SpannedToken],
    pos: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [SpannedToken]) -> Self {
        Self { tokens, pos: 0 }
    }

    pub fn parse_program(&mut self) -> Result<Program> {
        let mut functions = Vec::new();
        let mut externs = Vec::new();
        let mut macros = Vec::new();
        while self.pos < self.tokens.len() {
            if self.check(&Token::Ident("extern".to_string())) {
                externs.push(self.parse_extern_function()?);
            } else if self.check(&Token::Ident("macro".to_string())) {
                macros.push(self.parse_macro_def()?);
            } else {
                functions.push(self.parse_function()?);
            }
        }
        Ok(Program { functions, externs, macros })
    }

    fn parse_macro_def(&mut self) -> Result<MacroDef> {
        self.expect(Token::Ident("macro".to_string()))?;
        let name = self.parse_ident()?;
        self.expect(Token::LParen)?;
        let mut params = Vec::new();
        if !self.check(Token::RParen) {
            loop {
                params.push(self.parse_ident()?);
                if self.check(Token::Comma) {
                    self.advance();
                } else {
                    break;
                }
            }
        }
        self.expect(Token::RParen)?;
        self.expect(Token::LBrace)?;
        let mut body = Vec::new();
        while !self.check(Token::RBrace) {
            body.push(self.parse_stmt()?);
        }
        self.expect(Token::RBrace)?;
        Ok(MacroDef { name, params, body })
    }

    fn parse_extern_function(&mut self) -> Result<ExternFunction> {
        self.expect(Token::Ident("extern".to_string()))?;
        self.expect(Token::Fn)?;
        let name = self.parse_ident()?;
        self.expect(Token::LParen)?;
        let mut params = Vec::new();
        if !self.check(Token::RParen) {
            loop {
                let param_name = self.parse_ident()?;
                self.expect(Token::Colon)?;
                let param_ty = self.parse_ident()?;
                params.push(Param { name: param_name, ty: param_ty });
                if self.check(Token::Comma) {
                    self.advance();
                } else {
                    break;
                }
            }
        }
        self.expect(Token::RParen)?;
        self.expect(Token::Arrow)?;
        let ret_ty = self.parse_ident()?;
        Ok(ExternFunction { name, params, ret_ty })
    }

    fn parse_function(&mut self) -> Result<Function> {
        let is_async = if self.check(&Token::Ident("async".to_string())) {
            self.advance();
            true
        } else {
            false
        };
        self.expect(Token::Fn)?;
        let name = self.parse_ident()?;
        self.expect(Token::LParen)?;
        let mut params = Vec::new();
        if !self.check(Token::RParen) {
            loop {
                let param_name = self.parse_ident()?;
                self.expect(Token::Colon)?;
                let param_ty = self.parse_ident()?;
                params.push(Param { name: param_name, ty: param_ty });
                if self.check(Token::Comma) {
                    self.advance();
                } else {
                    break;
                }
            }
        }
        self.expect(Token::RParen)?;
        self.expect(Token::LBrace)?;
        let mut body = Vec::new();
        while !self.check(Token::RBrace) {
            body.push(self.parse_stmt()?);
        }
        self.expect(Token::RBrace)?;
        Ok(Function { name, params, body, is_async })
    }

    fn parse_stmt(&mut self) -> Result<Stmt> {
        if self.check(Token::Let) {
            self.advance();
            let name = self.parse_ident()?;
            self.expect(Token::Eq)?;
            let expr = self.parse_expr()?;
            self.expect(Token::Semicolon)?;
            Ok(Stmt::Let { name, expr })
        } else if self.check(&Token::Ident("invoke".to_string())) {
            self.advance();
            let name = self.parse_ident()?;
            self.expect(Token::LParen)?;
            let mut args = Vec::new();
            if !self.check(Token::RParen) {
                loop {
                    args.push(self.parse_expr()?);
                    if self.check(Token::Comma) {
                        self.advance();
                    } else {
                        break;
                    }
                }
            }
            self.expect(Token::RParen)?;
            self.expect(Token::Semicolon)?;
            Ok(Stmt::MacroInvoke { name, args })
        } else {
            let expr = self.parse_expr()?;
            self.expect(Token::Semicolon)?;
            Ok(Stmt::Expr(expr))
        }
    }

    fn parse_expr(&mut self) -> Result<Expr> {
        if let Some(Token::Int) = self.peek_token() {
            let value = self.parse_int()?;
            Ok(Expr::Int(value))
        } else if let Some(Token::Ident) = self.peek_token() {
            // Check for 'await' and 'spawn' keywords
            if let Some(SpannedToken { token: Token::Ident(ref s), .. }) = self.peek() {
                if s == "await" {
                    self.advance();
                    let expr = self.parse_expr()?;
                    return Ok(Expr::Await(Box::new(expr)));
                } else if s == "spawn" {
                    self.advance();
                    let expr = self.parse_expr()?;
                    return Ok(Expr::Spawn(Box::new(expr)));
                }
            }
            let ident = self.parse_ident()?;
            if self.check(&Token::LParen) {
                self.advance();
                let mut args = Vec::new();
                if !self.check(&Token::RParen) {
                    loop {
                        args.push(self.parse_expr()?);
                        if self.check(&Token::Comma) {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                }
                self.expect(Token::RParen)?;
                Ok(Expr::Call { name: ident, args })
            } else {
                Ok(Expr::Ident(ident))
            }
        } else {
            Err(anyhow::anyhow!("Unexpected token in expression"))
        }
    }

    // --- Utility methods ---
    fn expect(&mut self, expected: Token) -> Result<()> {
        if self.check(&expected) {
            self.advance();
            Ok(())
        } else {
            Err(anyhow::anyhow!("Expected {:?}, found {:?}", expected, self.peek_token()))
        }
    }
    fn check(&self, expected: &Token) -> bool {
        match (self.peek_token(), expected) {
            (Some(Token::Fn), Token::Fn)
            | (Some(Token::Let), Token::Let)
            | (Some(Token::Arrow), Token::Arrow)
            | (Some(Token::LParen), Token::LParen)
            | (Some(Token::RParen), Token::RParen)
            | (Some(Token::LBrace), Token::LBrace)
            | (Some(Token::RBrace), Token::RBrace)
            | (Some(Token::Comma), Token::Comma)
            | (Some(Token::Colon), Token::Colon)
            | (Some(Token::Semicolon), Token::Semicolon)
            | (Some(Token::Eq), Token::Eq) => true,
            _ => false,
        }
    }
    fn advance(&mut self) {
        self.pos += 1;
    }
    fn peek(&self) -> Option<&SpannedToken> {
        self.tokens.get(self.pos)
    }
    fn peek_token(&self) -> Option<&Token> {
        self.peek().map(|st| &st.token)
    }
    fn parse_ident(&mut self) -> Result<String> {
        if let Some(SpannedToken { token: Token::Ident(ref s), .. }) = self.peek() {
            let ident = s.clone();
            self.advance();
            Ok(ident)
        } else {
            Err(anyhow::anyhow!("Expected identifier"))
        }
    }
    fn parse_int(&mut self) -> Result<i64> {
        if let Some(SpannedToken { token: Token::Int(i), .. }) = self.peek() {
            let value = *i;
            self.advance();
            Ok(value)
        } else {
            Err(anyhow::anyhow!("Expected integer"))
        }
    }
}
