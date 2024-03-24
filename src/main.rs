use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use clap::{Parser, Subcommand};

use rusty_safe::*;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// List items
    List,
    /// Show item
    Show { name: Option<String> },
    /// Generate password
    Generate,
}

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    if let Some(config_path) = cli.config.as_deref() {
        println!("Config file: {}", config_path.display());
    }

    let file = File::open("passwords.json")?;
    let buf_reader = BufReader::new(file);
    let items: Vec<Item> = serde_json::from_reader(buf_reader)?;

    match &cli.command {
        Some(Commands::List) => {
            println!("Listing all items...");
            items
                .into_iter()
                .for_each(|item| println!("{}", item.get_name()));
        }
        Some(Commands::Show { name }) => match name {
            None => println!("Please specify an item"),
            Some(name) => match items.into_iter().find(|item| item.get_name().eq(name)) {
                None => println!("Item not found"),
                Some(item) => println!("{}", item),
            },
        },
        Some(Commands::Generate) => {
            let config = GeneratorConfig::new(PasswordType::PassWord, 8, true, true, true, true);
            let password = generate_password(&config);
            println!("Your new password: {}", password);
        }
        None => {}
    }

    Ok(())
}
