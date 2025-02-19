use crate::*;

impl Parser<'_> {
    pub fn parse_macros(&mut self) -> Result<Vec<TokenKind>, &Vec<ParserError>> {
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
    fn parse_single_macro_argument(&mut self, arg_name: String) -> Vec<FullArgument> {
        let input_str = self.input.to_string();
        let (val, _) = match self.lexer.next() {
            Some((v, l)) => (v, l),
            None => panic!(),
        };
        let mut args = Vec::new();
        match val {
            Ok(TokenKind::Colon) => {
                let (val, loc) = match self.lexer.next() {
                    Some((v, l)) => (v, l),
                    None => panic!(),
                };
                match val {
                    Ok(TokenKind::Ident(arg_type_str)) => {
                        let mut leave = false;
                        let arg_type =
                            ArgumentType::from_string(&arg_type_str).unwrap_or_else(|| {
                                self.errors.push(ParserError {
                                    input: input_str,
                                    message: format!("Invalid argument type: {}", arg_type_str),
                                    line: loc.start,
                                    column: loc.end,
                                });
                                leave = true;
                                ArgumentType::Reg
                            });
                        if leave {
                            return args;
                        }
                        args.push(FullArgument {
                            name: arg_name.to_string(),
                            arg_type,
                        });
                    }
                    _ => {
                        self.errors.push(ParserError {
                            input: input_str,
                            message: "Expected argument type after colon".to_string(),
                            line: loc.start,
                            column: loc.end,
                        });
                    }
                }
            }
            _ => {
                self.errors.push(ParserError {
                    input: input_str,
                    message: "Expected colon after argument name".to_string(),
                    line: 0,
                    column: 0,
                });
            }
        }
        args
    }

    fn parse_macro_arguments(&mut self, name: String) -> Vec<TokenKind> {
        let input_str = self.input.to_string();
        let mut tokens = Vec::new();
        let mut args = Vec::new();
        loop {
            let (val, _) = match self.lexer.next() {
                Some((v, l)) => (v, l),
                None => panic!(),
            };
            match val {
                Ok(TokenKind::Tab) | Ok(TokenKind::Whitespace) | Ok(TokenKind::Comma) => {
                    continue;
                }
                Ok(TokenKind::Ident(arg_name)) => {
                    args.extend(self.parse_single_macro_argument(arg_name));
                }
                Ok(TokenKind::RightParen) => break,
                _ => {
                    self.errors.push(ParserError {
                        input: self.input.to_string(),
                        message: "Invalid macro argument syntax".to_string(),
                        line: 0,
                        column: 0,
                    });
                }
            }
        }
        let value = if let Some(v) = self.lexer.next() {
            v.0
        } else {
            panic!();
        };
        match value {
            Ok(TokenKind::LeftBrace) => {
                let mut brace_count = 1;
                let mut macro_tokens = Vec::new();

                for (tok, span) in self.lexer.by_ref() {
                    match tok {
                        Ok(TokenKind::LeftBrace) => brace_count += 1,
                        Ok(TokenKind::RightBrace) => {
                            brace_count -= 1;
                            if brace_count == 0 {
                                break;
                            }
                        }
                        Ok(t) => macro_tokens.push(t),
                        _ => {
                            self.errors.push(ParserError {
                                input: self.input.to_string(),
                                message: "Invalid token in macro body".to_string(),
                                line: span.start,
                                column: span.end,
                            });
                        }
                    }
                }
                tokens.push(TokenKind::Macro(MacroContent {
                    name,
                    args,
                    tokens: macro_tokens,
                }));
            }
            _ => {
                self.errors.push(ParserError {
                    input: input_str,
                    message: "Expected open brace to start macro body".to_string(),
                    line: 0,
                    column: 0,
                });
            }
        }
        tokens
    }

    fn parse_single_macro(&mut self) -> Vec<TokenKind> {
        let input_str = self.input.to_string();
        let mut tokens = Vec::new();

        let name = if let Ok(TokenKind::Ident(v)) = self.lexer.next().unwrap().0 {
            v
        } else {
            self.errors.push(ParserError {
                input: input_str,
                message: "expected ident after macro decl".to_string(),
                line: 0,
                column: 0,
            });
            return tokens;
        };

        let (val, loc) = match self.lexer.next() {
            Some((v, l)) => (v, l),
            None => panic!(),
        };
        match val {
            Ok(TokenKind::LeftParen) => {
                tokens.extend(self.parse_macro_arguments(name));
            }
            _ => {
                self.errors.push(ParserError {
                    input: input_str,
                    message: "Expected open paren after macro name".to_string(),
                    line: loc.start,
                    column: loc.end,
                });
            }
        }
        tokens
    }
}
