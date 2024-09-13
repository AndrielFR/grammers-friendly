// Copyright (C) 2024 AndrielFR
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use grammers_client::{Client, Update};

use crate::{traits::MiddlewareImpl, Data};

/// A Middleware.
///
/// Can be before-type of after-type, which implies that it will be runned before
/// Or after the handlers.
#[derive(Clone)]
pub struct Middleware {
    pub mid: Box<dyn MiddlewareImpl>,
    pub mtype: MiddlewareType,
}

impl Middleware {
    /// Construct a new `Middleware`.
    ///
    /// Receives a struct which implements [`MiddlewareImpl`] and [`MiddlewareType`].
    ///
    /// [`MiddlewareImpl`]: crate::traits::MiddlewareImpl
    /// [`MiddlewareType`]: crate::MiddlewareType
    pub fn new<M: MiddlewareImpl>(mid: M, mtype: MiddlewareType) -> Self {
        Self {
            mid: Box::new(mid),
            mtype,
        }
    }

    /// Construct a new before-type `Middleware`.
    ///
    /// Receives a struct which implements [`MiddlewareImpl`].
    ///
    /// [`MiddlewareImpl`]: crate::traits::MiddlewareImpl
    pub fn before<M: MiddlewareImpl>(mid: M) -> Self {
        Self {
            mid: Box::new(mid),
            mtype: MiddlewareType::Before,
        }
    }

    /// Construct a new after-type `Middleware`.
    ///
    /// Receives a struct which implements [`MiddlewareImpl`].
    ///
    /// [`MiddlewareImpl`]: crate::traits::MiddlewareImpl
    pub fn after<M: MiddlewareImpl>(mid: M) -> Self {
        Self {
            mid: Box::new(mid),
            mtype: MiddlewareType::After,
        }
    }

    /// Run the middleware.
    pub(crate) async fn call(&mut self, client: &mut Client, update: &mut Update, data: &mut Data) {
        if let Err(e) = self.mid.call(client, update, data).await {
            log::error!("Error while running middleware: {:?}", e);
        }
    }

    /// Get the middleware type.
    pub fn mtype(&self) -> MiddlewareType {
        self.mtype.clone()
    }
}

/// Middleware Type.
///
/// In thesis, you don't need to use this,
/// Just use [`Middleware`] constructors: `::before(...)`, and/or `::after(...)`.
///
/// [`Middleware`]: crate::Middleware
#[derive(Clone, PartialEq)]
pub enum MiddlewareType {
    /// Runned before any `handler` in the same `router`.
    Before,

    /// Runned after all `handlers` in the same `router`.
    After,
}
