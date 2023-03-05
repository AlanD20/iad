package internal

import (
	"fmt"
	"strconv"

	"github.com/aland20/iad/internal/utils"
)

type Scanner struct {
	Source  string // The consumed line
	Tokens  []Token
	Start   uint16 // Start of each token
	Current uint16 // Current char position in the line
	Line    uint16
}

func NewScanner(source string) *Scanner {

	return &Scanner{
		Source: source,
	}
}

func (s *Scanner) ScanTokens() []Token {

	for !s.isAtEnd() {
		s.Start = s.Current
		s.ScanToken()
	}

	s.Tokens = append(s.Tokens, Token{
		TokenType: EOF,
		Word:      "",
		Value:     "",
		Line:      s.Line,
	})

	return s.Tokens
}

func (s *Scanner) ScanToken() {

	c := s.peek()

	switch c {
	// Skip unnecessary lines
	case ' ', '\r', '\t':
		break
	case '\n':
		s.advanceLine()
		// Basic operations & chars
	case '(':
		s.addToken(LEFT_PAREN, "")
	case ')':
		s.addToken(RIGHT_PAREN, "")
	case '{':
		s.addToken(LEFT_BRACE, "")
	case '}':
		s.addToken(RIGHT_BRACE, "")
	case ',':
		s.addToken(COMMA, "")
	case '.':
		s.addToken(DOT, "")
	case ';':
		s.addToken(SEMICOLON, "")
	case '+':
		s.addToken(PLUS, "")
	case '-':
		s.addToken(MINUS, "")
	case '*':
		s.addToken(STAR, "")
		// Multiple Characters
	case '!':
		token := BANG
		if s.match('=') {
			token = BANG_EQUAL
		}
		s.addToken(token, "")
	case '=':
		token := EQUAL
		if s.match('=') {
			token = EQUAL_EQUAL
		}
		s.addToken(token, "")
	case '>':
		token := GREATER
		if s.match('=') {
			token = GREATER_EQUAL
		}
		s.addToken(token, "")
	case '<':
		token := LESS
		if s.match('=') {
			token = LESS_EQUAL
		}
		s.addToken(token, "")
		// Slash and Comment
	case '/':
		if s.match('/') {
			for s.peek() != '\n' && !s.isAtEnd() {
				s.advance()
			}
		} else {
			s.addToken(SLASH, "")
		}
		// String literals
	case '"':
		s.string()
	default:
		// Number Literal
		if utils.IsDigit(c) {
			s.number()
		} else if utils.IsAlpha(c) {
			s.identifier()
		} else {
			// utils.Error(s.getLine(), "Unexpected character")
			utils.Report(s.getLine(), s.getCurrent(), "Unexpected character")
		}
	}

	// Advance to next character
	s.advance()
}

func (s *Scanner) isAtEnd() bool {
	return s.Current >= uint16(len(s.Source))
}

func (s *Scanner) advance() {
	s.Current = s.Current + 1
}

func (s *Scanner) advanceLine() {
	s.Line = s.Line + 1
}

func (s *Scanner) getLine() string {
	return fmt.Sprint(s.Line)
}

func (s *Scanner) getCurrent() string {
	return fmt.Sprint(s.Current)
}

func (s *Scanner) addToken(tokenType TokenType, value any) {

	end := s.Current
	if s.Start == s.Current {
		end = s.Current + 1
	}

	word := s.Source[s.Start:end]

	s.Tokens = append(s.Tokens, Token{
		TokenType: tokenType,
		Word:      word,
		Value:     value,
		Line:      s.Line,
	})
}

func (s *Scanner) match(expected byte) bool {

	if s.isAtEnd() {
		return false
	}

	if s.peekNext() != expected {
		return false
	}

	s.Current = s.Current + 1
	return true
}

func (s *Scanner) peek() byte {
	if s.isAtEnd() {
		return 0
	}

	return s.Source[s.Current]
}

func (s *Scanner) peekNext() byte {
	if s.Current+1 >= uint16(len(s.Source)) {
		return 0
	}

	return s.Source[s.Current+1]
}

func (s *Scanner) string() {
	for s.peekNext() != '"' && !s.isAtEnd() {
		// Supports multi-line string values
		// We have to advance when we hit new line
		if s.peek() == '\n' {
			s.advanceLine()
		}
		s.advance()
	}

	// Must be closed in one line
	if s.isAtEnd() {
		utils.Error(s.getLine(), "Missing string quote")
	}

	// Closing quote
	s.advance()
	// Trim surrounding quotes
	s.Start = s.Start + 1
	value := s.Source[s.Start:s.Current]
	// Add Token with value
	s.addToken(STRING, value)
}

func (s *Scanner) number() {
	for utils.IsDigit(s.peek()) {
		s.advance()
	}

	// Look for fractional part
	if s.peek() == '.' && utils.IsDigit(s.peekNext()) {
		// Consume the "."
		s.advance()

		for utils.IsDigit(s.peek()) {
			s.advance()
		}
	}
	value := s.Source[s.Start:s.Current]
	realValue, err := strconv.ParseFloat(value, 64)

	if err != nil {
		utils.Error(s.getLine(), "invalid number")
	}

	s.addToken(NUMBER, realValue)
}

func (s *Scanner) identifier() {
	for utils.IsAlphaNumeric(s.peek()) {
		s.advance()
	}

	text := s.Source[s.Start:s.Current]
	token := GetIdentifier(text)
	s.addToken(token, "")
}
