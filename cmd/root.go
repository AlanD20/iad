package cmd

import (
	"github.com/aland20/interpreter/internal"
	"github.com/spf13/cobra"
)

func getRootCommand() *cobra.Command {

	var input, output string

	var rootCmd = &cobra.Command{
		Use:   "interpreter",
		Short: "interpreter is a simple scripting language",
		Long:  `Learn more at https://github.com/AlanD20/interpreter`,
		Run: func(cmd *cobra.Command, args []string) {

			if input != "" {
				// Process input file
				internal.RunFile(input, output)
			} else {
				internal.RunRepl()
			}
		},
	}

	rootCmd.Flags().StringVarP(&input, "input", "i", "", "Entry file path for interpreter.")

	rootCmd.Flags().StringVarP(&output, "output", "o", "", "Output file path.")

	return rootCmd
}

func Execute() error {

	rootCmd := getRootCommand()
	// rootCmd.AddCommand(NewExampleCommand())

	return rootCmd.Execute()
}
