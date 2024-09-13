// Copyright (C) 2024 AndrielFR
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::collections::{hash_map::Entry, HashMap};

use async_trait::async_trait;
use grammers_client::{client::chats::ParticipantPermissions, Client, Update};

use crate::traits::{Filter, GetChat, GetSender};

/// Admin filter.
///
/// Checks for user perms in chat.
/// Pass if user has admin rights.
///
/// Has internal cache.
#[derive(Clone, Default)]
pub struct AdminFilter {
    perms: HashMap<i64, HashMap<i64, ParticipantPermissions>>,
}

#[async_trait]
impl Filter for AdminFilter {
    async fn is_ok(&mut self, client: &Client, update: &Update) -> bool {
        let chat = update.get_chat();
        let user = update.get_sender();

        if let Some(chat) = chat {
            let chat_id = chat.id();

            if let Some(user) = user {
                let user_id = user.id();

                if let Entry::Vacant(e) = self.perms.entry(chat_id) {
                    let perms = client.get_permissions(&chat, &user).await;
                    if let Ok(perms) = perms {
                        let mut hash = HashMap::new();
                        hash.insert(user_id, perms.clone());
                        e.insert(hash);
                        return perms.is_admin() || perms.is_creator();
                    }
                } else if let Some(hash) = self.perms.get_mut(&chat_id) {
                    if hash.get(&user_id).is_none() {
                        let perms = client.get_permissions(&chat, &user).await;
                        if let Ok(perms) = perms {
                            hash.insert(user_id, perms.clone());
                            return perms.is_admin() || perms.is_creator();
                        }
                    } else if let Some(perms) = hash.get(&user_id) {
                        return perms.is_admin() || perms.is_creator();
                    }
                }
            }
        }

        false
    }
}

/// Checks for user perms in chat.
/// Pass if user has admin rights.
///
/// Has internal cache.
pub fn admin() -> AdminFilter {
    AdminFilter::default()
}
