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

#[derive(Clone)]
pub struct AndFilter {
    first: Box<dyn Filter>,
    second: Box<dyn Filter>,
}

impl AndFilter {
    pub fn new(first: impl Filter, second: impl Filter) -> Self {
        Self {
            first: Box::new(first),
            second: Box::new(second),
        }
    }
}

#[async_trait]
impl Filter for AndFilter {
    async fn is_ok(&mut self, client: &Client, update: &Update) -> bool {
        self.first.is_ok(client, update).await && self.second.is_ok(client, update).await
    }
}

pub fn and(first: impl Filter, second: impl Filter) -> AndFilter {
    AndFilter::new(first, second)
}
