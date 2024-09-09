// Copyright (C) 2024 AndrielFR
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use grammers_client::{Client, Update};

use crate::{traits::MiddlewareImpl, Data};

#[derive(Clone)]
pub struct Middleware {
    pub mid: Box<dyn MiddlewareImpl>,
    pub mtype: MiddlewareType,
}

impl Middleware {
    pub fn new<M: MiddlewareImpl>(mid: M, mtype: MiddlewareType) -> Self {
        Self {
            mid: Box::new(mid),
            mtype,
        }
    }

    pub async fn call(
        &mut self,
        client: &mut Client,
        update: &mut Update,
        data: &mut Data,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let r = self.mid.call(client, update, data).await;
        if let Err(e) = r {
            log::error!("Error in middleware: {:?}", e);
        }

        Ok(())
    }

    pub fn mtype(&self) -> MiddlewareType {
        self.mtype.clone()
    }
}

#[derive(Clone, PartialEq)]
pub enum MiddlewareType {
    Before,
    After,
}
