package cmd

import (
	"fmt"
	"os"

	"github.com/aland20/interpreter/internal"
	"github.com/spf13/cobra"
)

func NewRunCommand() *cobra.Command {

	var output string

	runCommand := &cobra.Command{
		Use:              "run",
		Short:            "Execute a script file.",
		TraverseChildren: true,
		Run: func(cmd *cobra.Command, args []string) {

			if len(args) == 0 {
				fmt.Println("Please provide an input file")
				os.Exit(0)
			}

			// Process file
			internal.RunFile(args[0], output)
		},
	}

	runCommand.Flags().StringVarP(&output, "output", "o", "", "Output file path.")

	return runCommand
}
