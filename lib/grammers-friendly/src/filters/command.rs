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

pub struct CommandFilter {
    prefixes: String,
    command: String,
}

impl CommandFilter {
    pub fn new(prefixes: &str, command: &str) -> Self {
        Self {
            prefixes: prefixes.to_string(),
            command: command.to_string(),
        }
    }
}

impl Filter for CommandFilter {
    fn is_ok(&self, _client: &Client, update: &Update) -> bool {
        if let Update::NewMessage(message) = update {
            let text = message.text();

            if self.prefixes.is_empty() {
                let command = text.split_whitespace().next().unwrap();
                return command == self.command;
            }

            let regex =
                Regex::new(format!("^[{}]{}", self.prefixes, self.command).as_str()).unwrap();
            return regex.is_match(text);
        }

        false
    }
}

pub fn command(prefixes: &str, command: &str) -> CommandFilter {
    CommandFilter::new(prefixes, command)
}
