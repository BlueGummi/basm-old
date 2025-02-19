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
    pub fn parse(&mut self) -> Result<Vec<TokenKind>, &Vec<ParserError>> {
        let mut tokens = Vec::new();

        while let Some((token, span)) = self.lexer.next() {
            match token {
                Ok(TokenKind::Whitespace) | Ok(TokenKind::Tab) => {}
                Ok(TokenKind::MacroDef(_)) => tokens.extend(self.parse_single_macro()),
                Ok(t) => {
                    tokens.push(t);
                }
                Err(()) => {
                    self.errors.push(ParserError {
                        input: self.input.to_string(),
                        message: "Unexpected token".to_string(),
                        line: span.start,
                        column: span.end,
                    });
                }
            }
        }
        if !self.errors.is_empty() {
            return Err(&self.errors);
        }

        Ok(tokens)
    }
}
