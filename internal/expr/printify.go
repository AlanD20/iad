package expr

import (
	"fmt"
	"strings"
)

type PrintifyVisitor struct{}

func (v PrintifyVisitor) String(expression Expr) string {
	return expression.Accept(v).(string)
}

func (v PrintifyVisitor) Print(expression Expr) {
	fmt.Println("Expr:", v.String(expression))
}

func (v PrintifyVisitor) VisitBinaryExpr(b *Binary) any {
	return v.parenthesize(b.Operator.Word, b.Left, b.Right)
}

func (v PrintifyVisitor) VisitGroupingExpr(g *Grouping) any {
	return v.parenthesize("group ", g.Expression)
}

func (v PrintifyVisitor) VisitLiteralExpr(l *Literal) any {
	if l.Value == "" {
		return "null"
	}

	return fmt.Sprintf("%v", l.Value)
}

func (v PrintifyVisitor) VisitUnaryExpr(u *Unary) any {
	return v.parenthesize(u.Operator.Word, u.Right)
}

func (v PrintifyVisitor) parenthesize(name string, exprs ...Expr) any {
	var syntax []string

	syntax = append(syntax, "(", name)
	for _, exp := range exprs {
		syntax = append(syntax, exp.Accept(v).(string))
	}
	syntax = append(syntax, ")")

	return strings.Join(syntax, " ")
}
