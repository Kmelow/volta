use tabled::Tabled;
use serde::{Deserialize, Serialize};
use slug::slugify;
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

pub fn list(db: UnQLite) -> Vec<Entry> {
    let mut entry = db.first();
    let mut entries: Vec<Entry> = Vec::new();
    
    loop {
        if entry.is_none() {
            break;
        }

        let record = entry.expect("valid entry");
        let (_, value) = record.key_value();

        let stored: Entry = serde_json::from_str(&*String::from_utf8(value).expect("Failed utf8"))
            .expect("Failed serde");
        entries.push(stored);
        // println!("* Entry {:?} --> {:?}", String::from_utf8(key), stored);

        entry = record.next();
    }
    entries
}
