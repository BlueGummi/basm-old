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
    if let Some((token, _)) = token_iter.next() {
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
                        start_pos: 0,
                        last_pos: 0,
                    })
                }
            }
            _ => Err(ParserError {
                file: file.to_string(),
                help: None,
                input: input.to_string(),
                message: "unexpected token".to_string(),
                start_pos: 0,
                last_pos: 0,
            }),
        }
    } else {
        Err(ParserError {
            file: file.to_string(),
            help: None,
            input: input.to_string(),
            message: "unexpected end of expression".to_string(),
            start_pos: 0,
            last_pos: 0,
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
