// Copyright (C) 2024 AndrielFR
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use async_trait::async_trait;
use grammers_client::{types::Chat, Client, Update};

use crate::{traits::Filter, utils};

pub struct PrivateFilter;

#[async_trait]
impl Filter for PrivateFilter {
    async fn is_ok(&self, _client: &Client, update: &Update) -> bool {
        let chat = utils::get_chat(update).expect("Failed to get chat");

        match chat {
            Chat::User(_) => true,
            Chat::Group(_) | Chat::Channel(_) => false,
        }
    }
}

pub fn private() -> PrivateFilter {
    PrivateFilter
}
