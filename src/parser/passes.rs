use crate::eval::evaluate_expression;
use crate::*;
type PassOne = Result<Vec<(Result<TokenKind, ()>, std::ops::Range<usize>)>, Vec<ParserError>>;
impl<'a> Parser<'a> {
    pub fn first_pass(
        file: String,
        input: String,
        lexer: logos::SpannedIter<'a, TokenKind>,
    ) -> PassOne {
        let mut tokens = Vec::new();
        let mut lexer = lexer.peekable();
        let mut errors = Vec::new();

        while let Some((token, span)) = lexer.next() {
            match token {
                Ok(TokenKind::Ident(ident)) => {
                    if let Some((Ok(TokenKind::Colon), _)) = lexer.peek() {
                        let (_, _) = lexer.next().unwrap();
                        tokens.push((Ok(TokenKind::Label(ident)), span));
                    } else {
                        tokens.push((Ok(TokenKind::Ident(ident)), span));
                    }
                }
                Ok(TokenKind::LeftParen) => 'lpn: {
                    match parse_expression_after_left_paren(&file, input.to_string(), &mut lexer) {
                        Ok(Some((value, new_span))) => {
                            tokens.push((Ok(TokenKind::IntLit(value)), new_span));
                        }
                        Ok(None) => {
                            tokens.push((Ok(TokenKind::LeftParen), span));
                            break 'lpn;
                        }
                        Err(e) => {
                            errors.push(e);
                        }
                    }
                }
                _ => {
                    tokens.push((token, span));
                }
            }
        }

        if !errors.is_empty() {
            return Err(errors);
        }
        Ok(tokens)
    }
    pub fn second_pass(
        &mut self,
        tokens: Vec<(Result<TokenKind, ()>, std::ops::Range<usize>)>,
    ) -> Vec<(Result<TokenKind, ()>, std::ops::Range<usize>)> {
        let mut new_tokens = Vec::new();
        let mut token_iter = tokens.into_iter().peekable();

        while let Some((token, span)) = token_iter.next() {
            match token {
                Ok(TokenKind::Ident(name)) => {
                    let mut has_colon = false;
                    let mut peek_iter = token_iter.clone();
                    while let Some((peek_token, _)) = peek_iter.peek() {
                        match peek_token {
                            Ok(TokenKind::Newline) => break,
                            Ok(TokenKind::Colon)
                            | Ok(TokenKind::LeftBrace)
                            | Ok(TokenKind::StringLit(_)) => {
                                has_colon = true;
                                break;
                            }
                            _ => {
                                peek_iter.next();
                            }
                        }
                    }

                    if has_colon {
                        new_tokens.push((Ok(TokenKind::Ident(name)), span));
                    } else {
                        let mut args = Vec::new();
                        while let Some((token, loc)) = token_iter.peek() {
                            match token {
                                Ok(TokenKind::Comma) => {
                                    token_iter.next();
                                }
                                Ok(TokenKind::Newline) => {
                                    break;
                                }
                                Ok(t) => {
                                    if let Some(v) = self.parse_argument(t.clone()) {
                                        args.push((v, loc.clone()));
                                    }
                                    token_iter.next();
                                }
                                _ => {
                                    token_iter.next();
                                }
                            }
                        }
                        new_tokens.push((
                            Ok(TokenKind::Instruction(InstructionData { name, args })),
                            span,
                        ));
                    }
                }
                _ => {
                    new_tokens.push((token, span));
                }
            }
        }
        new_tokens
    }
}

fn parse_expression_after_left_paren(
    file: &str,
    input: String,
    lexer: &mut std::iter::Peekable<logos::SpannedIter<'_, TokenKind>>,
) -> Result<Option<(i64, logos::Span)>, ParserError> {
    let mut peek_iter = lexer.clone();
    while let Some((peek_token, _)) = peek_iter.peek() {
        match peek_token {
            Ok(TokenKind::Newline) => break,
            Ok(TokenKind::Colon) | Ok(TokenKind::LeftBrace) => {
                return Ok(None);
            }
            _ => {
                peek_iter.next();
            }
        }
    }

    loop {
        let next_token = lexer.peek().cloned();
        match next_token {
            Some((Ok(TokenKind::Comma), _)) => {
                lexer.next();
            }
            Some((Ok(TokenKind::Newline), _)) => {
                break;
            }
            Some((Ok(_), span)) => {
                let value = evaluate_expression(&file.to_string(), input.to_string(), lexer)?;
                return Ok(Some((value, span.clone())));
            }
            Some((Err(_), span)) => {
                return Err(ParserError {
                    file: file.to_string(),
                    help: None,
                    input: input.to_string(),
                    message: String::from("invalid token in expression"),
                    start_pos: span.start,
                    last_pos: span.end,
                });
            }
            None => {
                break;
            }
        }
    }

    Err(ParserError {
        file: file.to_string(),
        help: None,
        input: input.to_string(),
        message: String::from("failed to parse expression after left paren"),
        start_pos: 0,
        last_pos: 0,
    })
}
