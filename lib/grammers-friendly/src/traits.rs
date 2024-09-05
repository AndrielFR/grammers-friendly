// Copyright (C) 2024 AndrielFR
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::{pin::Pin, sync::Arc};

use async_trait::async_trait;
use downcast_rs::{impl_downcast, DowncastSync};
use dyn_clone::DynClone;
use futures_util::Future;
use grammers_client::{Client, Update};
use tokio::sync::RwLock;

use crate::filters::{AndFilter, NotFilter, OrFilter};

type PinBox =
    Pin<Box<dyn Future<Output = Result<(), Box<dyn std::error::Error>>> + Send + Sync + 'static>>;

pub trait AsyncFn: DynClone {
    fn call(
        &self,
        client: Client,
        update: Update,
        modules: Vec<Arc<RwLock<dyn Module + 'static>>>,
    ) -> PinBox;
}

impl<T, F> AsyncFn for T
where
    T: Fn(Client, Update, Vec<Arc<RwLock<dyn Module + 'static>>>) -> F,
    T: DynClone,
    F: Future<Output = Result<(), Box<dyn std::error::Error>>> + Send + Sync + 'static,
{
    fn call(
        &self,
        client: Client,
        update: Update,
        modules: Vec<Arc<RwLock<dyn Module>>>,
    ) -> PinBox {
        Box::pin(self(client, update, modules))
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
pub trait Module: DowncastSync {
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
}

impl_downcast!(sync Module);
