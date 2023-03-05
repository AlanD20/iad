package utils

import "regexp"

var reAlpha = regexp.MustCompile("[a-zA-Z_]")

func IsDigit(char byte) bool {
	return char >= '0' && char <= '9'
}

func IsAlpha(char byte) bool {
	return reAlpha.Match([]byte{char})
}

func IsAlphaNumeric(char byte) bool {
	return IsAlpha(char) || IsDigit(char)
}
