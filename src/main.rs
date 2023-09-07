use chrono::Datelike;
use clap::{Parser, Subcommand};

mod commands;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct CLI {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Init {
        path: Option<String>,
        year: Option<String>,
    },
}

#[derive(Debug)]
pub enum YearInput {
    Current,
    Specific(u16),
    All,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = CLI::parse();

    match &cli.command {
        None => Ok(println!("No command")),
        Some(Commands::Init { path, year }) => {
            let path = match &path {
                None => "./",
                Some(p) => p,
            };

            let year = match &year {
                None => YearInput::Current,
                Some(y) => {
                    if y == "all" {
                        YearInput::All
                    } else {
                        YearInput::Specific(y.parse().unwrap())
                    }
                }
            };

            let current_year = chrono::Utc::now().year() as u16;

            match year {
                YearInput::Specific(y) => {
                    assert!(
                        y >= 2015 && y <= current_year,
                        "Year must be between 2015 and the current year"
                    )
                }
                _ => {}
            }

            commands::init_command(path, year).await
        }
    }
}
