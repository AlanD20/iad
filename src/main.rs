mod cmd;
mod error;
// mod expr;
mod run;
mod scanner;
mod token;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = cmd::Cli::new();

    app.parse().unwrap_or_else(|e| println!("{}", e));

    Ok(())
}
