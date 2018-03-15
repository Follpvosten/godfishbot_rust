use telebot::RcBot;
use futures::Future;
use failure::Error;

use telebot::functions::*;
use telebot::objects;

pub fn handle_start(
    (bot, msg): (RcBot, objects::Message),
) -> impl Future<Item = (RcBot, objects::Message), Error = Error> {
    let text = r#"Welcome to godfishbot_rust!

This is a rewrite of my original "GodfishBot", which was written in Java and can be found at https://github.com/Follpvosten/GodfishBot
"#;
    //Type /help to get a list of commands!"#;

    bot.message(msg.chat.id, text.to_string()).send()
}

pub fn handle_about(
    (bot, msg): (RcBot, objects::Message),
) -> impl Future<Item = (RcBot, objects::Message), Error = Error> {
    let text = format!(
        r#"godfishbot_rust version {}

This program is free software and was released under the terms of the GNU General Public License Version 2.

You can obtain the source code at https://github.com/Follpvosten/godfishbot_rust/"#,
        env!("CARGO_PKG_VERSION")
    );

    bot.message(msg.chat.id, text.to_string()).send()
}

pub fn handle_hello(
    (bot, msg): (RcBot, objects::Message),
) -> impl Future<Item = (RcBot, objects::Message), Error = Error> {
    let text = format!("Hello, {} ( ͡° ͜ʖ ͡°)", msg.from.unwrap().first_name);

    bot.message(msg.chat.id, text).send()
}
