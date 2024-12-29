wallet_address := "7AqhQyK8PFM2bCs5yYEtjKUHJkR24VoHSYZqdMXAsUr4"
payer_path := "client/config/wallet.json"
pool_mint := "7ZU5acRDvnfRYWNUvqK3fgR7HyXnsSUDoTiBHMeWMMS1"
position_mint := "DxJ5FYwKtoRYcRr3piDa4UCqt5XNPPALEDGXjaB1qQVp"

default:
  just --list

get-pool:
    cargo run -- --payer-path {{payer_path}} get-pool {{pool_mint}}

get-position:
    cargo run -- --payer-path {{payer_path}} get-position {{position_mint}}

get-positions-by-owner:
    cargo run -- --payer-path {{payer_path}} get-positions-by-owner {{wallet_address}}

close-position:
    cargo run -- --payer-path {{payer_path}} close-position {{position_mint}}
