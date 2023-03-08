package internal

import (
	"fmt"
)

type TokenType byte
type TokenValue interface {
	string | float64
}

const (
	// Single-character tokens
	LEFT_PAREN TokenType = iota
	RIGHT_PAREN
	LEFT_BRACE
	RIGHT_BRACE
	COMMA
	DOT
	MINUS
	PLUS
	SEMICOLON
	SLASH
	STAR

	// One or two character tokens
	BANG
	BANG_EQUAL
	EQUAL
	EQUAL_EQUAL
	GREATER
	GREATER_EQUAL
	LESS
	LESS_EQUAL

	// Literals
	BOOL
	IDENTIFIER
	NUMBER
	STRING

	// Keywords/Identifiers
	AND
	CLASS
	CONST
	ELSE
	EXTENDS
	FALSE
	FOR
	FUNC
	IF
	LET
	NULL
	OR
	PRINT
	RETURN
	THIS
	TRUE

	EOF
)

var keywords = map[string]TokenType{

	"and":     AND,
	"class":   CLASS,
	"const":   CONST,
	"else":    ELSE,
	"extends": EXTENDS,
	"false":   FALSE,
	"for":     FOR,
	"func":    FUNC,
	"if":      IF,
	"let":     LET,
	"null":    NULL,
	"or":      OR,
	"print":   PRINT,
	"return":  RETURN,
	"this":    THIS,
	"true":    TRUE,
}

type Token struct {
	TokenType TokenType
	Word      string // individual word
	Value     any    // individual word
	Line      uint16
}

func NewToken(tokenType TokenType, word string, value string, line uint16) Token {

	return Token{
		TokenType: tokenType,
		Word:      word,
		Value:     value,
		Line:      line,
	}
}

func (t Token) String() string {
	return fmt.Sprintf("%v %v %v", t.TokenType, t.Word, t.Value)
}

func GetIdentifier(text string) TokenType {

	keyword, exists := keywords[text]

	if !exists {
		return IDENTIFIER
	}

	return keyword
}
