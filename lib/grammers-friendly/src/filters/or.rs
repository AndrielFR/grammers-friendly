// Copyright (C) 2024 AndrielFR
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use async_trait::async_trait;
use grammers_client::{Client, Update};

use crate::traits::Filter;

/// Ok if `first` or `other` is ok
#[derive(Clone)]
pub struct OrFilter {
    first: Box<dyn Filter>,
    other: Box<dyn Filter>,
}

impl OrFilter {
    pub fn new(first: impl Filter, other: impl Filter) -> Self {
        Self {
            first: Box::new(first),
            other: Box::new(other),
        }
    }
}

#[async_trait]
impl Filter for OrFilter {
    async fn is_ok(&mut self, client: &Client, update: &Update) -> bool {
        self.first.is_ok(client, update).await || self.other.is_ok(client, update).await
    }
}

/// Ok if `first` or `other` is ok
pub fn or(first: impl Filter, other: impl Filter) -> OrFilter {
    OrFilter::new(first, other)
}
