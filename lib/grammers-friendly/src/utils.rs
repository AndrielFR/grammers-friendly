// Copyright (C) 2024 AndrielFR
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use grammers_client::{
    button::Inline,
    types::{CallbackQuery, Chat, Message},
    Update,
};

pub fn get_chat(update: &Update) -> Option<Chat> {
    let mut chat = None;

    if let Update::NewMessage(message) | Update::MessageEdited(message) = update {
        chat = Some(message.chat());
    } else if let Update::CallbackQuery(query) = update {
        chat = Some(query.chat().clone());
    }

    chat
}

pub fn get_query(update: &Update) -> Option<CallbackQuery> {
    let mut query = None;

    if let Update::CallbackQuery(q) = update {
        query = Some(q.clone());
    }

    query
}

pub fn get_message(update: &Update) -> Option<Message> {
    let mut message = None;

    if let Update::NewMessage(m) | Update::MessageEdited(m) = update {
        message = Some(m.clone());
    }

    message
}

pub fn get_sender(update: &Update) -> Option<Chat> {
    let mut sender = None;

    if let Update::NewMessage(message) | Update::MessageEdited(message) = update {
        sender = message.sender();
    } else if let Update::CallbackQuery(query) = update {
        sender = Some(query.sender().clone());
    }

    sender
}

pub fn split_kb_to_columns(buttons: Vec<Inline>, count: usize) -> Vec<Vec<Inline>> {
    let mut columns = Vec::new();

    let mut col = Vec::new();
    for button in buttons.into_iter() {
        if col.len() == count {
            columns.push(col);
            col = Vec::new();
        }

        col.push(button);
    }

    if !col.is_empty() {
        columns.push(col);
    }

    columns
}

pub fn split_query<Q: Into<Vec<u8>>>(query: Q) -> Vec<String> {
    let mut splitted = Vec::new();

    String::from_utf8(query.into())
        .unwrap()
        .split_whitespace()
        .for_each(|part| {
            splitted.push(part.to_string());
        });

    splitted
}
