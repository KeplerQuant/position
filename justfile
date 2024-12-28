payer_path := "client/config/wallet.json"
pool_id := "7ZU5acRDvnfRYWNUvqK3fgR7HyXnsSUDoTiBHMeWMMS1"
position_id := "DxJ5FYwKtoRYcRr3piDa4UCqt5XNPPALEDGXjaB1qQVp"

default:
  just --list

get-pool:
    cargo run -- --payer-path {{payer_path}} get-pool {{pool_id}}

get-position:
    cargo run -- --payer-path {{payer_path}} get-position {{position_id}}
