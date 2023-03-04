package internal

import (
	"fmt"

	"github.com/aland20/iad/internal/utils"
)

func Run(input string, output string) {

	// Validate input file
	path, err := utils.ValidateFilePath(input)

	if err != nil {
		fmt.Println(err.Error())
		return
	}

	fmt.Println("path: ", path)
}
