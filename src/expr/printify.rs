use super::binary::Binary;
use super::grouping::Grouping;
use super::literal::Literal;
use super::unary::Unary;
use super::{Expr, ExprVisitor};

pub struct Printify {}

impl Printify {
    fn parenthesize(&self, name: String, expressions: Vec<Box<dyn Expr>>) -> String {
        let mut syntax: Vec<String> = vec![];

        syntax.push(format!("( {}", name));
        for expr in expressions.iter() {
            syntax.push(expr.accept(self));
        }
        syntax.push(format!(" )"));

        return syntax.join(" ");
    }
}

impl ExprVisitor<String> for Printify {
    fn visit_binary_expr(&self, binary: &Binary) -> String {
        "binary".to_string()
    }
    fn visit_grouping_expr(&self, grouping: &Grouping) -> String {
        "grouping".to_string()
    }
    fn visit_literal_expr(&self, literal: &Literal) -> String {
        "literal".to_string()
    }
    fn visit_unary_expr(&self, unary: &Unary) -> String {
        let right = vec![unary.right];
        return self.parenthesize(unary.operator.word, right);
    }
}
