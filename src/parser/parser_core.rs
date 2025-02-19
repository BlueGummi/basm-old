use crate::*;
use logos::Logos;

pub struct Parser<'a> {
    pub lexer: logos::SpannedIter<'a, TokenKind>,
    pub input: &'a str,
    pub errors: Vec<ParserError>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        let errors = Vec::new();
        let lexer = TokenKind::lexer(input).spanned();
        Parser {
            lexer,
            input,
            errors,
        }
    }
}
