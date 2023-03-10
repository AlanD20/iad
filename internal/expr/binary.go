package expr

import "github.com/aland20/interpreter/internal"

type Binary struct {
	Left     Expr
	Operator internal.Token
	Right    Expr
}

func NewBinary(left Expr, operator internal.Token, right Expr) Expr {
	return &Binary{
		Left:     left,
		Operator: operator,
		Right:    right,
	}
}

func (b *Binary) Accept(v Visitor) any {
	return v.VisitBinaryExpr(b)
}
