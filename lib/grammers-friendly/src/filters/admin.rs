// Copyright (C) 2024 AndrielFR
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use async_trait::async_trait;
use grammers_client::{Client, Update};

use crate::traits::{Filter, GetChat, GetSender};

pub struct AdminFilter;

#[async_trait]
impl Filter for AdminFilter {
    async fn is_ok(&self, client: &Client, update: &Update) -> bool {
        let chat = update.get_chat().expect("Failed to get chat");
        let user = update.get_sender().expect("Failed to get sender");

        let perm = client.get_permissions(chat, user).await.ok();
        if let Some(perm) = perm {
            return perm.is_admin() || perm.is_creator();
        }

        false
    }
}

pub fn admin() -> AdminFilter {
    AdminFilter
}
