use std::io::{self, BufRead, Read, Write};

use crate::{lexer::Lexer, token::Token};

const PROMPT: &str = ">> ";

pub fn start<R, W>(input: R, output: W)
where
    R: Read,
    W: Write,
{
    let mut reader = io::BufReader::new(input);
    let mut writer = output;

    loop {
        write!(writer, "{}", PROMPT).unwrap();
        writer.flush().unwrap();

        let mut line = String::new();
        if reader.read_line(&mut line).is_err() {
            return;
        }

        let mut lex = Lexer::new(&line);
        let mut tok = lex.next_token();
        while tok != Token::Eof {
            println!("{:?}", tok);
            tok = lex.next_token();
        }
    }
}
