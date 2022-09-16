use crate::utils::{clip, mask_pass};
use owo_colors::{OwoColorize, Style};
use serde::{Deserialize, Serialize};
use slug::slugify;
use tabled::Tabled;
use unqlite::{Cursor, UnQLite, KV};

#[derive(Serialize, Deserialize, Tabled, Debug)]
pub struct Entry {
    pub domain: String,
    pub user: String,
    pub pass: String,
}

pub fn create(key: String, val: Entry, db: UnQLite) -> Vec<Entry> {
    let json = serde_json::to_string(&val).expect("Failed to serde");
    db.kv_store(slugify(key), json).expect("Failed to store");

    list(db)
}

pub fn read(key: String, db: UnQLite) -> Vec<Entry> {
    let db_value_json = db.kv_fetch(slugify(key)).expect("Error while reading");
    let db_value: String =
        serde_json::from_str(&String::from_utf8(db_value_json).expect("Failed to convert bytes"))
            .unwrap();
    println!("{}", db_value);

    list(db)
}

pub fn delete(key: String, db: UnQLite) -> Vec<Entry> {
    db.kv_delete(slugify(key))
        .expect("Key not found when deleting");

    list(db)
}

/// list returns all saved passowords in the database
pub fn list(db: UnQLite) -> Vec<Entry> {
    filter_list(None, db)
}

/// filter returns a vector of saved entries based on a `subs` substring
pub fn filter(subs: String, db: UnQLite) -> Vec<Entry> {
    filter_list(Some(subs), db)
}

/// filter_list retrieves all passwords in the database and filters according to parameter `subs`
fn filter_list(subs: Option<String>, db: UnQLite) -> Vec<Entry> {
    let mut entry = db.first();
    let mut entries: Vec<Entry> = Vec::new();

    loop {
        if entry.is_none() {
            break;
        }

        let record = entry.expect("valid entry");
        let (key, value) = record.key_value();

        let key_string = String::from_utf8(key).expect("Failed to retrieve key");

        match subs.clone() {
            Some(subs) => {
                if key_string.contains(&*subs) {
                    let stored: Entry =
                        serde_json::from_str(&*String::from_utf8(value).expect("Failed utf8"))
                            .expect("Failed serde");
                    entries.push(stored);
                }
            }
            None => {
                let stored: Entry =
                    serde_json::from_str(&*String::from_utf8(value).expect("Failed utf8"))
                        .expect("Failed serde");
                entries.push(stored);
            }
        }
        entry = record.next();
    }

    let style = Style::new().green().bold();

    if entries.len() == 1 {
        let clipped_pass = mask_pass(entries[0].pass[0..4].to_string(), Some(4));
        println!(
            "{} {}",
            "Copied password to clipboard 📋".style(style),
            clipped_pass
        );
        clip(entries[0].pass.clone())
    }

    entries
}
