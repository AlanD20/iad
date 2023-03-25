use crate::token::TokenValue;

use super::{ExprVisitor, Visitor};

pub struct Literal {
    pub value: TokenValue,
}

impl Literal {
    pub fn new(value: TokenValue) -> Self {
        Literal { value }
    }
}

impl Visitor<&dyn ExprVisitor<()>> for Literal {
    fn accept(&self, visitor: &dyn ExprVisitor<()>) {
        visitor.visit_literal_expr(self);
    }
}
