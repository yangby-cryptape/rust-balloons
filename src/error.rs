// Copyright (C) 2019 Boyu Yang
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::result;

#[derive(Debug)]
pub enum Error {
    Custom(String),
}

impl Error {
    pub fn custom(msg: String) -> Self {
        Error::Custom(msg)
    }
}

pub type Result<T> = result::Result<T, Error>;
