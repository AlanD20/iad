use super::{Expr, ExprVisitor, Visitor};

pub struct Grouping {
    pub expression: Box<dyn Expr>,
}

impl Grouping {
    pub fn new(expression: Box<dyn Expr>) -> Self {
        Grouping { expression }
    }
}

impl Visitor<&dyn ExprVisitor<()>> for Grouping {
    fn accept(&self, visitor: &dyn ExprVisitor<()>) {
        visitor.visit_grouping_expr(self);
    }
}
