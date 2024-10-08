// Copyright (C) 2024 AndrielFR
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use async_trait::async_trait;
use grammers_client::{Client, Update};

use crate::traits::{Filter, GetMessage};

/// Text filter.
///
/// Pass if message contains `text`.
#[derive(Clone)]
pub struct TextFilter {
    text: String,
}

impl TextFilter {
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }
}

#[async_trait]
impl Filter for TextFilter {
    async fn is_ok(&mut self, _client: &Client, update: &Update) -> bool {
        let message = update.get_message();

        if let Some(message) = message {
            return message.text().contains(&self.text);
        }

        false
    }
}

/// Pass if message contains `text`.
pub fn text(text: &str) -> TextFilter {
    TextFilter::new(text)
}
