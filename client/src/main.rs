use anyhow::Result;
use client::config::Config;

#[tokio::main]
pub async fn main() -> Result<()> {
    let settings = Config::from_path("config/config.toml")?;

    println!("{:#?}", settings);

    Ok(())
}
