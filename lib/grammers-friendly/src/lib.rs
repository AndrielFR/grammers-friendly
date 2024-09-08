// Copyright (C) 2024 AndrielFR
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![macro_use]
extern crate downcast_rs;

mod data;
mod dispatcher;
mod filter;
pub mod filters;
mod handler;
mod middleware;
pub mod traits;
pub mod utils;

pub use data::Data;
pub use dispatcher::Dispatcher;
pub use handler::{Handler, UpdateType};
pub use middleware::Middleware;

#[cfg(feature = "macros")]
pub use grammers_macros as macros;
