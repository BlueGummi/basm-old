use logos::Logos;
use serde::Serialize;

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
    pub fn from_str(s: &str) -> Option<Self> {
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
impl TokenKind {
    pub fn is_empty(&self) -> bool {
        matches!(self, TokenKind::Tab | TokenKind::Whitespace)
    }
}


