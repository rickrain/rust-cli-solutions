use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::{Error, ErrorKind, Read, Write};
use std::panic;

use clap::{Arg, Command};

fn main() -> Result<(), Error> {
    let arg_key = Arg::new("key")
        .index(1)
        .takes_value(true)
        .required(true)
        .help("The key.");

    let arg_value = Arg::new("value")
        .index(2)
        .takes_value(true)
        .required(true)
        .help("The value.");

    let matches = Command::new(clap::crate_name!())
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about(clap::crate_description!())
        .subcommand(
            Command::new("get")
                .about("Gets the value in the database associated with a given key.")
                .arg(&arg_key),
        )
        .subcommand(
            Command::new("set")
                .about("Sets the key/value pair in the database.")
                .arg(&arg_key)
                .arg(&arg_value)
                .arg(
                    Arg::new("force")
                        .short('f')
                        .long("force")
                        .takes_value(false)
                        .help("Overwrites existing key/value in the database."),
                ),
        )
        .subcommand(
            Command::new("remove")
                .about("Removes the key/value pair in the database for a given key.")
                .arg(&arg_key),
        )
        .subcommand(Command::new("init").about("Initalize a new empty key/value database."))
        .get_matches();

    let mut db = Database::from_disk("kv.db")?;

    match matches.subcommand() {
        Some(("get", get_matches)) => {
            let key = get_matches.value_of("key").unwrap();
            match db.get(key) {
                Some((k,v)) => {
                    println!("{} : {}", k, v);
                }
                None => {
                    println!("No entry found for key '{}'.", key);
                }
            }
        }
        Some(("set", set_matches)) => {
            let key = set_matches.value_of("key").unwrap();
            let value = set_matches.value_of("value").unwrap();

            db.insert(
                key.to_string(),
                value.to_string(),
                set_matches.is_present("force"),
            )?;
        }
        Some(("remove", rm_matches)) => {
            let key = rm_matches.value_of("key").unwrap();
            match db.remove(key) {
                Some((k,v)) => {
                    println!("({} : {}) removed from database.", k, v);
                }
                None => {
                    println!("No entry found for key '{}'.", key);
                }
            }

        }
        Some(("init", _init_matches)) => {
            db.init()?;
        }
        _ => {
            return Err(Error::new(
                ErrorKind::Other,
                "Subcommand not specified or was unknown.",
            ));
        }
    }

    Ok(())
}

struct Database {
    map: HashMap<String, String>, // Where key/value pairs are stored
    db_filename: String,          // Filename that key/value database is persisted to
}

// Ensure the database contents are persisted back to disk when the instance is dropped.
impl Drop for Database {
    fn drop(&mut self) {
        if let Err(e) = self.flush() {
            panic!("Error writing to database file. Error: {}", e);
        }
    }
}

impl Database {
    // Read a key/value database from disk into memory.
    // If the specified file doesn't exist, create it.
    fn from_disk(path: &str) -> Result<Database, Error> {
        // Open existing or create a new key/value database file.
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)?;

        // Read contents of file as a single long String.
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        // Populate a hashmap in memory of the file's contents.
        let mut hashmap = HashMap::new();
        for line in contents.lines() {
            let mut chunks = line.split('\t');
            let key = chunks.next().unwrap();
            let value = chunks.next().unwrap();

            hashmap.insert(key.to_string(), value.to_string());
        }

        // Instantiate a new instance of Database and return to caller.
        Ok(Database {
            map: hashmap,
            db_filename: path.to_string(),
        })
    }

    fn get(&self, key: &str) -> Option<(&String, &String)> {
        self.map.get_key_value(key)
    }

    // Insert a new key/value pair into the database.
    // Replaces existing entry if 'replace_existing' is true.
    fn insert(
        &mut self,
        key: String,
        value: String,
        replace_existing: bool,
    ) -> std::io::Result<bool> {
        if (self.map.contains_key(&key)) && (!replace_existing) {
            return Err(Error::new(
                ErrorKind::AlreadyExists,
                format!("{} already exists in database.", key),
            ));
        }
        self.map.insert(key, value);

        Ok(true)
    }

    // Remove an entry from the database.
    fn remove(&mut self, key: &str) -> Option<(String, String)> {
        self.map.remove_entry(key)
    }

    // Initialize a new empty key/value database.
    fn init(&mut self) -> std::io::Result<()> {
        self.map.clear(); // Clear the HashMap entries
        self.flush()?; // Flush the empty HashMap to disk

        Ok(())
    }

    // Persist the key/value database to disk.
    fn flush(&self) -> std::io::Result<()> {
        let mut options = OpenOptions::new();
        let mut file = options.write(true).truncate(true).open(&self.db_filename)?;

        for (k, v) in &self.map {
            let line = format!("{}\t{}\n", k, v);
            let _bytes_written = file.write(line.as_bytes())?;
        }

        Ok(())
    }
}
