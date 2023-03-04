package utils

import "fmt"

func Error(line string, message string) {

	Report(line, "", message)
}

func Report(line string, where string, message string) bool {

	fmt.Printf("[line %s] Error %s: %s \n", line, where, message)
	return true
}
