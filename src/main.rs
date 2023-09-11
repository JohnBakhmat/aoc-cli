use clap::{Parser, Subcommand};
#[allow(unused)]
mod add;
mod init;
mod pull;
mod temp;

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
    Add {
        lang: String,
        path: Option<String>,
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
    println!("\n\n");
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

            init::run(path, year).await
        }
        Some(Commands::Add { lang, path }) => {
            let path = match &path {
                None => "./",
                Some(p) => p,
            };

            add::run(path, lang).await
        }
    }
}
