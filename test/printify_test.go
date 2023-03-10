package test

import (
	"testing"

	"github.com/aland20/interpreter/internal"
	"github.com/aland20/interpreter/internal/expr"
)

func TestString(t *testing.T) {

	left := expr.NewUnary(
		internal.NewToken(internal.MINUS, "-", "", 1),
		expr.NewLiteral(123),
	)
	right := expr.NewGrouping(expr.NewLiteral(45.67))

	customExpression := expr.NewBinary(
		left,
		internal.NewToken(internal.STAR, "*", "", 1),
		right,
	)

	pf := &expr.PrintifyVisitor{}
	result := pf.String(customExpression)

	Equals(t, result, "( * ( - 123 ) ( group  45.67 ) )")
}
