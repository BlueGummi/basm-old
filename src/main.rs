use logos::Logos;
use serde::Serialize;
use std::fmt;

#[derive(Debug, PartialEq, Serialize)]
pub struct MacroContent {
    pub name: String,
    pub args: Vec<FullArgument>, // Vector of FullArgument
    pub tokens: Vec<TokenKind>,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct FullArgument {
    pub name: String,
    pub arg_type: ArgumentType,
}

#[derive(Debug, PartialEq, Serialize)]
pub enum ArgumentType {
    Mem,
    Imem,
    Ireg,
    Imm,
    Reg,
}

impl ArgumentType {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "mem" => Some(ArgumentType::Mem),
            "imem" => Some(ArgumentType::Imem),
            "ireg" => Some(ArgumentType::Ireg),
            "imm" => Some(ArgumentType::Imm),
            "reg" => Some(ArgumentType::Reg),
            _ => None,
        }
    }
}

#[derive(Logos, Debug, PartialEq, Serialize)]
pub enum TokenKind {
    #[token("\n")]
    Newline,

    #[token(" ")]
    Whitespace,

    #[token("\t")]
    Tab,

    #[token("(")]
    LeftParen,

    #[token(")")]
    RightParen,

    #[token("[")]
    LeftBracket,

    #[token("]")]
    RightBracket,

    #[token("{")]
    LeftBrace,

    #[token("}")]
    RightBrace,

    #[token(",")]
    Comma,

    #[token(".")]
    Dot,

    #[token("~")]
    Tilde,

    #[token("`")]
    Grave,

    #[token("#")]
    Pound,

    #[token("+")]
    Plus,

    #[token("++")]
    PlusPlus,

    #[token("-")]
    Minus,

    #[token("--")]
    MinusMinus,

    #[token("*")]
    Star,

    #[token("/")]
    Slash,

    #[token("%")]
    Mod,

    #[token("!")]
    Bang,

    #[token("=")]
    Equal,

    #[token(">")]
    Greater,

    #[token(">>")]
    GreaterGreater,

    #[token("<")]
    Less,

    #[token("<<")]
    LessLess,

    #[token("&")]
    Amp,

    #[token("&&")]
    AmpAmp,

    #[token("|")]
    Pipe,

    #[token("||")]
    PipePipe,

    #[token("^")]
    Xor,

    #[token(":")]
    Colon,

    #[regex("r[0-9]", |lex| lex.slice()[1..].parse::<u8>().unwrap())]
    Register(u8),

    #[regex("'([^\\']|\\.)'", |lex| lex.slice().chars().nth(1).unwrap())]
    CharLit(char),

    #[regex("\"([^\\\"]|\\.)*\"", |lex| lex.slice()[1..lex.slice().len()-1].to_string())]
    StringLit(String),

    #[regex(r"0[xX][0-9a-fA-F]+", |lex| i64::from_str_radix(&lex.slice()[2..], 16).unwrap())]
    HexLit(i64),

    #[regex(r"0[bB][01]+", |lex| i64::from_str_radix(&lex.slice()[2..], 2).unwrap())]
    BinLit(i64),

    #[regex(r"0[oO][0-7]+", |lex| i64::from_str_radix(&lex.slice()[2..], 8).unwrap())]
    OctLit(i64),

    #[regex(r"[0-9]+", |lex| lex.slice().parse::<i64>().unwrap())]
    IntLit(i64),

    #[regex(r"macro_rules!\s+[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    MacroDef(String),

    #[regex("[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Ident(String),

    #[regex("%[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice()[1..].to_string())]
    MacroIdent(String),

    #[regex(";.*", logos::skip)]
    Comment,

    Macro(MacroContent),
}

#[derive(Debug)]
pub struct LexerError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Error at line {}:{} - {}",
            self.line, self.column, self.message
        )
    }
}

