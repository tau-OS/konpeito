use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, author)]
pub struct Konpeito {
    #[clap(subcommand)]
    pub subcmd: KonpeitoCmd,
    #[clap(short = 'S', long)]
    /// Service to use for keyring operations
    #[clap(default_value = crate::APPID)]
    pub service: String,
}

#[derive(Subcommand)]
pub enum KonpeitoCmd {
    #[clap(about = "Query a key from the keyring")]
    Get { key: String },
    #[clap(about = "Set a key in the keyring")]
    Set { key: String, value: Option<String> },
    #[clap(about = "Delete a key in the keyring")]
    Delete { key: String },
    #[clap(about = "List all keys in the keyring")]
    List,
}
