#[derive(Debug, Eq, PartialEq)]
pub enum Token {
    Illegal,
    Eof,

    // Identifiers + literals
    Ident(String),
    Int(i64),

    // Operators
    Assign,   // =
    Plus,     // +
    Minus,    // -
    Bang,     // !
    Asterisk, // *
    Slash,    // /

    // Comparisions
    Lt,    // <
    Gt,    // >
    Eq,    // ==
    NotEq, // !=

    // Delimeters
    Comma,     // ,
    Semicolon, // ;
    Lparen,    // (
    Rparen,    // )
    Lcurly,    // {
    Rcurly,    // }

    // Keywords
    Function, // fn
    Let,      // let
    True,     // true
    False,    // false
    If,       // if
    Else,     // else
    Return,   // return
}

pub fn lookup_ident(ident: &str) -> Token {
    match ident {
        // keywords
        "fn" => Token::Function,
        "let" => Token::Let,
        "true" => Token::True,
        "false" => Token::False,
        "if" => Token::If,
        "else" => Token::Else,
        "return" => Token::Return,

        // identifier
        _ => Token::Ident(ident.to_string()),
    }
}
