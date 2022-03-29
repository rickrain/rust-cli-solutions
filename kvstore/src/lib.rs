use clap::{Arg, Command};
use database::Database;
use std::io::{Error, ErrorKind};

mod database;

pub enum SubCommand {
    Get {
        key: String,
    },
    Set {
        key: String,
        value: String,
        force: bool,
    },
    Remove {
        key: String,
    },
    Init,
}

pub fn get_args() -> std::io::Result<SubCommand> {
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
        .subcommand_required(true)
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

    match matches.subcommand() {
        Some(("get", get_matches)) => Ok(SubCommand::Get {
            key: get_matches.value_of("key").unwrap().to_string(),
        }),
        Some(("set", set_matches)) => Ok(SubCommand::Set {
            key: set_matches.value_of("key").unwrap().to_string(),
            value: set_matches.value_of("value").unwrap().to_string(),
            force: set_matches.is_present("force"),
        }),
        Some(("remove", rm_matches)) => Ok(SubCommand::Remove {
            key: rm_matches.value_of("key").unwrap().to_string(),
        }),
        Some(("init", _init_matches)) => Ok(SubCommand::Init {}),

        // This should never get executed since get_matches() will bubble up an
        // error if there is not a subcommand provided.
        _ => Err(Error::new(
            ErrorKind::Other,
            "Subcommand not specified or was unknown.",
        )),
    }
}

pub fn run(cmd: SubCommand) -> std::io::Result<()> {
    let mut db = Database::from_disk("kv.db")?;

    match cmd {
        SubCommand::Get { key } => match db.get(&key) {
            Some((k, v)) => {
                println!("{} : {}", k, v);
                Ok(())
            }
            None => Err(Error::new(
                ErrorKind::NotFound,
                format!("No entry found for key '{}'.", key),
            )),
        },
        SubCommand::Set { key, value, force } => {
            db.insert(key, value, force)?;
            Ok(())
        }
        SubCommand::Remove { key } => match db.remove(&key) {
            Some((k, v)) => {
                println!("({} : {}) removed from database.", k, v);
                Ok(())
            }
            None => Err(Error::new(
                ErrorKind::NotFound,
                format!("No entry found for key '{}'.", key),
            )),
        },
        SubCommand::Init => db.init(),
    }
}
