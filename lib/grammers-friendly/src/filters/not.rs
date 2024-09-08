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

/// Ok if `filter` is not ok
#[derive(Clone)]
pub struct NotFilter {
    filter: Box<dyn Filter>,
}

impl NotFilter {
    pub fn new(filter: impl Filter) -> Self {
        Self {
            filter: Box::new(filter),
        }
    }
}

#[async_trait]
impl Filter for NotFilter {
    async fn is_ok(&mut self, client: &Client, update: &Update) -> bool {
        !self.filter.is_ok(client, update).await
    }
}

/// Ok if `filter` is not ok
pub fn not(filter: impl Filter) -> NotFilter {
    NotFilter::new(filter)
}
