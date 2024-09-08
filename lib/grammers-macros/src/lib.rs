// Copyright (C) 2024 AndrielFR
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_export]
macro_rules! command {
    ($command:expr) => {
        ::grammers_friendly::filters::CommandFilter::new("/", $command)
    };
    ($prefixes:expr, $command:expr) => {
        extern crate downcast;
        ::grammers_friendly::filters::CommandFilter::new($prefixes, $command)
    };
}
