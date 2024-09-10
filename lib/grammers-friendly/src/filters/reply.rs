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

/// Ok if message is reply to another message
#[derive(Clone)]
pub struct ReplyFilter;

#[async_trait]
impl Filter for ReplyFilter {
    async fn is_ok(&mut self, _client: &Client, update: &Update) -> bool {
        let message = update.get_message();

        if let Some(message) = message {
            return message.reply_to_message_id().is_some();
        }

        false
    }
}

pub fn reply() -> ReplyFilter {
    ReplyFilter
}
