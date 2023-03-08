package expr

import "github.com/aland20/iad/internal"

type Unary struct {
	Operator internal.Token
	Right    Expr
}

func NewUnary(operator internal.Token, right Expr) Expr {
	return &Unary{
		Operator: operator,
		Right:    right,
	}
}

func (u *Unary) Accept(v Visitor) any {
	return v.VisitUnaryExpr(u)
}
