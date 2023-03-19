use clap::{Arg, ArgAction, ArgMatches, Command};

use crate::run;

pub struct Cli {
    cli: ArgMatches,
}

impl Cli {
    pub fn new() -> Cli {
        let cli = Command::new(env!("CARGO_PKG_NAME"))
            .author(env!("CARGO_PKG_AUTHORS"))
            .about(env!("CARGO_PKG_DESCRIPTION"))
            .version(env!("CARGO_PKG_VERSION"))
            .subcommand_required(false)
            .subcommand(run_command())
            .get_matches();

        return Cli { cli };
    }

    pub fn parse(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self.cli.subcommand() {
            Some(("run", run)) => {
                let mut output: Option<&String> = None;

                if let Some(output_path) = run.get_one::<String>("output") {
                    output = Some(output_path);
                }

                if let Some(input) = run.get_one::<String>("input") {
                    run::file(input, output)?;
                }

                println!("Script executed!");

                Ok(())
            }
            _ => run::repl(),
        }
    }
}

fn run_command() -> Command {
    Command::new("run")
        .long_flag("run")
        .about("Take input script to execute or output the result")
        .arg(
            Arg::new("input")
                .short('i')
                .action(ArgAction::Set)
                .num_args(1)
                .required(true),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .action(ArgAction::Set)
                .num_args(1)
                .required(false),
        )
}
