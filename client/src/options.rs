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
    #[arg(short, long, help = "Solana RPC URL (overrides config file)")]
    pub rpc_url: Option<String>,
    #[arg(
        short,
        long,
        help = "Path to your wallet's keypair file (overrides config file)"
    )]
    pub payer_path: Option<String>,
    #[clap(subcommand)]
    pub commands: Commands,
}

#[derive(Debug, Parser)]
pub enum Commands {
    Pool { pool_id: Pubkey },
}
