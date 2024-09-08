// Copyright (C) 2024 AndrielFR
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::sync::Arc;

use grammers_client::{Client, Update};

use crate::{
    traits::{AsyncFn, Filter},
    Data,
};

/// Use the Handler struct to create a new handler. The handle method is used to run the function if the filters pass.
#[derive(Clone)]
pub struct Handler {
    func: Arc<dyn AsyncFn>,
    filter: Box<dyn Filter>,
    update_type: UpdateType,
}

impl Handler {
    /// Create a new handler
    pub fn new(update_type: UpdateType, func: impl AsyncFn, filter: impl Filter) -> Self {
        Self {
            func: Arc::new(func),
            filter: Box::new(filter),
            update_type,
        }
    }

    /// Create a new handler with `NewMessage` update type
    pub fn new_message(func: impl AsyncFn, filter: impl Filter) -> Self {
        Self::new(UpdateType::NewMessage, func, filter)
    }

    /// Create a new handler with `MessageEdited` update type
    pub fn message_edited(func: impl AsyncFn, filter: impl Filter) -> Self {
        Self::new(UpdateType::MessageEdited, func, filter)
    }

    /// Create a new handler with `MessageDeleted` update type
    pub fn message_deleted(func: impl AsyncFn, filter: impl Filter) -> Self {
        Self::new(UpdateType::MessageDeleted, func, filter)
    }

    /// Create a new handler with `CallbackQuery` update type
    pub fn callback_query(func: impl AsyncFn, filter: impl Filter) -> Self {
        Self::new(UpdateType::CallbackQuery, func, filter)
    }

    /// Create a new handler with `InlineQuery` update type
    pub fn inline_query(func: impl AsyncFn, filter: impl Filter) -> Self {
        Self::new(UpdateType::InlineQuery, func, filter)
    }

    /// Create a new handler with `Raw` update type
    pub fn raw(func: impl AsyncFn, filter: impl Filter) -> Self {
        Self::new(UpdateType::Raw, func, filter)
    }

    /// Check all the filters and if ok, run the func
    pub async fn handle(&mut self, client: &Client, update: &Update, data: &Data) -> bool {
        if matches!(self.update_type, UpdateType::NewMessage)
            && matches!(update, Update::NewMessage(_))
            || matches!(self.update_type, UpdateType::MessageEdited)
                && matches!(update, Update::MessageEdited(_))
            || matches!(self.update_type, UpdateType::MessageDeleted)
                && matches!(update, Update::MessageDeleted(_))
            || matches!(self.update_type, UpdateType::CallbackQuery)
                && matches!(update, Update::CallbackQuery(_))
            || matches!(self.update_type, UpdateType::InlineQuery)
                && matches!(update, Update::InlineQuery(_))
            || matches!(self.update_type, UpdateType::Raw)
        {
            if !self.filter.is_ok(client, update).await {
                return false;
            }

            let r = self
                .func
                .call(client.clone(), update.clone(), data.clone())
                .await;
            if let Err(e) = r {
                log::error!("Error running handler: {}", e);
            }

            return true;
        }
        false
    }
}

/// Just the update type
#[derive(Clone)]
pub enum UpdateType {
    NewMessage,
    MessageEdited,
    MessageDeleted,
    CallbackQuery,
    InlineQuery,
    Raw,
}
