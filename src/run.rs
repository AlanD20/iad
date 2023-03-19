use std::{io::Write, path::PathBuf};

pub fn file(input: &String, output: Option<&String>) -> Result<(), Box<dyn std::error::Error>> {
    let mut path = PathBuf::new();
    path.push(std::env::current_dir()?);
    path.push(input);

    let content = std::fs::read_to_string(&path)?;

    println!("output will be at: {:?}", output);
    println!("here is file content: \n {}", content);

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

        print!("Line is okay to process: {}", line);
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
