use crate::token::Token;

use super::{Expr, ExprVisitor, Visitor};

pub struct Binary {
    pub left: Box<dyn Expr>,
    pub operator: Token,
    pub right: Box<dyn Expr>,
}

impl Binary {
    pub fn new(left: Box<dyn Expr>, operator: Token, right: Box<dyn Expr>) -> Self {
        Binary {
            left,
            operator,
            right,
        }
    }
}

impl Visitor<&dyn ExprVisitor<()>> for Binary {
    fn accept(&self, visitor: &dyn ExprVisitor<()>) {
        visitor.visit_binary_expr(self);
    }
}
