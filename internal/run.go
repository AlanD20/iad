package internal

import (
	"fmt"
	"os"
	"strings"

	"github.com/aland20/interpreter/internal/utils"
	"github.com/chzyer/readline"
)

func RunRepl() {
	input, err := readline.NewEx(&readline.Config{
		Prompt:          ">> ",
		HistoryFile:     "/tmp/readline.tmp",
		InterruptPrompt: "^C",
		EOFPrompt:       "exit",

		HistorySearchFold: true,
	})

	if err != nil {
		fmt.Println("Something went wrong!")
		panic(err.Error())
	}

	defer input.Close()
	input.CaptureExitSignal()

	for {
		line, err := input.Readline()
		if err != nil { // io.EOF
			break
		}
		line = strings.TrimSpace(line)

		switch strings.ToLower(line) {
		case "clear":
			readline.ClearScreen(input)
		case "exit", "quit", ".q", ".exit":
			input.Close()
		default:
			Run(line)
		}
	}

}

func RunFile(input string, output string) {

	// Validate input file
	path, err := utils.ValidateFilePath(input)

	if err != nil {
		panic(err.Error())
	}

	source, err := os.ReadFile(path)

	if err != nil {
		panic(err.Error())
	}

	Run(string(source))
}

func Run(source string) {
	scanner := NewScanner(source)
	tokens := scanner.ScanTokens()

	for _, token := range tokens {
		fmt.Println(token.Word)
	}
}
