#![feature(conservative_impl_trait)]
extern crate failure;

#[macro_use]
extern crate serde_derive;

extern crate futures;
extern crate telebot;

pub mod config;

pub mod text_commands;
