use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct CLI {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Init { path: Option<String> },
}

fn main() {
    let cli = CLI::parse();

    match &cli.command {
        None => println!("No command"),
        Some(Commands::Init { path }) => {
            println!("Init command with path: {:?}", path);
        }
    }
}
