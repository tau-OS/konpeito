//! # Konpeito
//! Konpeito is a personal key-value store writt&en in Rust, powered by sled.
//! It is a simple, fast, and encrypted key-value store that can store any type of data.

use std::{path::PathBuf, io::Read};

use clap::Parser;
mod key;
use std::io::Write;
/// Konpeito Personal Key-value store
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Path to the database store
    #[clap(short, long, default_value = "~/.local/share/konpeito")]
    db: String,

    /// Path to the key file
    #[clap(short, long, default_value = "~/.ssh/id_rsa")]
    secret: PathBuf,

    /// The command to execute
    #[clap(subcommand)]
    command: Command,
}

#[derive(clap::Subcommand, Debug)]
enum Command {
    /// Gets the value of a key
    Get {
        /// The key to get
        key: String,
    },
    /// Sets the value of a key
    Set {
        /// The key to set
        key: String,
        /// The value to set.
        /// Can be a string or piped input
        value: Option<String>
    },
    /// Lists all keys and values (WIP)
    List,
    /// Deletes a key
    Delete {
        /// The key to delete
        key: String,
    },
}


fn main() {
    //println!("Hello, world!");
    let cli = Cli::parse();
    //let mc = new_magic_crypt!("bits", 256);

    //let a = mc.encrypt_to_base64("hello");

    //println!("{:?}", a);
    //let tree = sled::open(cli.db).expect("failed to open db");
    // if ~ exists in secret path, expand it
    let secret = PathBuf::from(shellexpand::tilde(cli.secret.to_str().unwrap()).to_string());
    //println!("{:?}", secret);

    let ks = key::KeyStore::load_with_passwdfile(&cli.db, secret);

    match cli.command {
        Command::Get { key } => {
            //println!("{}", key);
            let a = ks.get(&key);
            // write to stdout
            let mut out = std::io::stdout();
            //out.flush().unwrap();
            out.write_all(a.unwrap().as_slice()).unwrap();
            //out.flush().unwrap();
        }
        Command::Set { key, value } => {
            //println!("{:?}", value);
            if let Some(value) = value {
                ks.set(&key, value.as_bytes().to_vec());
            } else {
                let mut buf = Vec::new();
                std::io::stdin().read_to_end(&mut buf).unwrap();
                //println!("{:?}", buf);
                ks.set(&key, buf);
            }
        }
        Command::List => {
            //println!("List");
            ks.list();
        }
        Command::Delete { key } => {
            ks.delete(&key);
            eprintln!("Deleted key `{}`", key);
        }
    }


    //tree.insert("a", "b").expect("failed to insert");

}
