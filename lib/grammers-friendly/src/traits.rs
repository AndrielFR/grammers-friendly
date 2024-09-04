// Copyright (C) 2024 AndrielFR
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::pin::Pin;

use async_trait::async_trait;
use dyn_clone::DynClone;
use futures_util::Future;
use grammers_client::{Client, Update};

type PinBox =
    Pin<Box<dyn Future<Output = Result<(), Box<dyn std::error::Error>>> + Send + Sync + 'static>>;

pub trait AsyncFn: DynClone {
    fn call(&self, client: Client, update: Update) -> PinBox;
}

impl<T, F> AsyncFn for T
where
    T: Fn(Client, Update) -> F,
    T: DynClone,
    F: Future<Output = Result<(), Box<dyn std::error::Error>>> + Send + Sync + 'static,
{
    fn call(&self, client: Client, update: Update) -> PinBox {
        Box::pin(self(client, update))
    }
}

dyn_clone::clone_trait_object!(AsyncFn);

/// Middleware
#[async_trait]
pub trait MiddlewareImpl: DynClone {
    async fn call(&self, client: Client, update: Update) -> Result<(), Box<dyn std::error::Error>>;
}

dyn_clone::clone_trait_object!(MiddlewareImpl);

/// Filter
#[async_trait]
pub trait Filter {
    /// Needs to return bool
    /// `True` -> pass
    /// `False` -> not pass
    async fn is_ok(&self, client: &Client, update: &Update) -> bool;
}
