use tabled::{Style, Table};
use unqlite::UnQLite;

#[allow(unused)]
use clap::{Parser, Subcommand};

use crate::database::{create, delete, filter, list, read, Entry};
use crate::utils::random_pass;

mod database;
mod utils;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Cli {
    /// Domain to search
    #[clap(value_parser)]
    domain: Option<String>,

    /// Subcommands
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Create or Update password for a domain
    Add {
        /// Name of domain
        #[clap(short, long, action, value_parser)]
        domain: String,

        /// User for login
        #[clap(short, long, action, value_parser)]
        user: String,

        /// Password
        /// If it is not provided a random strong password will be generated
        #[clap(short, long, action, value_parser)]
        pass: Option<String>,
    },

    /// Read entry from db
    Read {
        /// Name of domain
        #[clap(short, long, action, value_parser)]
        domain: String,
    },

    /// List all entries for the db
    Ls {},

    /// Remove entry from db
    Rm {
        /// Name of domain
        #[clap(short, long, action, value_parser)]
        domain: String,
    },
}

fn main() {
    // The database memory is not handled by Rust, and the database is on-disk,
    // so `mut` is not neccessary.
    let unqlite = UnQLite::create("./test.db");

    // Parsing Cli commands
    let cli = Cli::parse();

    let table: Vec<Entry> = if let Some(domain) = cli.domain.as_deref() {
        println!("Searching for domain: {}", domain);
        filter(String::from(domain), unqlite)
    } else {
        match &cli.command {
            Some(Commands::Ls {}) => {
                println!("LS");
                list(unqlite)
            }

            Some(Commands::Add { domain, user, pass }) => {
                println!("ADD");

                let pass: String = match pass {
                    Some(p) => p.to_owned(),
                    None => {
                        println!("Creating entry with random password");
                        random_pass()
                    }
                };
                let key = String::from(domain);
                let e = Entry {
                    domain: key.clone(),
                    user: String::from(user),
                    pass,
                };

                create(key, e, unqlite)
            }

            Some(Commands::Read { domain }) => {
                println!("READ");
                read(String::from(domain), unqlite)
            }

            Some(Commands::Rm { domain }) => {
                println!("RM");
                delete(String::from(domain), unqlite)
            }

            None => {
                list(unqlite)
            }
        }
    };

    println!("{}", Table::new(table).with(Style::modern()));
}
