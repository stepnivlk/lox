use crate::expr::{Acceptor, Expr, Visitor};

pub struct AstPrinter {}

impl AstPrinter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn print(&self, expr: &Expr) -> String {
        expr.accept(self)
    }

    fn parenthesize(&self, name: &str, exprs: Vec<&Expr>) -> String {
        let mut res = format!("({}", name);

        for expr in exprs {
            let part = format!(" {}", expr.accept(self));
            res.push_str(&part);
        }

        res.push_str(" )");

        res
    }
}

impl Visitor for &AstPrinter {
    type Result = String;

    fn visit(&self, expr: &Expr) -> Self::Result {
        match expr {
            Expr::Binary { left, operator, right } => self.parenthesize(
                &operator.lexeme,
                vec![left, right]
            ),
            Expr::Grouping { expression } => self.parenthesize(
                "group",
                vec![expression]
            ),
            Expr::Literal { value } => {
                match value {
                    Some(literal) => literal.to_string(),
                    None => "nil".to_string(),
                }
            },
            Expr::Unary { operator, right } => self.parenthesize(
                &operator.lexeme,
                vec![right]
            ),
        }
    }
}
