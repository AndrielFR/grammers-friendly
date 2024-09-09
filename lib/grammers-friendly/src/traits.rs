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

/// The async `func` from handlers
pub trait AsyncFn: Send + Sync + 'static {
    fn call(&self, client: Client, update: Update, data: Data) -> PinBox;
}

impl<T, F> AsyncFn for T
where
    T: Fn(Client, Update, Data) -> F + Send + Sync + 'static,
    F: Future<Output = Result<(), Box<dyn std::error::Error>>> + Send + 'static,
{
    fn call(&self, client: Client, update: Update, data: Data) -> PinBox {
        Box::pin(self(client, update, data))
    }
}

/// Filter
#[async_trait]
pub trait Filter: CloneFilter + Send + Sync + 'static {
    /// Needs to return bool
    /// `True` -> pass
    /// `False` -> not pass
    async fn is_ok(&mut self, client: &Client, update: &Update) -> bool;

    /// Wrappes `self` and `second` into `AndFilter`
    fn and(self, second: impl Filter) -> AndFilter
    where
        Self: Sized,
    {
        AndFilter::new(self, second)
    }

    /// Wrappes `self` and `other` into `OrFilter`
    fn or(self, other: impl Filter) -> OrFilter
    where
        Self: Sized,
    {
        OrFilter::new(self, other)
    }

    /// Wrappes `self` into `NotFilter`
    fn not(self) -> NotFilter
    where
        Self: Sized,
    {
        NotFilter::new(self)
    }
}

pub trait CloneFilter {
    fn clone_filter(&self) -> Box<dyn Filter>;
}

impl<T> CloneFilter for T
where
    T: Filter + Clone,
{
    fn clone_filter(&self) -> Box<dyn Filter> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Filter> {
    fn clone(&self) -> Self {
        self.clone_filter()
    }
}

/// Middleware
#[async_trait]
pub trait MiddlewareImpl: CloneMiddlewareImpl + Send + Sync + 'static {
    async fn call(
        &mut self,
        client: &mut Client,
        update: &mut Update,
        data: &mut Data,
    ) -> Result<(), Box<dyn std::error::Error>>;
}

pub trait CloneMiddlewareImpl {
    fn clone_midddleware_impl(&self) -> Box<dyn MiddlewareImpl>;
}

impl<T> CloneMiddlewareImpl for T
where
    T: MiddlewareImpl + Clone,
{
    fn clone_midddleware_impl(&self) -> Box<dyn MiddlewareImpl> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn MiddlewareImpl> {
    fn clone(&self) -> Self {
        self.clone_midddleware_impl()
    }
}

/// Module
pub trait Module: DowncastSync + CloneModule {}

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
