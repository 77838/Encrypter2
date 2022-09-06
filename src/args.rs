use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct CLIAPP {
    #[clap(subcommand)]
    pub actions: UserSubCommand,
}

#[derive(Debug, Subcommand)]
pub enum UserSubCommand {
    Encrypt{ file_path: String , key: String },
    Decrypt{file_path: String , key: String },
    Generate,
}
