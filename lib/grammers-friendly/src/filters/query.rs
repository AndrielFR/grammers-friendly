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

use crate::traits::{Filter, GetQuery};

/// Ok if regex match
#[derive(Clone)]
pub struct QueryFilter {
    query: Regex,
}

impl QueryFilter {
    pub fn new(query: String) -> Self {
        let mut new_query = Vec::new();

        query
            .split_whitespace()
            .enumerate()
            .for_each(|(pos, word)| {
                if pos == 0 || word.contains('(') {
                    new_query.push(word.to_string());
                    return;
                }

                if word.contains(':') {
                    let mut maybe = false;

                    let (_var, mut ty) = word.trim().split_once(':').unwrap();
                    if ty.contains(':') {
                        let mut splitted = ty.split(':');
                        ty = splitted.next().unwrap();

                        splitted.for_each(|word| match word {
                            "my" | "may" | "maybe" => {
                                maybe = true;
                            }
                            _ => {}
                        });
                    }

                    let mut new_word = match ty {
                        "all" => r"[\w+|\d+|\d+\.\d+|true|false|yes|no|1|0]",
                        "int" => r"\d+",
                        "str" => r"\w+",
                        "sym" => r"\W+",
                        "bool" => "[true|false|yes|no|1|0]",
                        "float" => r"\d+\.\d+",
                        "double" => r"\d+\.\d+",
                        _ => "",
                    }
                    .to_string();
                    if !new_word.contains('[') {
                        new_word = format!("({})", new_word);
                    }

                    if word.contains('?') || maybe {
                        new_word.push('?');
                    }

                    new_query.push(new_word);
                } else {
                    new_query.push(word.to_string());
                }
            });

        let query = new_query.join(r"\s");
        println!("{}", query);
        Self {
            query: Regex::new(&query).unwrap(),
        }
    }
}

#[async_trait]
impl Filter for QueryFilter {
    async fn is_ok(&mut self, _client: &Client, update: &Update) -> bool {
        let query = update.get_query();

        if let Some(query) = query {
            let text = String::from_utf8(query.data().into()).unwrap();

            return self.query.is_match(&text);
        }

        false
    }
}

pub fn query(query: impl Into<String>) -> QueryFilter {
    QueryFilter::new(query.into())
}
