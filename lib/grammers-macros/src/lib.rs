// Copyright (C) 2024 AndrielFR
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/// Command filter macro.
///
/// Used with pré-setted prefixes `/` and `!`.
///
/// # Example
///
/// ```
/// macros::command!("start")
/// ```
///
/// Which is equivalent to
///
/// ```
/// filters::command("/!", "start")
/// ```
#[macro_export]
macro_rules! command {
    ($command:expr) => {
        ::grammers_friendly::filters::command("/!", $command)
    };
    ($prefixes:expr, $command:expr) => {
        ::grammers_friendly::filters::command($prefixes, $command)
    };
}
