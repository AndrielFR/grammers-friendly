// Copyright (C) 2024 AndrielFR
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use grammers_client::{Client, Update};
use regex::Regex;

use crate::traits::Filter;

pub struct RegexFilter {
    regex: Regex,
}

impl RegexFilter {
    pub fn new(regex: &str) -> Self {
        Self {
            regex: Regex::new(regex).unwrap(),
        }
    }
}

impl Filter for RegexFilter {
    fn is_ok(&self, _client: &Client, update: &Update) -> bool {
        if let Update::NewMessage(message) = update {
            return self.regex.is_match(message.text());
        }

        false
    }
}

pub fn regex(regex: &str) -> RegexFilter {
    RegexFilter::new(regex)
}
