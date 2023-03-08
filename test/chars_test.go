package test

import (
	"testing"

	"github.com/aland20/iad/internal/utils"
)

func TestIsDigit(t *testing.T) {

	digit := utils.IsDigit('4')
	notDigit := utils.IsDigit('f')
	notDigit2 := utils.IsDigit('_')

	Assert(t, digit, "Is Digit is True")
	Assert(t, !notDigit, "Is not Digit is True")
	Assert(t, !notDigit2, "Is not Digit is True")
}

func TestIsAlpha(t *testing.T) {

	alpha := utils.IsAlpha('g')
	alpha2 := utils.IsAlpha('_')
	notAlpha := utils.IsAlpha('5')

	Assert(t, alpha, "Is Alpha is True")
	Assert(t, alpha2, "Is Alpha is True")
	Assert(t, !notAlpha, "Is not Alpha is True")
}

func TestIsAlphaNumeric(t *testing.T) {

	alphaNumeric := utils.IsAlphaNumeric('g')
	alphaNumeric2 := utils.IsAlphaNumeric('_')
	alphaNumeric3 := utils.IsAlphaNumeric('5')
	notAlphaNumeric := utils.IsAlphaNumeric('*')
	notAlphaNumeric2 := utils.IsAlphaNumeric('-')

	Assert(t, alphaNumeric, "Is AlphaNumeric is True")
	Assert(t, alphaNumeric2, "Is AlphaNumeric is True")
	Assert(t, alphaNumeric3, "Is AlphaNumeric is True")
	Assert(t, !notAlphaNumeric, "Is not AlphaNumeric is True")
	Assert(t, !notAlphaNumeric2, "Is not AlphaNumeric is True")
}
