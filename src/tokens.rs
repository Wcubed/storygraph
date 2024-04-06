use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone, Copy)]
#[logos(skip r"[ \t]+")]
pub enum Token {
    #[regex("[a-zA-Z]+")]
    Name,

    #[token(",")]
    Comma,

    #[token("\n")]
    Newline,
}
