package utils

import (
	"errors"
	"os"
)

func ValidateFilePath(input string) (string, error) {

	// Handle empty input file if necessary
	if input == "" {
		return "", errors.New("Input file is required")
	}

	pwd, _ := os.Getwd()
	path := pwd + "/" + input
	inputFile, err := os.Stat(path)

	// Make sure path exists
	if err != nil {
		return "", errors.New("Entry file is invalid")
	}

	// Make sure is file
	if inputFile.IsDir() {
		return "", errors.New("Entry must be a file")
	}

	return path, nil
}
