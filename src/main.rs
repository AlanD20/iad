mod cmd;
mod run;
mod token;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = cmd::Cli::new();

    app.parse().unwrap_or_else(|e| println!("{}", e));

    Ok(())
}
