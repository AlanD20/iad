package expr

type Visitor interface {
	VisitBinaryExpr(b *Binary) any
	VisitGroupingExpr(g *Grouping) any
	VisitLiteralExpr(l *Literal) any
	VisitUnaryExpr(u *Unary) any
}

type Expr interface {
	Accept(Visitor) any
}

type ExprVisitor struct{}

func (v ExprVisitor) VisitBinaryExpr(b *Binary) any {
	return "binary"
}

func (v ExprVisitor) VisitGroupingExpr(g *Grouping) any {
	return "grouping"
}

func (v ExprVisitor) VisitLiteralExpr(l *Literal) any {
	return "lietral"
}

func (v ExprVisitor) VisitUnaryExpr(u *Unary) any {
	return "unary"
}