pub fn lex(input: &str) -> Result<Vec<TokenKind>, LexerError> {
    let mut lexer = TokenKind::lexer(input);
    let mut tokens = Vec::new();
    let mut line = 1;
    let mut column = 1;

    while let Some(token) = lexer.next() {
        // begin token iteration here
        match token {
            Ok(TokenKind::Newline) => {
                line += 1;
                column = 1;
                tokens.push(TokenKind::Newline);
            }
            Ok(TokenKind::Whitespace) | Ok(TokenKind::Tab) => {
                column += lexer.slice().len();
            }
            Ok(TokenKind::MacroDef(macro_def)) => {
                // O.o macro spotted
                let name = macro_def
                    .split_whitespace()
                    .nth(1)
                    .ok_or_else(|| LexerError {
                        message: "Macro definition should have a name".to_string(),
                        line,
                        column,
                    })?
                    .to_string();
                'macro_loop: loop {
                    // start collecting goodies in the macro :3
                    match lexer.next() {
                        Some(Ok(TokenKind::Tab)) | Some(Ok(TokenKind::Whitespace)) => {
                            continue;
                        }
                        Some(Ok(TokenKind::LeftParen)) => {
                            // O.o look, macro arguments!
                            let mut args = Vec::new();
                            loop {
                                // look through macro arguments
                                match lexer.next() {
                                    Some(Ok(TokenKind::Tab))
                                    | Some(Ok(TokenKind::Whitespace))
                                    | Some(Ok(TokenKind::Comma)) => {
                                        continue;
                                    }
                                    Some(Ok(TokenKind::Ident(arg_name))) => 'outer: loop {
                                        // teehee,
                                        // found an argument
                                        match lexer.next() {
                                            Some(Ok(TokenKind::Tab))
                                            | Some(Ok(TokenKind::Whitespace)) => {
                                                continue;
                                            }
                                            Some(Ok(TokenKind::Colon)) => loop {
                                                match lexer.next() {
                                                    Some(Ok(TokenKind::Tab))
                                                    | Some(Ok(TokenKind::Whitespace)) => {
                                                        continue;
                                                    }
                                                    Some(Ok(TokenKind::Ident(arg_type_str))) => {
                                                        let arg_type =
                                                            ArgumentType::from_str(&arg_type_str)
                                                                .ok_or_else(|| LexerError {
                                                                message: format!(
                                                                    "Invalid argument type: {}",
                                                                    arg_type_str
                                                                ),
                                                                line,
                                                                column,
                                                            })?;
                                                        args.push(FullArgument {
                                                            name: arg_name.to_string(),
                                                            arg_type,
                                                        });
                                                        break 'outer;
                                                    }
                                                    _ => {
                                                        return Err(LexerError {
                                                            message:
                                                                "Expected argument type after colon"
                                                                    .to_string(),
                                                            line,
                                                            column,
                                                        });
                                                    }
                                                }
                                            },
                                            _ => {
                                                return Err(LexerError {
                                                    message: "Expected colon after argument name"
                                                        .to_string(),
                                                    line,
                                                    column,
                                                });
                                            }
                                        }
                                    },
                                    Some(Ok(TokenKind::RightParen)) => break,
                                    _ => {
                                        return Err(LexerError {
                                            message: "Invalid macro argument syntax".to_string(),
                                            line,
                                            column,
                                        });
                                    }
                                }
                            }
                            loop {
                                match lexer.next() {
                                    Some(Ok(TokenKind::Tab)) | Some(Ok(TokenKind::Whitespace)) => {
                                        continue;
                                    }
                                    Some(Ok(TokenKind::LeftBrace)) => {
                                        let mut brace_count = 1;
                                        let mut macro_tokens = Vec::new();

                                        for tok in lexer.by_ref() {
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
                                                    return Err(LexerError {
                                                        message: "Invalid token in macro body"
                                                            .to_string(),
                                                        line,
                                                        column,
                                                    });
                                                }
                                            }
                                        }
                                        tokens.push(TokenKind::Macro(MacroContent {
                                            name,
                                            args,
                                            tokens: macro_tokens,
                                        }));
                                        break 'macro_loop;
                                    }
                                    _ => {
                                        return Err(LexerError {
                                            message: "Expected open brace to start macro body"
                                                .to_string(),
                                            line,
                                            column,
                                        });
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(LexerError {
                                message: "Expected open paren after macro name".to_string(),
                                line,
                                column,
                            });
                        }
                    }
                }
            }
            Ok(t) => {
                tokens.push(t);
                column += lexer.slice().len();
            }
            _ => {
                return Err(LexerError {
                    message: "Unexpected token".to_string(),
                    line,
                    column,
                });
            }
        }
    }

    Ok(tokens)
}

fn main() {
    let input_string = r#"macro_rules! my_macro (arg1 : reg, arg2 : imm) {
    mov %arg1, %arg2
}"#;
    println!("{input_string}");
    match lex(input_string) {
        Ok(tokens) => println!("{}", serde_json::to_string_pretty(&tokens).unwrap()),
        Err(e) => println!("Error: {}", e),
    }
}
