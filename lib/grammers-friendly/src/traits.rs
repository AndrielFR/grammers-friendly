// Copyright (C) 2024 AndrielFR
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::pin::Pin;

use async_trait::async_trait;
use downcast_rs::{impl_downcast, DowncastSync};
use dyn_clone::DynClone;
use futures_util::Future;
use grammers_client::{
    types::{CallbackQuery, Chat, Message},
    Client, Update,
};

use crate::{
    filters::{AndFilter, NotFilter, OrFilter},
    utils, Data,
};

type PinBox =
    Pin<Box<dyn Future<Output = Result<(), Box<dyn std::error::Error>>> + Send + 'static>>;

pub trait AsyncFn: DynClone {
    fn call(&self, client: Client, update: Update, data: Data) -> PinBox;
}

impl<T, F> AsyncFn for T
where
    T: Fn(Client, Update, Data) -> F,
    T: DynClone,
    F: Future<Output = Result<(), Box<dyn std::error::Error>>> + Send + 'static,
{
    fn call(&self, client: Client, update: Update, data: Data) -> PinBox {
        Box::pin(self(client, update, data))
    }
}

dyn_clone::clone_trait_object!(AsyncFn);

/// Filter
#[async_trait]
pub trait Filter {
    /// Needs to return bool
    /// `True` -> pass
    /// `False` -> not pass
    async fn is_ok(&self, client: &Client, update: &Update) -> bool;

    fn and(self, other: impl Filter + Send + Sync + 'static) -> AndFilter
    where
        Self: Send + Sync + Sized + 'static,
    {
        AndFilter::new(self, other)
    }

    fn or(self, other: impl Filter + Send + Sync + 'static) -> OrFilter
    where
        Self: Send + Sync + Sized + 'static,
    {
        OrFilter::new(self, other)
    }

    fn not(self) -> NotFilter
    where
        Self: Send + Sync + Sized + 'static,
    {
        NotFilter::new(self)
    }
}

/// Middleware
#[async_trait]
pub trait MiddlewareImpl: DynClone {
    async fn call(&self, client: Client, update: Update) -> Result<(), Box<dyn std::error::Error>>;
}

dyn_clone::clone_trait_object!(MiddlewareImpl);

/// Module
#[async_trait]
pub trait Module: DowncastSync + CloneModule {
    async fn ante_call(
        &mut self,
        client: &mut Client,
        update: &mut Update,
    ) -> Result<(), Box<dyn std::error::Error>>;
    async fn post_call(
        &mut self,
        client: &mut Client,
        update: &mut Update,
    ) -> Result<(), Box<dyn std::error::Error>>;

    fn into_any(self: Box<Self>) -> Box<dyn Module>
    where
        Self: Sized,
    {
        self
    }
}

impl_downcast!(sync Module);

pub trait CloneModule {
    fn clone_module(&self) -> Box<dyn Module>;
}

impl<T> CloneModule for T
where
    T: Module + Clone,
{
    fn clone_module(&self) -> Box<dyn Module> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Module> {
    fn clone(&self) -> Self {
        self.clone_module()
    }
}

pub trait GetChat {
    fn get_chat(&self) -> Option<Chat>;
}

/// Get chat from [Update]
impl GetChat for Update {
    fn get_chat(&self) -> Option<Chat> {
        utils::get_chat(self)
    }
}

pub trait GetMessage {
    fn get_message(&self) -> Option<Message>;
}

/// Get message from [Update]
impl GetMessage for Update {
    fn get_message(&self) -> Option<Message> {
        utils::get_message(self)
    }
}

pub trait GetQuery {
    fn get_query(&self) -> Option<CallbackQuery>;
}

/// Get query from [Update]
impl GetQuery for Update {
    fn get_query(&self) -> Option<CallbackQuery> {
        utils::get_query(self)
    }
}

pub trait GetSender {
    fn get_sender(&self) -> Option<Chat>;
}

/// Get sender from [Update]
impl GetSender for Update {
    fn get_sender(&self) -> Option<Chat> {
        utils::get_sender(self)
    }
}

pub trait GetText {
    fn get_text(&self) -> Option<&str>;
}

/// Get text from [Update]
impl GetText for Update {
    fn get_text(&self) -> Option<&str> {
        utils::get_text(self)
    }
}
