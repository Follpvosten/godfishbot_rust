extern crate futures;
extern crate telebot;

use self::telebot::RcBot;
use self::futures::Future;
use failure::Error;

use self::telebot::functions::*;

pub fn handle_start(
    (bot, msg): (RcBot, telebot::objects::Message),
) -> impl Future<Item = (RcBot, telebot::objects::Message), Error = Error> {
    let mut text = msg.text.unwrap().clone();
    if text.is_empty() {
        text = "<nix>".into();
    }

    bot.message(msg.chat.id, text).send()
}
