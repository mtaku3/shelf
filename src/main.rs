mod app;

use std::fs::OpenOptions;
use app::{store, Config, visit, Book, throw, read};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Store {
        category: String,
        tag: String,
        path: Option<String>,
    },
    Throw {
        category: String,
        tag: Option<String>,
    },
    Visit {
        category: Option<String>
    },
    Read {
        category: String,
        tag: String,
    },
}

fn main() {
    let cli = Cli::parse();

    let config = Config::new(None).unwrap();
    let does_config_file_exist: bool = match config.get_path().try_exists() {
        Ok(v) => v,
        Err(e) => {
            panic!("Unexpected error happened while checking if config file exists: {}", e);
        },
    };
    if !does_config_file_exist {
        match OpenOptions::new().create(true).write(true).open(config.get_path()) {
            Ok(_) => {},
            Err(e) => {
                panic!("Unexpected error happened while creating config file: {}", e);
            },
        };
    }

    match &cli.command {
        Commands::Store { category, tag, path } => {
            store(&config, category, tag, path.as_deref()).unwrap();
        },
        Commands::Visit { category } => {
            let books = visit(&config, category.as_deref()).unwrap();
            Book::print(&books).unwrap();
        },
        Commands::Throw { category, tag } => {
            throw(&config, category, tag.as_deref()).unwrap();
        },
        Commands::Read { category, tag } => {
            let book = read(&config, category, tag).unwrap();
            Book::print(&[book]).unwrap();
        },
    }
}
