use crate::{lex::Lexer, tokens::Token};
use color_eyre::Result;

fn parse(input: &str) {
    let mut lex = Lexer::new(input);
}

#[cfg(test)]
mod tests {
    use super::parse;

    #[test]
    fn simple_parse() {
        let input = "Steve";

        parse(input);
    }
}
