use crate::token::Token;

use super::{Expr, ExprVisitor, Visitor};

pub struct Unary {
    pub operator: Token,
    pub right: Box<dyn Expr>,
}

impl Unary {
    pub fn new(operator: Token, right: Box<dyn Expr>) -> Self {
        Unary { operator, right }
    }
}

impl Visitor<&dyn ExprVisitor<()>> for Unary {
    fn accept(&self, visitor: &dyn ExprVisitor<()>) {
        visitor.visit_unary_expr(self);
    }
}
