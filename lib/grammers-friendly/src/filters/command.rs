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

pub struct CommandFilter {
    prefixes: String,
    command: String,
}

impl CommandFilter {
    pub fn new(prefixes: &str, command: &str) -> Self {
        Self {
            prefixes: prefixes.to_string(),
            command: command.trim().to_string(),
        }
    }
}

#[async_trait]
impl Filter for CommandFilter {
    async fn is_ok(&self, client: &Client, update: &Update) -> bool {
        let me = client.get_me().await;
        let message = update.get_message();

        let mut command = self.command.clone();

        if let Some(message) = message {
            let text = message.text();

            if self.prefixes.is_empty() {
                command = text.split_whitespace().next().unwrap().to_string();
                return command == self.command;
            }

            if let Ok(me) = me {
                if me.is_bot() {
                    // Username is mandatory to bots
                    let username = me.username().unwrap();
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
            }

            let regex =
                Regex::new(format!(r#"^[{0}]({1}$|{1}(\s))"#, self.prefixes, command).as_str())
                    .unwrap();
            return regex.is_match(text);
        }

        false
    }
}

pub fn command(prefixes: &str, command: &str) -> CommandFilter {
    CommandFilter::new(prefixes, command)
}
