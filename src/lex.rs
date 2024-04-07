use color_eyre::eyre::{eyre, ContextCompat, Result};
use logos::Logos;

use crate::tokens::Token;

/// Convenience wrapper for the logos lexer.
pub struct Lexer<'a> {
    lex: logos::Lexer<'a, Token>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let lex = Token::lexer(input);

        Self { lex }
    }

    pub fn consume_exact(&mut self, token: Token) -> Result<()> {
        let parse_token = self
            .lex
            .next()
            .context(format!("Expected {token:?} got nothing"))?;

        let next_token =
            parse_token.map_err(|()| eyre!("Expected {token:?}, no valid token found"))?;

        if next_token == token {
            Ok(())
        } else {
            Err(eyre!("Expected {token:?}, got {next_token:?}"))
        }
    }

    pub fn consume_slice_exact(&mut self, token: Token) -> Result<String> {
        let parse_token = self
            .lex
            .next()
            .context(format!("Expected {token:?} got nothing"))?;

        let next_token =
            parse_token.map_err(|()| eyre!("Expected {token:?}, no valid token found"))?;

        if next_token == token {
            Ok(self.lex.slice().to_owned())
        } else {
            Err(eyre!("Expected {token:?}, got {next_token:?}"))
        }
    }
}
