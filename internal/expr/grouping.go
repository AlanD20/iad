package expr

type Grouping struct {
	Expression Expr
}

func NewGrouping(expression Expr) Expr {
	return &Grouping{
		Expression: expression,
	}
}

func (g *Grouping) Accept(v Visitor) any {
	return v.VisitGroupingExpr(g)
}
