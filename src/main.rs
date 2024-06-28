use std::io::{Read, Write};

use clap::Parser;

mod cli;
pub mod keys;

pub const APPID: &'static str = "com.fyralabs.konpeito";

fn main() -> color_eyre::Result<()> {
    let p = cli::Konpeito::parse();

    match p.subcmd {
        cli::KonpeitoCmd::Get { key } => {
            std::io::stdout().write_all(&keys::get_key(&key, &p.service)?)?;
        }
        cli::KonpeitoCmd::Set { key, value } => {
            let mut buf = vec![];
            let value = match &value {
                Some(value) => value.as_bytes(),
                None => {
                    // get from stdin
                    std::io::stdin().read_to_end(&mut buf)?;
                    &*buf
                }
            };
            keys::set_key(&key, value, &p.service)?;
        }
        cli::KonpeitoCmd::Delete { key } => {
            keys::del_key(&key, &p.service)?;
        }
        
        cli::KonpeitoCmd::List => {
            let keys = keys::list_keys(&p.service)?;
            for key in keys {
                println!("{}", key);
            }
        }
    }
    Ok(())
}
