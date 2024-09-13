// Copyright (C) 2024 AndrielFR
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

mod data;
mod dispatcher;
mod filter;
pub mod filters;
mod handler;
mod middleware;
mod router;
pub mod traits;
pub mod utils;

pub use data::Data;
pub use dispatcher::Dispatcher;
pub use handler::{Handler, UpdateType};
pub use middleware::{Middleware, MiddlewareType};
pub use router::Router;

#[cfg(feature = "macros")]
pub use grammers_macros as macros;

pub mod prelude {
    pub use crate::traits::*;
    pub use crate::{
        filters, utils, Data, Dispatcher, Handler, Middleware, MiddlewareType::*, Router,
        UpdateType::*,
    };

    #[cfg(feature = "macros")]
    pub use crate::macros;
}
