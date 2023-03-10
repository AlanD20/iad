# Documentation

The documentation assumes you have installed the `interpreter` and you have access to the executable by using `interpreter` in your CLI.

## Installation

You may build the project by cloning the project locally and make sure you have installed `go-1.20`. To build the project, run the following command:

```bash
go build -o interpreter main.go
```

Now, you have access to the interpreter and continue with the documentaiton.

## CLI

- **REPL Mode**: An empty arguments or flags will put you into REPL mode.

- **Run Script File**: You may use `run` command to provide a script file.

  ```bash
    interpreter run my-script.iad
    interpreter run my-script.iad -o out
  ```
