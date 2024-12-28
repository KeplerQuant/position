default:
  just --list

get-pool:
    cargo run -- --payer-path client/config/wallet.json get-pool 7ZU5acRDvnfRYWNUvqK3fgR7HyXnsSUDoTiBHMeWMMS1

get-position:
    cargo run -- --payer-path client/config/wallet.json get-position DxJ5FYwKtoRYcRr3piDa4UCqt5XNPPALEDGXjaB1qQVp
