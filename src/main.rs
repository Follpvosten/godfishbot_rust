extern crate futures;
extern crate telebot;
extern crate tokio_core;

extern crate godfishbot_rust;

use telebot::RcBot;
use tokio_core::reactor::Core;
use futures::stream::Stream;
use std::env;

use godfishbot_rust::config::*;
use godfishbot_rust::text_commands::*;

fn main() {
    println!("Starting godfishbot...");

    let config_filename: String = env::args()
        .nth(1)
        .or(Some(String::from("BotConfig.toml")))
        .unwrap();

    println!("Loading config file {:?}...", config_filename);
    let bot_config = read_config_file(&config_filename).expect("Error loading config file");

    println!("Creating bot...");
    let mut lp = Core::new().unwrap();
    let my_bot = RcBot::new(lp.handle(), &bot_config.bot_token).update_interval(200);

    println!("Registering handlers...");
    let start_cmd = my_bot.new_cmd("/start").and_then(handle_start);
    let about_cmd = my_bot.new_cmd("/about").and_then(handle_about);

    my_bot.register(start_cmd);
    my_bot.register(about_cmd);

    println!("Starting bot...");
    my_bot.run(&mut lp).unwrap();
}
