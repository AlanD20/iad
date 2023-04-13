package cmd

import (
	"github.com/aland20/interpreter/internal"
	"github.com/spf13/cobra"
)

func getRootCommand() *cobra.Command {

	var rootCommand = &cobra.Command{
		Use:   "interpreter",
		Short: "interpreter is a simple scripting language",
		Long:  `Learn more at https://github.com/AlanD20/interpreter`,
		Run: func(cmd *cobra.Command, args []string) {

			internal.RunRepl()
		},
	}

	return rootCommand
}

func Execute() error {

	rootCommand := getRootCommand()
	rootCommand.AddCommand(NewRunCommand())

	return rootCommand.Execute()
}
