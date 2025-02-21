#[derive(Debug)]
pub enum Expr {
    Int(i64),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Shl(Box<Expr>, Box<Expr>),
    Shr(Box<Expr>, Box<Expr>),
    BitAnd(Box<Expr>, Box<Expr>),
    BitOr(Box<Expr>, Box<Expr>),
    Xor(Box<Expr>, Box<Expr>),
}

impl Expr {
    pub fn evaluate(&self) -> i64 {
        match self {
            Expr::Int(n) => *n,
            Expr::Add(lhs, rhs) => lhs.evaluate() + rhs.evaluate(),
            Expr::Sub(lhs, rhs) => lhs.evaluate() - rhs.evaluate(),
            Expr::Mul(lhs, rhs) => lhs.evaluate() * rhs.evaluate(),
            Expr::Shl(lhs, rhs) => lhs.evaluate() << rhs.evaluate() as u32,
            Expr::Shr(lhs, rhs) => lhs.evaluate() >> rhs.evaluate() as u32,
            Expr::BitAnd(lhs, rhs) => lhs.evaluate() & rhs.evaluate(),
            Expr::BitOr(lhs, rhs) => lhs.evaluate() | rhs.evaluate(),
            Expr::Xor(lhs, rhs) => lhs.evaluate() ^ rhs.evaluate(),
        }
    }
}
