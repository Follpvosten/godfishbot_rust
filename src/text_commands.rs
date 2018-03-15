use telebot::RcBot;
use futures::Future;
use failure::Error;

use telebot::functions::*;
use telebot::objects;

pub fn handle_start(
    (bot, msg): (RcBot, objects::Message),
) -> impl Future<Item = (RcBot, objects::Message), Error = Error> {
    let mut text = msg.text.unwrap().clone();
    if text.is_empty() {
        text = "<nix>".into();
    }

    bot.message(msg.chat.id, text).send()
}
