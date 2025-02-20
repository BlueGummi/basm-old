use std::ops::{Add, Mul, Shl, Shr, Sub};

#[derive(Debug)]
pub enum Expr {
    Int(i64),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Shl(Box<Expr>, Box<Expr>),
    Shr(Box<Expr>, Box<Expr>),
}

impl Expr {
    pub fn evaluate(&self) -> i64 {
        match self {
            Expr::Int(n) => *n,
            Expr::Add(lhs, rhs) => lhs.evaluate().add(rhs.evaluate()),
            Expr::Sub(lhs, rhs) => lhs.evaluate().sub(rhs.evaluate()),
            Expr::Mul(lhs, rhs) => lhs.evaluate().mul(rhs.evaluate()),
            Expr::Shl(lhs, rhs) => lhs.evaluate().shl(rhs.evaluate() as u32),
            Expr::Shr(lhs, rhs) => lhs.evaluate().shr(rhs.evaluate() as u32),
        }
    }
}
