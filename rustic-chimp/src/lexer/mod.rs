use crate::token::{lookup_ident, Token};

pub struct Lexer<'a> {
    /// input source
    input: &'a str,

    /// current position in input (points to current char)
    position: usize,

    /// current reading position in input (after current char)
    read_position: usize,

    /// current char under examination
    ch: char,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lex = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: '\0',
        };

        lex.read_char();
        lex
    }

    pub fn read_char(&mut self) {
        self.ch = self.peek_char();
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn peek_char(&self) -> char {
        if self.read_position >= self.input.len() {
            return '\0';
        }

        self.input.chars().nth(self.read_position).unwrap()
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let tok = match self.ch {
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::Eq
                } else {
                    Token::Assign
                }
            }
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::NotEq
                } else {
                    Token::Bang
                }
            }

            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Asterisk,
            '/' => Token::Slash,
            '<' => Token::Lt,
            '>' => Token::Gt,
            ',' => Token::Comma,
            ';' => Token::Semicolon,
            '(' => Token::Lparen,
            ')' => Token::Rparen,
            '{' => Token::Lcurly,
            '}' => Token::Rcurly,
            '\0' => Token::Eof,
            _ => {
                if is_letter(self.ch) {
                    return self.read_identifier();
                }
                if is_digit(self.ch) {
                    return self.read_number();
                }
                Token::Illegal
            }
        };

        self.read_char();
        tok
    }

    fn read_identifier(&mut self) -> Token {
        let pos = self.position;
        while is_letter(self.ch) {
            self.read_char();
        }

        lookup_ident(&self.input[pos..self.position])
    }

    fn read_number(&mut self) -> Token {
        let pos = self.position;
        while is_digit(self.ch) {
            self.read_char();
        }

        Token::Int(self.input[pos..self.position].parse().unwrap())
    }

    fn skip_whitespace(&mut self) {
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
            self.read_char()
        }
    }
}

fn is_digit(ch: char) -> bool {
    ch.is_ascii_digit()
}

fn is_letter(ch: char) -> bool {
    ch.is_ascii_alphabetic() || ch == '_'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_token() {
        let input = "let five = 5;
let ten = 10;
let add = fn(x, y) {
     x + y;
};

let result = add(five, ten);
!-/*5;
5 < 10 > 5;

if (5 < 10) {
    return true;
} else {
    return false;
}

10 == 10;
10 != 9;
";

        let tests = vec![
            Token::Let,
            Token::Ident("five".to_string()),
            Token::Assign,
            Token::Int(5),
            Token::Semicolon,
            Token::Let,
            Token::Ident("ten".to_string()),
            Token::Assign,
            Token::Int(10),
            Token::Semicolon,
            Token::Let,
            Token::Ident("add".to_string()),
            Token::Assign,
            Token::Function,
            Token::Lparen,
            Token::Ident("x".to_string()),
            Token::Comma,
            Token::Ident("y".to_string()),
            Token::Rparen,
            Token::Lcurly,
            Token::Ident("x".to_string()),
            Token::Plus,
            Token::Ident("y".to_string()),
            Token::Semicolon,
            Token::Rcurly,
            Token::Semicolon,
            Token::Let,
            Token::Ident("result".to_string()),
            Token::Assign,
            Token::Ident("add".to_string()),
            Token::Lparen,
            Token::Ident("five".to_string()),
            Token::Comma,
            Token::Ident("ten".to_string()),
            Token::Rparen,
            Token::Semicolon,
            Token::Bang,
            Token::Minus,
            Token::Slash,
            Token::Asterisk,
            Token::Int(5),
            Token::Semicolon,
            Token::Int(5),
            Token::Lt,
            Token::Int(10),
            Token::Gt,
            Token::Int(5),
            Token::Semicolon,
            Token::If,
            Token::Lparen,
            Token::Int(5),
            Token::Lt,
            Token::Int(10),
            Token::Rparen,
            Token::Lcurly,
            Token::Return,
            Token::True,
            Token::Semicolon,
            Token::Rcurly,
            Token::Else,
            Token::Lcurly,
            Token::Return,
            Token::False,
            Token::Semicolon,
            Token::Rcurly,
            Token::Int(10),
            Token::Eq,
            Token::Int(10),
            Token::Semicolon,
            Token::Int(10),
            Token::NotEq,
            Token::Int(9),
            Token::Semicolon,
            Token::Eof,
        ];

        let mut lex = Lexer::new(input);
        for expected_token in tests {
            let tok = lex.next_token();
            assert_eq!(expected_token, tok);
        }
    }
}
