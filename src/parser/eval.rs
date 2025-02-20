use crate::*;
use std::iter::Peekable;

type Evalex<'a> = Peekable<logos::SpannedIter<'a, tokens::TokenKind>>;

pub fn parse_expression(
    file: String,
    input: String,
    token_iter: &mut Evalex,
) -> Result<Expr, ParserError> {
    parse_add_sub(file, input, token_iter)
}

fn parse_add_sub(
    file: String,
    input: String,
    token_iter: &mut Evalex,
) -> Result<Expr, ParserError> {
    let mut expr = parse_mul_shift(file.to_string(), input.to_string(), token_iter)?;

    while let Some((token, _)) = token_iter.peek() {
        match token {
            Ok(TokenKind::Plus) => {
                token_iter.next();
                expr = Expr::Add(
                    Box::new(expr),
                    Box::new(parse_mul_shift(
                        file.to_string(),
                        input.to_string(),
                        token_iter,
                    )?),
                );
            }
            Ok(TokenKind::Minus) => {
                token_iter.next();
                expr = Expr::Sub(
                    Box::new(expr),
                    Box::new(parse_mul_shift(
                        file.to_string(),
                        input.to_string(),
                        token_iter,
                    )?),
                );
            }
            _ => break,
        }
    }
    Ok(expr)
}

fn parse_mul_shift(
    file: String,
    input: String,
    token_iter: &mut Evalex,
) -> Result<Expr, ParserError> {
    let mut expr = parse_primary(file.to_string(), input.to_string(), token_iter)?;

    while let Some((token, _)) = token_iter.peek() {
        match token {
            Ok(TokenKind::Star) => {
                token_iter.next();
                expr = Expr::Mul(
                    Box::new(expr),
                    Box::new(parse_primary(
                        file.to_string(),
                        input.to_string(),
                        token_iter,
                    )?),
                );
            }
            Ok(TokenKind::LessLess) => {
                token_iter.next();
                expr = Expr::Shl(
                    Box::new(expr),
                    Box::new(parse_primary(
                        file.to_string(),
                        input.to_string(),
                        token_iter,
                    )?),
                );
            }
            Ok(TokenKind::GreaterGreater) => {
                token_iter.next();
                expr = Expr::Shr(
                    Box::new(expr),
                    Box::new(parse_primary(
                        file.to_string(),
                        input.to_string(),
                        token_iter,
                    )?),
                );
            }
            _ => break,
        }
    }
    Ok(expr)
}

fn parse_primary(
    file: String,
    input: String,
    token_iter: &mut Evalex,
) -> Result<Expr, ParserError> {
    let mut last_loc = 0..0;

    if let Some((_, loc)) = token_iter.peek() {
        last_loc = loc.clone();
    }

    if let Some((token, l)) = token_iter.next() {
        last_loc = l.clone();

        match token {
            Ok(TokenKind::IntLit(num)) => Ok(Expr::Int(num)),
            Ok(TokenKind::LeftParen) => {
                let expr = parse_expression(file.to_string(), input.to_string(), token_iter)?;
                if let Some((Ok(TokenKind::RightParen), _)) = token_iter.next() {
                    Ok(expr)
                } else {
                    Err(ParserError {
                        file: file.to_string(),
                        help: None,
                        input: input.to_string(),
                        message: "unmatched parenthesis".to_string(),
                        start_pos: last_loc.start,
                        last_pos: last_loc.end,
                    })
                }
            }
            Ok(v) => Err(ParserError {
                file: file.to_string(),
                help: None,
                input: input.to_string(),
                message: format!("unexpected {v} in expression"), 
                start_pos: last_loc.start,
                last_pos: last_loc.end,
            }),
            _ =>  Err(ParserError {
                file: file.to_string(),
                help: None,
                input: input.to_string(),
                message: String::from("reached an error while parsing expression\n      maybe the expression is invalid?"), 
                start_pos: last_loc.start,
                last_pos: last_loc.end,
            }),

        }
    } else {
        Err(ParserError {
            file: file.to_string(),
            help: None,
            input: input.to_string(),
            message: "unexpected end of expression".to_string(),
            start_pos: last_loc.start,
            last_pos: last_loc.end,
        })
    }
}

pub fn evaluate_expression(
    file: &String,
    input: String,
    token_iter: &mut Evalex,
) -> Result<i64, ParserError> {
    let expr = parse_expression(file.to_string(), input, token_iter)?;
    Ok(expr.evaluate())
}
