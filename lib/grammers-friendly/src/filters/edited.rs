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

pub struct EditedFilter;

impl EditedFilter {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Filter for EditedFilter {
    async fn is_ok(&self, _client: &Client, update: &Update) -> bool {
        matches!(update, Update::MessageEdited(_))
    }
}

pub fn edited() -> EditedFilter {
    EditedFilter::new()
}
