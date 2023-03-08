package expr

type Literal struct {
	Value any
}

func NewLiteral(value any) Expr {
	return &Literal{
		Value: value,
	}
}

func (l *Literal) Accept(v Visitor) any {
	return v.VisitLiteralExpr(l)
}
