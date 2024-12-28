use clap::Parser;
use solana_sdk::pubkey::Pubkey;

#[derive(Parser, Debug)]
pub struct Options {
    #[arg(
        long,
        default_value = "https://api.devnet.solana.com",
        help = "Solana RPC URL"
    )]
    pub rpc_url: String,

    #[arg(
        long,
        default_value = "wss://api.devnet.solana.com/",
        help = "Solana WS URL"
    )]
    pub ws_url: String,

    #[arg(
        long,
        default_value = "config/wallet.json",
        help = "Path to your wallet's keypair file"
    )]
    pub payer_path: String,

    #[arg(
        long,
        default_value = "devi51mZmdwUJGU9hjN27vEz64Gps7uUefqxg27EAtH",
        help = "The program ID of Raydium V3 contract"
    )]
    pub raydium_v3_program: Pubkey,

    #[clap(subcommand)]
    pub commands: Commands,
}

#[derive(Debug, Parser)]
pub enum Commands {
    #[command(about = "Query pool info")]
    GetPool { pool_id: Pubkey },
    #[command(about = "Query position info")]
    GetPosition { position_id: Pubkey },
    #[command(about = "Query all positions belong to wallet")]
    GetPositionsByOwner { user_wallet: Pubkey },
}
