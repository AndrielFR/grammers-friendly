// Copyright (C) 2024 AndrielFR
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use async_trait::async_trait;
use grammers_client::{Client, Update};
use regex::Regex;

use crate::traits::Filter;

pub struct RegexFilter {
    regex: Regex,
}

impl RegexFilter {
    pub fn new(regex: &str) -> Self {
        Self {
            regex: Regex::new(regex).unwrap(),
        }
    }
}

#[async_trait]
impl Filter for RegexFilter {
    async fn is_ok(&self, _client: &Client, update: &Update) -> bool {
        let mut text = String::new();

        if let Update::NewMessage(message) | Update::MessageEdited(message) = update {
            text = message.text().to_string();
        } else if let Update::CallbackQuery(query) = update {
            text = String::from_utf8(query.data().to_vec()).unwrap();
        }

        return self.regex.is_match(&text);
    }
}

pub fn regex(regex: &str) -> RegexFilter {
    RegexFilter::new(regex)
}
