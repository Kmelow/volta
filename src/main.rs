use database::list;
use tabled::{Style, Table};
use unqlite::UnQLite;

#[allow(unused)]
use clap::{Parser, Subcommand};

use crate::database::{create, delete, read, Entry};
use crate::utils::random_pass;

mod database;
mod utils;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Cli {
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

    let table: Vec<Entry> = match &cli.command {
        Some(Commands::Ls {}) => {
            list(unqlite)
        }

        Some(Commands::Add { domain, user, pass }) => {
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
            read(String::from(domain), unqlite)
        }

        Some(Commands::Rm { domain }) => {
            delete(String::from(domain), unqlite)
        }

        None => vec![Entry{domain: String::from(""), user: String::from(""), pass: String::from("")}]
    };

    println!("{}", Table::new(table).with(Style::modern()));
}
