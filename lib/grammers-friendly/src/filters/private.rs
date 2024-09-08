// Copyright (C) 2024 AndrielFR
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use async_trait::async_trait;
use grammers_client::{types::Chat, Client, Update};

use crate::traits::{Filter, GetChat};

#[derive(Clone)]
pub struct PrivateFilter;

#[async_trait]
impl Filter for PrivateFilter {
    async fn is_ok(&mut self, _client: &Client, update: &Update) -> bool {
        let chat = update.get_chat();

        if let Some(chat) = chat {
            return match chat {
                Chat::User(_) => true,
                Chat::Group(_) | Chat::Channel(_) => false,
            };
        }

        false
    }
}

pub fn private() -> PrivateFilter {
    PrivateFilter
}
