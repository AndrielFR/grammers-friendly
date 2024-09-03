// Copyright (C) 2024 AndrielFR
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use grammers_client::{Client, Update};

use crate::traits::Filter;

pub struct TextFilter {
    text: String,
}

impl TextFilter {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
        }
    }
}

impl Filter for TextFilter {
    fn is_ok(&self, _client: &Client, update: &Update) -> bool {
        if let Update::NewMessage(message) = update {
            return message.text().contains(&self.text);
        }
        false
    }
}

pub fn text(text: &str) -> TextFilter {
    TextFilter::new(text)
}
