// Copyright (C) 2024 AndrielFR
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use grammers_client::{types::Chat, Update};

pub fn get_chat(update: &Update) -> Option<Chat> {
    let mut chat = None;

    if let Update::NewMessage(message) | Update::MessageEdited(message) = update {
        chat = Some(message.chat());
    } else if let Update::CallbackQuery(query) = update {
        chat = Some(query.chat().clone());
    }

    chat
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
