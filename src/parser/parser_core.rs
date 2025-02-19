use crate::*;
use logos::Logos;
use std::iter::Peekable;
use std::vec::IntoIter;
type ParsingLexer = Peekable<IntoIter<(Result<TokenKind, ()>, std::ops::Range<usize>)>>;
pub struct Parser<'a> {
    pub lexer: ParsingLexer,
    pub input: &'a str,
    pub errors: Vec<ParserError>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        let errors = Vec::new();
        let lexer = TokenKind::lexer(input).spanned();

        let first_pass_tokens = Self::first_pass(lexer);

        Parser {
            lexer: first_pass_tokens.into_iter().peekable(),
            input,
            errors,
        }
    }

    fn first_pass(
        lexer: logos::SpannedIter<'a, TokenKind>,
    ) -> Vec<(Result<TokenKind, ()>, std::ops::Range<usize>)> {
        let mut tokens = Vec::new();
        let mut lexer = lexer.peekable();

        while let Some((token, span)) = lexer.next() {
            match token {
                Ok(TokenKind::Ident(ident)) => {
                    if let Some((Ok(TokenKind::Colon), _)) = lexer.peek() {
                        let (_, colon_span) = lexer.next().unwrap();

                        if let Some((Ok(TokenKind::Ident(_)), _)) = lexer.peek() {
                            // If another identifier follows, treat it as normal tokens
                            tokens.push((Ok(TokenKind::Ident(ident)), span));
                            tokens.push((Ok(TokenKind::Colon), colon_span));
                        } else {
                            // Otherwise, it's a label
                            tokens.push((Ok(TokenKind::Label(ident)), span));
                        }
                    } else {
                        tokens.push((Ok(TokenKind::Ident(ident)), span));
                    }
                }
                _ => {
                    tokens.push((token, span));
                }
            }
        }

        tokens
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
