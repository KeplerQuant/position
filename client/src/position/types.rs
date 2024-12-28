use solana_sdk::pubkey::Pubkey;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PositionNftTokenInfo {
    pub key: Pubkey,
    pub program: Pubkey,
    pub position: Pubkey,
    pub mint: Pubkey,
    pub amount: u64,
    pub decimals: u8,
}
