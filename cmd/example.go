package cmd

import (
	"github.com/spf13/cobra"
)

func NewExampleCommand() *cobra.Command {

	var check bool

	exampleCommand := &cobra.Command{
		Use:              "example",
		Short:            "long description",
		TraverseChildren: true,
		Run: func(cmd *cobra.Command, args []string) {

			if check {
				isDone := make(chan bool)
				go func() {
					isDone <- true
				}()

				<-isDone
			}

			// Do your thing
		},
	}

	// Using flags
	exampleCommand.Flags().BoolVarP(&check, "check", "c", false, "is true.")

	return exampleCommand
}
