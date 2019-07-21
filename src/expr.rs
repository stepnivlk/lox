use crate::token::{Token, Literal};

pub trait Visitor {
    type Result;

    fn visit(&self, expr: &Expr) -> Self::Result;
}

pub trait Acceptor {
    fn accept<V: Visitor>(&self, visitor: V) -> V::Result;
}

pub enum Expr {
    Binary { left: Box<Expr>, operator: Token, right: Box<Expr> },
    Grouping { expression: Box<Expr> },
    Literal { value: Option<Literal> },
    Unary { operator: Token, right: Box<Expr> },
}

impl Acceptor for Expr {
    fn accept<V: Visitor>(&self, visitor: V) -> V::Result {
        visitor.visit(self)
    }
}
