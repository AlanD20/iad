package test

import (
	"strings"
	"testing"

	"github.com/aland20/iad/internal"
)

func TestScanner(t *testing.T) {

	codeLine := "let name = \"aland\";"
	scanner := internal.NewScanner(codeLine)
	tokens := scanner.ScanTokens()

	originalToken := []string{}
	for _, token := range tokens {
		originalToken = append(originalToken, token.Word)
	}

	Equals(t, len(tokens), 6) // Including EOF TokenType
	// Joining by space, will add extra spaces between semi-colon
	Equals(t, strings.Join(originalToken, " "), "let name = aland ; ")

}
