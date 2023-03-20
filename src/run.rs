use std::{io::Write, path::PathBuf};

use crate::scanner::Scanner;

pub fn run(source: &[u8]) {
    let mut scanner = Scanner::new(&source);

    match scanner.scan_tokens() {
        Ok(tokens) => {
            for token in tokens.iter() {
                println!("{}", token.word);
            }
        }
        Err(errors) => {
            for error in errors.iter() {
                println!("{}", error.to_string());
            }
        }
    }
}

pub fn file(input: &String, output: Option<&String>) -> Result<(), Box<dyn std::error::Error>> {
    let mut path = PathBuf::new();
    path.push(std::env::current_dir()?);
    path.push(input);

    let content = std::fs::read(&path)?;

    println!("output will be at: {:?}", output);
    run(&content);

    Ok(())
}

pub fn repl() -> Result<(), Box<dyn std::error::Error>> {
    loop {
        let line = readline()?;
        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        if line == "quit" || line == "exit" || line == ".q" {
            break;
        }

        run(line.as_bytes());
    }

    Ok(())
}

fn readline() -> Result<String, String> {
    write!(std::io::stdout(), ">> ").map_err(|e| e.to_string())?;
    std::io::stdout().flush().map_err(|e| e.to_string())?;
    let mut buffer = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .map_err(|e| e.to_string())?;
    Ok(buffer)
}
