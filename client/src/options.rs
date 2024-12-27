use clap::Parser;
use solana_sdk::pubkey::Pubkey;

#[derive(Parser, Debug)]
pub struct Options {
    #[arg(
        short,
        long,
        default_value = "config/config.toml",
        help = "Path to the configuration file to use"
    )]
    pub config: String,
    #[clap(subcommand)]
    pub commands: Commands,
}

#[derive(Debug, Parser)]
pub enum Commands {
    Pool { pool_id: Pubkey },
}
