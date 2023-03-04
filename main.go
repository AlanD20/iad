package main

import (
	"fmt"
	"os"

	"github.com/aland20/iad/cmd"
)

func main() {

	if err := cmd.Execute(); err != nil {
		fmt.Println("Something went wrong!")
		os.Exit(0)
	}
}
