use logos::Logos;

use crate::tokens::Token;

fn parse(input: &str) {
    let mut lex = Token::lexer(input);
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
