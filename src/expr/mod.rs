mod binary;
mod grouping;
mod literal;
mod printify;
mod unary;

use binary::Binary;
use grouping::Grouping;
use literal::Literal;
use unary::Unary;

// Other objects have to implement it
pub trait Visitor<T, R = ()> {
    fn accept(&self, visitor: T) -> R;
}

pub trait Expr<T = (), R = ()> {
    fn accept(&self, visitor: &dyn ExprVisitor<T>) -> R;
}

pub trait ExprVisitor<T> {
    fn visit_binary_expr(&self, binary: &Binary) -> T;
    fn visit_grouping_expr(&self, grouping: &Grouping) -> T;
    fn visit_literal_expr(&self, literal: &Literal) -> T;
    fn visit_unary_expr(&self, unary: &Unary) -> T;
}
