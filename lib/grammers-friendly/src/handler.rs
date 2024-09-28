// Copyright (C) 2024 AndrielFR
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::sync::Arc;

use grammers_client::{Client, Update};
use tokio::sync::Mutex;

use crate::{
    traits::{AsyncFnCallback, Filter},
    Data, Middleware, MiddlewareType,
};

/// A Handler.
///
/// Will be runned after before-type `middlewares`.
#[derive(Clone)]
pub struct Handler {
    func: Arc<dyn AsyncFnCallback>,
    filter: Box<dyn Filter>,
    update_type: UpdateType,
}

impl Handler {
    /// Construct a new handler.
    ///
    /// Receives a [`UpdateType`], `Fn(&mut Client, &mut Update, &mut Data)` and its filter.
    ///
    /// [`UpdateType`]: crate::UpdateType
    pub fn new<A: AsyncFnCallback, F: Filter>(update_type: UpdateType, func: A, filter: F) -> Self {
        Self {
            func: Arc::new(func),
            filter: Box::new(filter),
            update_type,
        }
    }

    /// Construct a new handler with `NewMessage` update type.
    ///
    /// Receives a Fn(&mut Client, &mut Update, &mut Data)` and its filter.
    pub fn new_message<A: AsyncFnCallback, F: Filter>(func: A, filter: F) -> Self {
        Self::new(UpdateType::NewMessage, func, filter)
    }

    /// Construct a new handler with `MessageEdited` update type.
    ///
    /// Receives a Fn(&mut Client, &mut Update, &mut Data)` and its filter.
    pub fn message_edited<A: AsyncFnCallback, F: Filter>(func: A, filter: F) -> Self {
        Self::new(UpdateType::MessageEdited, func, filter)
    }

    /// Construct a new handler with `MessageDeleted` update type.
    ///
    /// Receives a Fn(&mut Client, &mut Update, &mut Data)` and its filter.
    pub fn message_deleted<A: AsyncFnCallback, F: Filter>(func: A, filter: F) -> Self {
        Self::new(UpdateType::MessageDeleted, func, filter)
    }

    /// Construct a new handler with `CallbackQuery` update type.
    ///
    /// Receives a Fn(&mut Client, &mut Update, &mut Data)` and its filter.
    pub fn callback_query<A: AsyncFnCallback, F: Filter>(func: A, filter: F) -> Self {
        Self::new(UpdateType::CallbackQuery, func, filter)
    }

    /// Construct a new handler with `InlineQuery` update type.
    ///
    /// Receives a Fn(&mut Client, &mut Update, &mut Data)` and its filter.
    pub fn inline_query<A: AsyncFnCallback, F: Filter>(func: A, filter: F) -> Self {
        Self::new(UpdateType::InlineQuery, func, filter)
    }

    /// Construct a new handler with `Raw` update type.
    ///
    /// Receives a Fn(&mut Client, &mut Update, &mut Data)` and its filter.
    pub fn raw<A: AsyncFnCallback, F: Filter>(func: A, filter: F) -> Self {
        Self::new(UpdateType::Raw, func, filter)
    }

    /// Handle the update.
    ///
    /// First checks if [`UpdateType`] match,
    /// So, checks if its `filter` match and
    /// Lastly, if all ok, run the `function`.
    ///
    /// Return `True` if handled or `False` otherwise.
    ///
    /// [`UpdateType`]: crate::UpdateType
    pub async fn handle(
        &mut self,
        client: &mut Client,
        update: &mut Update,
        data: &mut Data,
        middlewares: &mut Vec<Arc<Mutex<Middleware>>>,
    ) -> bool {
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
            if !self.filter.is_ok(&*client, &*update).await {
                return false;
            }

            for middleware in middlewares
                .iter_mut()
                .filter(|m| m.try_lock().unwrap().mtype() == MiddlewareType::Before)
            {
                let mut mid = middleware.try_lock().unwrap();
                mid.call(client, update, data).await;
            }

            if let Err(e) = self.func.call(client, update, data).await {
                log::error!("Error while running handler: {}", e);
                return false;
            }

            for middleware in middlewares
                .iter_mut()
                .filter(|m| m.try_lock().unwrap().mtype() == MiddlewareType::After)
            {
                let mut mid = middleware.try_lock().unwrap();
                mid.call(client, update, data).await;
            }

            return true;
        }

        false
    }
}

/// Update Type.
///
/// In thesis, you don't need to use this,
/// Just use [`Handler`] constructors: `::new_message(...)`, `::message_edited(...)`, `::message_deleted(...)`,
/// `::callback_query(...)`, `::inline_query(...)` and/or `::raw(...)`
///
/// [`Handler`]: crate::Handler
#[derive(Clone)]
pub enum UpdateType {
    /// Just listen to new messages.
    NewMessage,

    /// Just listen to edited messages.
    MessageEdited,

    /// Just listen to deleted messages.
    MessageDeleted,

    /// Just listen to callback query.
    CallbackQuery,

    /// Just listen to inline query.
    InlineQuery,

    /// Listen to all updates in its raw form.
    Raw,
}
