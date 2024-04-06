use color_eyre::eyre::{eyre, ContextCompat, Result};
use logos::Logos;

use crate::tokens::Token;

/// Convenience wrapper for the logos lexer.
struct Lexer<'a> {
    lex: logos::Lexer<'a, Token>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let lex = Token::lexer(input);

        Self { lex }
    }

    pub fn consume_exact(&mut self, token: Token) -> Result<()> {
        let token = self
            .lex
            .next()
            .context(format!("Expected {token:?} got nothing"))?;

        Ok(())
    }
}
