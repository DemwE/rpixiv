mod args;
mod config;

// use clap::Parser;
// use dirs::config_dir;
// use pixiv::Pixiv;
// use reqwest::Client;
// use std::fs;

fn main() {
    // let args = args::Args::parse();

    let config = config::Config::new();
    config.check();
    let user = config.read();
    println!("User: {}", user.username);
    println!("Password: {}", user.password);
    // let client = Client::new();

    // let mut pixiv: Pixiv = Pixiv::new(&client);
    // pixiv.login(args.username, args.password);
}