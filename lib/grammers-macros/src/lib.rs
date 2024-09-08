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

#[macro_export]
macro_rules! get_module {
    ($var:ident, $data:ident, $type:ty) => {
        let __modules = $data.modules();
        let __index = {
            let mut num = None;

            for (i, module) in __modules.iter().enumerate() {
                let guard = module.lock().await;
                if guard.is::<$type>() {
                    num = Some(i);
                    break;
                }
            }

            num
        }
        .expect("module not found");
        let mut __guard = __modules
            .get(__index)
            .expect("module not found")
            .lock()
            .await;
        let $var = __guard
            .downcast_mut::<$type>()
            .expect("module is not of the expected type");
    };
}
