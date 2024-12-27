use client::config::Settings;

fn main() {
    let settings =
        Settings::new("config.toml").expect("Failed to load settings from configuration file");

    println!("{:#?}", settings);
}
