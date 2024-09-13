// Copyright (C) 2024 AndrielFR
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use async_trait::async_trait;
use grammers_client::{Client, Update};
use regex::Regex;

use crate::traits::{Filter, GetMessage};

/// Command filter.
///
/// Pass if `command` match.
#[derive(Clone)]
pub struct CommandFilter {
    is_bot: bool,

    prefixes: String,
    command: String,

    username: Option<String>,
}

impl CommandFilter {
    pub fn new(prefixes: impl Into<String>, command: impl Into<String>) -> Self {
        Self {
            is_bot: true,

            prefixes: prefixes.into(),
            command: command.into(),

            username: None,
        }
    }
}

#[async_trait]
impl Filter for CommandFilter {
    async fn is_ok(&mut self, client: &Client, update: &Update) -> bool {
        let message = update.get_message();

        let mut command = self.command.clone();

        if let Some(message) = message {
            let text = message.text();

            if self.prefixes.is_empty() {
                let input = text.split_whitespace().next().unwrap().to_string();
                return input == command;
            }

            if self.username.is_none() && self.is_bot {
                let me = client.get_me().await;

                if let Ok(me) = me {
                    if me.is_bot() {
                        self.username = me.username().map(String::from);
                    } else {
                        self.is_bot = false;
                    }
                }
            } else if let Some(username) = self.username.clone() {
                // Username is mandatory to bots
                command = command
                    .split_whitespace()
                    .enumerate()
                    .map(|(pos, word)| {
                        if pos == 0 {
                            format!(r#"{0}(@{1})?"#, word, username)
                        } else {
                            word.to_string()
                        }
                    })
                    .collect::<String>();
            }

            let regex =
                Regex::new(format!(r#"^[{0}]({1}$|{1}(\s))"#, self.prefixes, command).as_str())
                    .unwrap();
            return regex.is_match(text);
        }

        false
    }
}

/// Pass if `command` match.
pub fn command(prefixes: &str, command: &str) -> CommandFilter {
    CommandFilter::new(prefixes, command)
}
