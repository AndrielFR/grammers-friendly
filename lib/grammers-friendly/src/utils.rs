// Copyright (C) 2024 AndrielFR
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::cmp::Ordering;

use grammers_client::{
    button::{self, Inline},
    types::{CallbackQuery, Chat, Message},
    Update,
};

/// Get the chat from [Update]
pub fn get_chat(update: &Update) -> Option<Chat> {
    let mut chat = None;

    if let Update::NewMessage(message) | Update::MessageEdited(message) = update {
        chat = Some(message.chat());
    } else if let Update::CallbackQuery(query) = update {
        chat = Some(query.chat().clone());
    }

    chat
}

/// Get the message from [Update]
pub fn get_message(update: &Update) -> Option<Message> {
    let mut message = None;

    if let Update::NewMessage(m) | Update::MessageEdited(m) = update {
        message = Some(m.clone());
    }

    message
}

/// Get the query from [Update]
pub fn get_query(update: &Update) -> Option<CallbackQuery> {
    let mut query = None;

    if let Update::CallbackQuery(q) = update {
        query = Some(q.clone());
    }

    query
}

/// Get the sender from [Update]
pub fn get_sender(update: &Update) -> Option<Chat> {
    let mut sender = None;

    if let Update::NewMessage(message) | Update::MessageEdited(message) = update {
        sender = message.sender();
    } else if let Update::CallbackQuery(query) = update {
        sender = Some(query.sender().clone());
    }

    sender
}

/// Split the inline keyboard by n colums
pub fn split_kb_to_columns(buttons: Vec<Inline>, count: usize) -> Vec<Vec<Inline>> {
    let mut columns = Vec::new();

    let mut column = Vec::new();
    for button in buttons.into_iter() {
        if column.len() == count {
            columns.push(column);
            column = Vec::new();
        }

        column.push(button);
    }

    if !column.is_empty() {
        columns.push(column);
    }

    columns
}

/// Split the inline keyboard by n rows
pub fn split_kb_to_rows(buttons: Vec<Inline>, count: usize) -> Vec<Vec<Inline>> {
    let buttons_per_column = buttons.len().abs_diff(count);

    split_kb_to_columns(buttons, buttons_per_column)
}

/// Split the query from `query.data()`
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

pub fn gen_page_buttons(
    current_page: i64,
    total_pages: i64,
    query: impl Into<String>,
    max_buttons: i64,
) -> Vec<Inline> {
    let mut buttons = Vec::with_capacity(max_buttons as usize);
    let query = query.into();

    let start_symbol = '.';
    let previous_symbol = '·';
    let current_symbol = '•';
    let next_symbol = '·';
    let end_symbol = '.';

    let mut start = (current_page - (max_buttons / 2)).max(1);
    let end = (start + (max_buttons - 1)).min(total_pages);

    if start > 1 {
        let diff = total_pages - current_page;

        if diff > 1 {
            start += 1;
        } else if diff < 1 {
            start -= 1;
        }

        buttons.push(button::inline(
            format!("{0} {1}", start_symbol, 1),
            query.replace(":page:", "1"),
        ));
    }

    for page in start..=end {
        let callback = query.replace(":page:", &page.to_string());

        match page.cmp(&current_page) {
            Ordering::Greater => buttons.push(button::inline(
                format!("{0} {1}", page, next_symbol),
                callback.clone(),
            )),
            Ordering::Less => buttons.push(button::inline(
                format!("{0} {1}", previous_symbol, page),
                callback.clone(),
            )),
            Ordering::Equal => buttons.push(button::inline(
                format!("{0} {1} {0}", current_symbol, page),
                callback.clone(),
            )),
        }
    }

    if end < total_pages {
        buttons.remove(buttons.len() - 1);
        buttons.push(button::inline(
            format!("{0} {1}", total_pages, end_symbol),
            query.replace(":page:", &total_pages.to_string()),
        ));
    }

    buttons
}
